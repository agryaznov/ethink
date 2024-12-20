#![cfg(feature = "runtime-benchmarks")]

use ep_eth::{
    EthTransaction, LegacyTransaction, LegacyTransactionMessage, Receipt,
    TransactionAction, TransactionSignature
};
use frame_benchmarking::v2::*;

use hex_literal::hex;
use crate::{U256, Config, Call, RawOrigin, Pallet, Dispatchable, DispatchInfo, PostDispatchInfo};

#[benchmarks(
    where
     T: Config,
     T::AccountId: From<sp_core::H160> + AsRef<[u8]> + Into<sp_core::H160>,
     T::RuntimeCall: Dispatchable<Info = DispatchInfo, PostInfo = PostDispatchInfo>
)]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn transfer() {
        let tx = LegacyTransaction {
            nonce: U256::MAX,
            gas_price: U256::MAX,
            gas_limit: U256::MAX,
            action: TransactionAction::Create,
            value: U256::MAX,
            // TODO `i`: Size of the input in bytes., see instantiate() or seal_input()
            input: Default::default(),
            signature: TransactionSignature::new(
                38,
                hex!("be67e0a07db67da8d446f76add590e54b6e92cb6b8f9835aeb67540579a27717").into(),
                hex!("2d690516512020171c1ec870f6ff45398cc8609250326be89915fb538e7bd718").into(),
            )
            .expect("cant' create tx signature"),
        };

        // #[extrinsic_call]
        // _(RawOrigin::Signed(whitelisted_caller()), tx);

        #[block]
        {
            let _ = 1;
        }
    }
}
