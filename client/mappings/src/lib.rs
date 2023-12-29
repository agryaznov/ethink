pub use polkamask_rpc_core::types::Block as EthBlock;

use ethereum_types::{H256, U256};
use polkamask_rpc_core::types::{BlockTransactions as EthBlockTxs, Header as EthHeader};
use sp_runtime::traits::{Block as BlockT, Header, UniqueSaturatedInto};

pub struct SubBlock<B>(pub B);

impl<B: BlockT<Hash = H256>> From<SubBlock<B>> for EthBlock {
    // Generate dumb eth block from the given substrate block
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
            uncles_hash: Default::default(),
            author: Default::default(),
            miner: Default::default(),
            receipts_root: Default::default(),
            gas_used: Default::default(),
            gas_limit: Default::default(),
            extra_data: Default::default(),
            logs_bloom: Default::default(),
            timestamp: Default::default(),
            difficulty: Default::default(),
            nonce: Default::default(),
            size: Default::default(),
        };
        // For now just fill it with empty
        let transactions = EthBlockTxs::Hashes(vec![]);

        Self {
            header,
            transactions,
            total_difficulty: Default::default(),
            uncles: Default::default(),
            size: Default::default(),
            base_fee_per_gas: Default::default(),
        }
    }
}
