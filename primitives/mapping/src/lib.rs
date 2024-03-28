use ethereum_types::{H256, U256};
use ethink_rpc_core::types::{BlockTransactions as EthBlockTxs, Header as EthHeader};
use sp_runtime::traits::{Block as BlockT, Header, UniqueSaturatedInto};
// TODO move these types to primitives
pub use ethink_rpc_core::types::Block as EthBlock;
// TODO move all types to primitives /types
/// Substrate block, convertible to Ethereum block
pub struct SubBlock<B>(pub B);

impl<B: BlockT<Hash = H256>> From<SubBlock<B>> for EthBlock {
    // Generate dumb ETH block with empty tx list,
    // from the given substrate block
    fn from(b: SubBlock<B>) -> Self {
        let h = b.0.header();

        let header = EthHeader {
            hash: (h.hash()).into(),
            parent_hash: (*h.parent_hash()).into(),
            state_root: (*h.state_root()).into(),
            transactions_root: (*h.extrinsics_root()).into(),
            number: Some(U256::from(
                UniqueSaturatedInto::<u128>::unique_saturated_into(*h.number()),
            )),
            ..Default::default()
        };

        Self {
            header,
            ..Default::default()
        }
    }
}
