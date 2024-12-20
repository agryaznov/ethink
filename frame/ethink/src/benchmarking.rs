use crate::{
    BalanceOf, Call, Config, DispatchInfo, Dispatchable, EthTransaction, OriginFor, Pallet,
    PostDispatchInfo, RawOrigin, U256,
};
use ep_eth::{
    AccountId20, EthereumSignature, EthereumSigner, LegacyTransaction, LegacyTransactionMessage,
    Receipt, TransactionAction, TransactionSignature, H256
};
use frame_benchmarking::v2::*;
use hex_literal::hex;
use sp_std::vec;

const ALITH: [u8; 20] = [
    242, 79, 243, 169, 207, 4, 199, 29, 188, 148, 208, 181, 102, 247, 162, 123, 148, 86, 108, 172,
];
const ALITH_KEY: [u8; 32] = [
    95, 185, 45, 110, 152, 136, 79, 118, 222, 70, 143, 163, 246, 39, 143, 136, 7, 196, 139, 235, 193, 53, 149, 212, 90, 245, 189, 196, 218, 112, 33, 51
];

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

    #[benchmark]
    fn transact() -> Result<(), BenchmarkError> {
        // dumb contract code with size of
        // MaxCodeLen = ConstU32<{ 256 * 1024 }>
        // (as configured in runtime)
        let code = vec![42u8; 256 * 1024];
        // Compose transaction
        let alith_key = libsecp256k1::SecretKey::parse(&ALITH_KEY).expect("cant parse signer key");
        let msg = LegacyTransactionMessage {
            action: TransactionAction::Create,
            input: code.into(),
            nonce: 0u8.into(),
            gas_price: 0u8.into(),
            gas_limit: U256::MAX,
            value: 0u8.into(),
            chain_id: None,
        };
        let signing_msg = libsecp256k1::Message::parse(msg.hash().as_fixed_bytes());
        let sig = libsecp256k1::sign(&signing_msg, &alith_key).0;

        let signature =  TransactionSignature::new(27u64, H256(sig.r.b32()),  H256(sig.r.b32())).expect("cant convert signature");

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
}
