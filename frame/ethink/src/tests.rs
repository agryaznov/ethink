use crate::{mock::*, System};

use crate::{self as pallet_ethink, Pallet};
use ep_crypto::{AccountId20, EthereumSignature};
use ep_mapping::{SubstrateWeight, Weight};
use frame_support::{assert_err, assert_ok};
use sp_core::{ecdsa, Pair, U256};
use sp_runtime::BuildStorage;

mod test_utils {
    use crate::{mock::Test, tests::AccountId20, Config};
    use frame_support::traits::fungible::Mutate;

    pub fn set_balance(who: &AccountId20, amount: u64) {
        let _ = <Test as Config>::Currency::set_balance(who, amount);
    }
    pub fn get_balance(who: &AccountId20) -> u64 {
        <Test as Config>::Currency::free_balance(who)
    }
}

#[derive(Default)]
pub struct ExtBuilder;

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut s = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();
        pallet_balances::GenesisConfig::<Test> { balances: vec![] }
            .assimilate_storage(&mut s)
            .unwrap();
        let mut ext = sp_io::TestExternalities::new(s);
        ext.execute_with(|| {
            use frame_support::traits::OnGenesis;

            Pallet::<Test>::on_genesis();

            System::<Test>::set_block_number(1);
        });

        ext
    }
}

// TODO put these to a separate crate to DRY with e2e tests
#[derive(Clone)]
pub struct ContractInput(Vec<u8>);

impl From<Vec<u8>> for ContractInput {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl Into<Vec<u8>> for ContractInput {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

#[derive(Clone)]
/// Ethereum transaction input, used for transaciton building in tests
pub struct EthTxInput {
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: SubstrateWeight,
    pub action: ethereum::TransactionAction,
    pub value: u64,
    pub data: ContractInput,
    pub chain_id: Option<u64>,
    pub signer: ecdsa::Pair,
}

impl Default for EthTxInput {
    fn default() -> Self {
        Self {
            nonce: 1u64,
            gas_price: 0u64,
            gas_limit: SubstrateWeight::from(Weight::MAX),
            action: ethereum::TransactionAction::Call(Default::default()),
            value: 0u64,
            data: vec![0].into(),
            chain_id: None,
            signer: ecdsa::Pair::generate().0,
        }
    }
}
use crate::{EthTransaction, LegacyTransactionMessage};
use ethereum::LegacyTransaction;

impl From<EthTxInput> for LegacyTransactionMessage {
    fn from(v: EthTxInput) -> Self {
        let nonce = v.nonce.into();
        let gas_price = v.gas_price.into();
        let gas_limit: U256 = v.gas_limit.into();
        let value = v.value.into();

        Self {
            nonce,
            gas_price,
            gas_limit,
            action: v.action,
            value,
            input: v.data.into(),
            chain_id: v.chain_id,
        }
    }
}

/// Build Eth tx message, sign it and build an Eth transaction
pub fn compose_and_sign_tx(i: EthTxInput) -> EthTransaction {
    let msg: LegacyTransactionMessage = i.clone().into();
    let sig = EthereumSignature::new(i.signer.sign_prehashed(&msg.hash().into()));
    let sig: Option<ethereum::TransactionSignature> = sig.into();
    let signature = sig.expect("signer generated no signature");

    EthTransaction::Legacy(LegacyTransaction {
        nonce: msg.nonce,
        gas_price: msg.gas_price,
        gas_limit: msg.gas_limit,
        action: msg.action,
        value: msg.value,
        input: msg.input,
        signature,
    })
}

#[test]
fn calling_user_account_transfers_balance() {
    ExtBuilder::default().build().execute_with(|| {
        let init_balance = 100_000_000;
        let transfer_balance = 20_000_000;

        let _ = test_utils::set_balance(&ALITH, init_balance);

        let input = EthTxInput {
            action: ethereum::TransactionAction::Call(BALTATHAR.into()),
            data: vec![].into(),
            value: transfer_balance,
            ..Default::default()
        };
        let eth_tx = compose_and_sign_tx(input);

        let origin = RuntimeOrigin::from(pallet_ethink::RawOrigin::EthTransaction(ALITH.into()));
        assert_ok!(Ethink::transact(origin, eth_tx));

        let alith_balance = test_utils::get_balance(&ALITH);
        let baltathar_balance = test_utils::get_balance(&BALTATHAR);

        assert_eq!(alith_balance, init_balance - transfer_balance);
        assert_eq!(baltathar_balance, transfer_balance);
    });
}

#[test]
fn transaction_increments_nonce() {
    ExtBuilder::default().build().execute_with(|| {
        let _ = test_utils::set_balance(&ALITH, 10_000_000);

        let input = EthTxInput {
            action: ethereum::TransactionAction::Call(BALTATHAR.into()),
            data: vec![].into(),
            ..Default::default()
        };
        let eth_tx = compose_and_sign_tx(input);

        let origin = RuntimeOrigin::from(pallet_ethink::RawOrigin::EthTransaction(ALITH.into()));
        assert_ok!(Ethink::transact(origin, eth_tx));

        let nonce: u64 = System::<Test>::account_nonce(ALITH).into();

        assert_eq!(nonce, 1);
    });
}

// TODO calling_contract_works()
