// SPDX-License-Identifier: Apache-2.0
// This file is part of Polkamask.
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Polkamask pallet
//!
//! The Polkamask pallet works as a proxy in front of `pallet-contracts` for the transactions
//! coming from the Ethereum RPC.

// `no_std` when compiling to WebAssembly
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::comparison_chain, clippy::large_enum_variant)]

#[cfg(all(feature = "std", test))]
mod mock;
#[cfg(all(feature = "std", test))]
mod tests;

use frame_support::{
    dispatch::{DispatchInfo, DispatchResultWithPostInfo, PostDispatchInfo},
    traits::{
        fungible::{Inspect, Mutate, MutateHold},
        tokens::Preservation,
        EnsureOrigin, Get, PalletInfoAccess, Time,
    },
};
use frame_system::{pallet_prelude::OriginFor, CheckWeight, Pallet as System, WeightInfo};
use scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{H160, H256, U256};
use sp_runtime::{
    traits::{DispatchInfoOf, Dispatchable, One, Saturating, UniqueSaturatedInto, Zero},
    transaction_validity::{
        InvalidTransaction, TransactionValidity, TransactionValidityError, ValidTransactionBuilder,
    },
    RuntimeDebug, SaturatedConversion,
};
use sp_std::{marker::PhantomData, prelude::*};

pub use ethereum::{
    AccessListItem, BlockV2 as Block, LegacyTransactionMessage, Log, ReceiptV3 as Receipt,
    TransactionAction, TransactionV2 as Transaction,
};

pub type BalanceOf<T> =
    <<T as Config>::Currency as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum RawOrigin {
    EthereumTransaction(H160),
}

pub fn ensure_ethereum_transaction<OuterOrigin>(o: OuterOrigin) -> Result<H160, &'static str>
where
    OuterOrigin: Into<Result<RawOrigin, OuterOrigin>>,
{
    // TODO check this really verifyes the signature
    match o.into() {
        Ok(RawOrigin::EthereumTransaction(n)) => Ok(n),
        _ => Err("Bad origin: not a valid Ethereum transaction"),
    }
}

