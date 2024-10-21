use crate::{mock::*, System};

use crate::{self as pallet_ethink, Pallet, Weight};
use ep_eth::{compose_and_sign_tx, AccountId20, EthTxInput, TransactionAction};
use frame_support::assert_ok;
use pallet_contracts::{Code, CollectEvents, DebugInfo};
use sp_runtime::BuildStorage;

mod test_utils {
    use crate::{mock::Test, tests::AccountId20, Config};
    use frame_support::traits::fungible::Mutate;

    pub fn set_balance(who: &AccountId20, amount: u128) {
        let _ = <Test as Config>::Currency::set_balance(who, amount);
    }
    pub fn get_balance(who: &AccountId20) -> u128 {
        <Test as Config>::Currency::free_balance(who)
    }
}

#[derive(Default)]
pub struct ExtBuilder;

pub const GAS_LIMIT: Weight = Weight::from_parts(100_000_000_000, 3 * 1024 * 1024);

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

// This is a simple Wasm contract which when called terminates itself,
// sending all its balance to Baltathar
const CONTRACT_CODE: &str = r#"
(module
	(import "seal0" "seal_terminate" (func $seal_terminate (param i32 i32)))
	(import "env" "memory" (memory 1 1))

    ;; beneficiary address of Baltathar
	(data (i32.const 0)
        "\3c\d0\a7\05\a2\dc\65\e5\b1\e1"
        "\20\58\96\ba\a2\be\8a\07\c6\e0"
	)

	(func (export "deploy"))
	(func (export "call")
    	(call $seal_terminate
			(i32.const 0)	;; Pointer to beneficiary address
			(i32.const 20)	;; Length of beneficiary address
		)
		(unreachable) ;; seal_terminate never returns
    )
)
"#;

#[test]
fn calling_contract_account_executes_it() {
    let wasm = wat::parse_str(CONTRACT_CODE).unwrap();

    ExtBuilder::default().build().execute_with(|| {
        let _ = test_utils::set_balance(&ALITH, 10_000_000_000);
        // Instantiate contract and deposit balance (ED) to it
        let contract_addr = Contracts::bare_instantiate(
            ALITH,
            0,
            GAS_LIMIT,
            None,
            Code::Upload(wasm),
            vec![],
            vec![],
            DebugInfo::Skip,
            CollectEvents::Skip,
        )
        .result
        .expect("Failed to instantiate contract")
        .account_id;

        println!("CONTRACT ADDR: {:?}", &contract_addr);

        // Compose transaction
        let input = EthTxInput {
            action: TransactionAction::Call(contract_addr.into()),
            data: vec![].into(),
            ..Default::default()
        };
        let eth_tx = compose_and_sign_tx(input);
        println!("Signed Eth Tx: {:#?}", &eth_tx);

        let origin = RuntimeOrigin::from(pallet_ethink::RawOrigin::EthTransaction(ALITH.into()));
        // Ensure Baltathar has no balance before the call
        assert_eq!(test_utils::get_balance(&BALTATHAR), 0);
        // Call contract
        assert_ok!(Ethink::transact(origin, eth_tx));
        // As the result of the call,
        // our contract should terminate and send its balance to Baltathar
        assert!(Contracts::code_hash(&contract_addr.into()).is_none());
        // The only balance the contract had was existentional deposit,
        // which is now trasferred to Baltathar
        assert_eq!(test_utils::get_balance(&BALTATHAR), ED);
        // Check that sender account nonce incremented
        let nonce: u64 = System::<Test>::account_nonce(ALITH).into();
        assert_eq!(nonce, 1);
    });
}
