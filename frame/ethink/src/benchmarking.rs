use crate::{
    BalanceOf, Call, Config, DispatchInfo, Dispatchable, EthTransaction, OriginFor, Pallet,
    PostDispatchInfo, RawOrigin, Weight, U256,
};
use ep_eth::{
    AccountId20, EthereumSignature, EthereumSigner, LegacyTransaction, LegacyTransactionMessage,
    Receipt, TransactionAction, TransactionSignature, H256,
};
use frame_benchmarking::v2::*;
use hex_literal::hex;
use pallet_contracts::{Code, CollectEvents, DebugInfo};
use sp_std::vec;

// TODO take from mock
const ALITH: [u8; 20] = [
    242, 79, 243, 169, 207, 4, 199, 29, 188, 148, 208, 181, 102, 247, 162, 123, 148, 86, 108, 172,
];
const ALITH_KEY: [u8; 32] = [
    95, 185, 45, 110, 152, 136, 79, 118, 222, 70, 143, 163, 246, 39, 143, 136, 7, 196, 139, 235,
    193, 53, 149, 212, 90, 245, 189, 196, 218, 112, 33, 51,
];

// // This is a simple Wasm contract which when called terminates itself,
// // sending all its balance to Baltathar
// const CONTRACT_CODE: &str = r#"
//     (module
//         (import "seal0" "seal_terminate" (func $seal_terminate (param i32 i32)))
//         (import "env" "memory" (memory 1 1))

//         ;; beneficiary address of Baltathar
//         (data (i32.const 0)
//             "\3c\d0\a7\05\a2\dc\65\e5\b1\e1"
//             "\20\58\96\ba\a2\be\8a\07\c6\e0"
//         )

//         (func (export "deploy"))
//         (func (export "call")
//             (call $seal_terminate
//                 (i32.const 0)	;; Pointer to beneficiary address
//                 (i32.const 20)	;; Length of beneficiary address
//             )
//             (unreachable) ;; seal_terminate never returns
//         )
//     )
//     "#;

// fn deploy_contract<T: Config>() {
//     //-> T::AccountTd {
    //     T::Contracts::bare_instantiate(
    //         ALITH.into(),
    //         0,
    //         Weight::MAX,
    //         None,
    //         Code::Upload(wasm),
    //         vec![],
    //         vec![],
    //         DebugInfo::Skip,
    //         CollectEvents::Skip,
    //     )
    //     .result
    //     .expect("Failed to instantiate contract")
    //     .account_id
//     let _value = T::Currency::minimum_balance();
// }

#[benchmarks(
    where
     T: Config,
     T::AccountId: From<sp_core::H160> + AsRef<[u8]> + Into<sp_core::H160>,
     T::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>,
     OriginFor<T>: Into<Result<RawOrigin, OriginFor<T>>>,
     BalanceOf<T>: TryFrom<sp_core::U256>,
     T::RuntimeOrigin: From<RawOrigin>,
)]
mod benchmarks {
    use super::*;

    const CONTRACT_ADDR: [u8;20] = [
        188, 109, 36, 50, 142, 195, 197, 246, 227, 227, 19, 127, 255, 152, 203, 232, 206, 130, 7, 161
    ];
    #[benchmark]
    fn transact() -> Result<(), BenchmarkError> {
        let alith_key = libsecp256k1::SecretKey::parse(&ALITH_KEY).expect("cant parse signer key");
//        Compose transaction
        let msg = LegacyTransactionMessage {
            action: TransactionAction::Call(CONTRACT_ADDR.into()),
            input: vec![],
            nonce: 0u8.into(),
            gas_price: 0u8.into(),
            gas_limit: U256::from(u64::MAX),
            value: 0u8.into(),
            chain_id: None,
        };
        let signing_msg = libsecp256k1::Message::parse(msg.hash().as_fixed_bytes());
        let sig = libsecp256k1::sign(&signing_msg, &alith_key).0;

        let signature = TransactionSignature::new(27u64, H256(sig.r.b32()), H256(sig.s.b32()))
            .expect("cant convert signature");
        // TODO: hack storage to contain contract,
        // OR HACK: just emulate storage like it's contains contract on that address
        let tx = EthTransaction::Legacy(LegacyTransaction {
            nonce: msg.nonce,
            gas_price: msg.gas_price,
            gas_limit: msg.gas_limit,
            action: msg.action,
            value: msg.value,
            input: msg.input,
            signature,
        });

        #[extrinsic_call]
        _(RawOrigin::EthTransaction(ALITH.into()), tx);

        Ok(())
    }

       impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