pub struct EnsureEthereumTransaction;
impl<O: Into<Result<RawOrigin, O>> + From<RawOrigin>> EnsureOrigin<O>
    for EnsureEthereumTransaction
{
    type Success = H160;
    fn try_origin(o: O) -> Result<Self::Success, O> {
        o.into().map(|o| match o {
            RawOrigin::EthereumTransaction(id) => id,
        })
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<O, ()> {
        Ok(O::from(RawOrigin::EthereumTransaction(Default::default())))
    }
}

// TODO refactor (this is taken from pallet_ethereum as is for now)
impl<T> Call<T>
where
    OriginFor<T>: Into<Result<RawOrigin, OriginFor<T>>>,
    T: Send + Sync + Config,
    T::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
    T::AccountId: From<sp_core::H160>,
    BalanceOf<T>: TryFrom<sp_core::U256>,
{
    pub fn is_self_contained(&self) -> bool {
        matches!(self, Call::transact { .. })
    }

    pub fn check_self_contained(&self) -> Option<Result<H160, TransactionValidityError>> {
        if let Call::transact { tx } = self {
            let check = || {
                // TODO change error to proper one (add to primitives as it was in pallet_ethereum?)
                let origin = Pallet::<T>::extract_tx_fields(tx)
                    .0
                    .ok_or(InvalidTransaction::Custom(42u8))?;

                Ok(origin)
            };

            Some(check())
        } else {
            None
        }
    }

    pub fn pre_dispatch_self_contained(
        &self,
        origin: &H160,
        dispatch_info: &DispatchInfoOf<T::RuntimeCall>,
        len: usize,
    ) -> Option<Result<(), TransactionValidityError>> {
        // TODO: do we need this fn at all??
        Some(Ok(()))
    }

    pub fn validate_self_contained(
        &self,
        origin: &H160,
        dispatch_info: &DispatchInfoOf<T::RuntimeCall>,
        len: usize,
    ) -> Option<TransactionValidity> {
        if let Call::transact { tx } = self {
            if let Err(e) = CheckWeight::<T>::do_validate(dispatch_info, len) {
                return Some(Err(e));
            }
            // TODO refactor
            let tx_nonce = match tx {
                Transaction::Legacy(t) => t.nonce,
                Transaction::EIP2930(t) => t.nonce,
                Transaction::EIP1559(t) => t.nonce,
            };
            // TODO: add some proper validation here
            let mut builder = ValidTransactionBuilder::default().and_provides((origin, tx_nonce));

            Some(builder.build())
        } else {
            None
        }
    }
}

/// Provider of the contracts functionality
/// This is pallet_contracts in our case
pub trait Executor<RuntimeCall> {
    /// Check if AccountId is owned by a contract
    fn is_contract(who: H160) -> bool;
    /// Construct proper runtime call for the input provided
    fn construct_call(to: H160, value: U256, data: Vec<u8>) -> RuntimeCall;
}

pub use self::pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::origin]
    pub type Origin = RawOrigin;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// The overarching call type.
        type Call: Dispatchable<RuntimeOrigin = Self::RuntimeOrigin, PostInfo = PostDispatchInfo>;
        /// The fungible in which fees are paid and contract balances are held.
        //TODO mb we fdont need all of these
        type Currency: Inspect<Self::AccountId> + Mutate<Self::AccountId>;
        /// Contracts engine
        type Contracts: Executor<<Self as Config>::Call>;
    }

    // #[pallet::hooks]
    // impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    // 	// TODO probably we'd need to make something in this hook in order to genereate tx_receipt:
    // 	// fn on_finalize(n: T::BlockNumber) {
    // }

    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        OriginFor<T>: Into<Result<RawOrigin, OriginFor<T>>>,
        T::AccountId: From<sp_core::H160>,
        T::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
        BalanceOf<T>: TryFrom<sp_core::U256>,
    {
        /// Transact an Ethereum transaction.
        #[pallet::call_index(0)]
        // TODO weight
        #[pallet::weight(42)]
        pub fn transact(origin: OriginFor<T>, tx: Transaction) -> DispatchResult {
            // TODO
            //let source = ensure_ethereum_transaction(origin)?;

            // We received Ethereum transaction,
            // need to route it either as a contract call or jsut a balance transfer
            // determinant for this is pallet_contracts' ContractInfo storage:
            // if it has the destination AccountId among its keys,
            // then it's a contract call. For now we going to do this via
            // pallet_contracts::code_hash()
            // This could possibly be optimized later with another method which uses
            // StorageMap::contains_key() instead of StorageMap::get() under the hood.

            log::error!(target: "polkamask:pallet", "Received Eth Tx: {:?}", &tx);

            let (from, to, value, data) = Self::extract_tx_fields(&tx);

            log::error!(target: "polkamask:pallet", "From {:?}", &from);
            log::error!(target: "polkamask:pallet", "To {:?}", &to);
            log::error!(target: "polkamask:pallet", "Value {:?}", &value);

            let from: T::AccountId = from.ok_or(Error::<T>::TxConvertionFailed)?.into();
            let to = to.ok_or(Error::<T>::TxConvertionFailed)?;

            // TODO probably with dispatchables we don't need this anymore?
            System::<T>::inc_account_nonce(from);

            let call = T::Contracts::construct_call(to, value, data);
            log::error!(target: "polkamask:pallet", "Dispatching Call....");
            let _ = call.dispatch(origin).map_err(|e| {
                log::error!(target: "polkamask:pallet", "Failed: {:?}", &e);
                Error::<T>::TxExecutionFailed
            })?;

            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event {
        /// An Ethereum transaction was successfully executed.
        Executed {
            from: H160,
            to: H160,
            transaction_hash: H256,
            //            exit_reason: ExitReason,
            extra_data: Vec<u8>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Signature is invalid.
        InvalidSignature,
        // TODO
        TxConvertionFailed,
        // TODO
        TxExecutionFailed,
    }

    /// The current Ethereum receipts.
    #[pallet::storage]
    pub type CurrentReceipts<T: Config> = StorageValue<_, Vec<Receipt>>;

    // #[pallet::genesis_config]
    // #[derive(Default)]
    // pub struct GenesisConfig {}

    // #[pallet::genesis_build]
    // impl<T: Config> GenesisBuild<T> for GenesisConfig {
    // 	fn build(&self) {
    // 		<Pallet<T>>::store_block(None, U256::zero());
    // 		frame_support::storage::unhashed::put::<EthereumStorageSchema>(
    // 			PALLET_ETHEREUM_SCHEMA,
    // 			&EthereumStorageSchema::V3,
    // 		);
    // 	}
    // }
}

impl<T: Config> Pallet<T> {
    fn extract_tx_fields(tx: &Transaction) -> (Option<H160>, Option<H160>, U256, Vec<u8>) {
        let mut sig = [0u8; 65];
        let mut msg = [0u8; 32];
        let (to, value, data) = match tx {
            Transaction::Legacy(t) => {
                sig[0..32].copy_from_slice(&t.signature.r()[..]);
                sig[32..64].copy_from_slice(&t.signature.s()[..]);
                sig[64] = t.signature.standard_v();
                msg.copy_from_slice(
                    &ethereum::LegacyTransactionMessage::from(t.clone()).hash()[..],
                );
                (
                    match t.action {
                        TransactionAction::Call(h) => Some(h),
                        // TODO
                        TransactionAction::Create => None,
                    },
                    t.value,
                    t.input.clone(),
                )
            }
            Transaction::EIP2930(t) => {
                sig[0..32].copy_from_slice(&t.r[..]);
                sig[32..64].copy_from_slice(&t.s[..]);
                sig[64] = t.odd_y_parity as u8;
                msg.copy_from_slice(
                    &ethereum::EIP2930TransactionMessage::from(t.clone()).hash()[..],
                );
                (
                    match t.action {
                        TransactionAction::Call(h) => Some(h),
                        // TODO
                        TransactionAction::Create => None,
                    },
                    t.value,
                    t.input.clone(),
                )
            }
            Transaction::EIP1559(t) => {
                sig[0..32].copy_from_slice(&t.r[..]);
                sig[32..64].copy_from_slice(&t.s[..]);
                sig[64] = t.odd_y_parity as u8;
                msg.copy_from_slice(
                    &ethereum::EIP1559TransactionMessage::from(t.clone()).hash()[..],
                );
                (
                    match t.action {
                        TransactionAction::Call(h) => Some(h),
                        // TODO
                        TransactionAction::Create => None,
                    },
                    t.value,
                    t.input.clone(),
                )
            }
        };

        let from = sp_io::crypto::secp256k1_ecdsa_recover(&sig, &msg)
            .ok()
            .map(|p| H160::from(H256::from(sp_io::hashing::keccak_256(&p))));

        (from, to, value, data)
    }
}

#[derive(Eq, PartialEq, Clone, RuntimeDebug)]
pub enum ReturnValue {
    Bytes(Vec<u8>),
    Hash(H160),
}
