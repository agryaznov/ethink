// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]
#![deny(unused_crate_dependencies)]

use ethereum::Log;
pub use ethereum::TransactionV2 as EthTx;
use ethereum_types::Bloom;
use scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

pub use evm::{backend::Basic as EthAccount, ExitReason};
// Substrate
use sp_core::{Hasher, H160, H256, U256};
use sp_runtime::{traits::Block as BlockT, RuntimeDebug};
use sp_state_machine::OverlayedChanges;
use sp_std::vec::Vec;

#[derive(Clone, Eq, PartialEq, Default, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct TransactionStatus {
    pub transaction_hash: H256,
    pub transaction_index: u32,
    pub from: H160,
    pub to: Option<H160>,
    pub contract_address: Option<H160>,
    pub logs: Vec<Log>,
    pub logs_bloom: Bloom,
}

#[derive(Eq, PartialEq, Clone, Encode, Decode, sp_runtime::RuntimeDebug)]
pub struct TxPoolResponse {
    pub ready: Vec<ethereum::TransactionV2>,
    pub future: Vec<ethereum::TransactionV2>,
}

pub trait RuntimeStorageOverride<B: BlockT, C, H: Hasher>: Send + Sync {
    fn is_enabled() -> bool;

    fn set_overlayed_changes(
        client: &C,
        overlayed_changes: &mut OverlayedChanges<H>,
        block: B::Hash,
        version: u32,
        address: H160,
        balance: Option<U256>,
        nonce: Option<U256>,
    );

    fn into_account_id_bytes(address: H160) -> Vec<u8>;
}

impl<B: BlockT, C, H: Hasher> RuntimeStorageOverride<B, C, H> for () {
    fn is_enabled() -> bool {
        false
    }

    fn set_overlayed_changes(
        _client: &C,
        _overlayed_changes: &mut OverlayedChanges<H>,
        _block: B::Hash,
        _version: u32,
        _address: H160,
        _balance: Option<U256>,
        _nonce: Option<U256>,
    ) {
    }

    fn into_account_id_bytes(_address: H160) -> Vec<u8> {
        Vec::default()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WeightInfo {
    pub ref_time_limit: Option<u64>,
    pub proof_size_limit: Option<u64>,
    pub ref_time_usage: Option<u64>,
    pub proof_size_usage: Option<u64>,
}

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UsedGas {
    /// The used_gas as returned by the evm gasometer on exit.
    pub standard: U256,
    /// The result of applying a gas ratio to the most used
    /// external metric during the evm execution.
    pub effective: U256,
}

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EVMExecInfo<T> {
    pub exit_reason: ExitReason,
    pub value: T,
    pub used_gas: UsedGas,
    pub weight_info: Option<WeightInfo>,
    pub logs: Vec<Log>,
}

sp_api::decl_runtime_apis! {
    /// Runtime-exposed API necessary for ETH-compatibility layer.
    pub trait ETHRuntimeRPC {
        /// Returns runtime defined pallet_evm::ChainId.
        fn chain_id() -> u64;
        /// Returns account balance.
        fn account_free_balance(address: H160) -> U256;
        /// Returns account nonce.
        fn nonce(address: H160) -> U256;
        /// Call
        fn call(
             from: H160,
            to: H160,
            data: Vec<u8>,
            value: U256,
            gas_limit: U256,
        ) -> Result<Vec<u8>, sp_runtime::DispatchError>;

        fn convert_transaction(transaction: ethereum::TransactionV2) -> <Block as BlockT>::Extrinsic;

        fn print_xt(from: H160, to: H160, value: U256) -> Result<(), sp_runtime::DispatchError>;
    }
}

/// Fallback transaction converter when the `ConvertTransactionRuntimeApi` is not available. For almost all
/// non-legacy cases, you can instantiate this type as `NoTransactionConverter`.
pub trait ConvertTransaction<E> {
    fn convert_transaction(&self, transaction: ethereum::TransactionV2) -> E;
}

/// No fallback transaction converter is available.
/// `NoTransactionConverter` is a non-instantiable type (an enum with no variants),
/// so we are guaranteed at compile time that `NoTransactionConverter` can never be instantiated.
pub enum NoTransactionConverter {}
impl<E> ConvertTransaction<E> for NoTransactionConverter {
    // `convert_transaction` is a method taking `&self` as a parameter, so it can only be called via an instance of type Self,
    // so we are guaranteed at compile time that this method can never be called.
    fn convert_transaction(&self, _transaction: ethereum::TransactionV2) -> E {
        match *self {}
    }
}
