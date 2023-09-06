use super::*;

impl<B: BlockT, C> Duck<B, C> {
    pub async fn fee_history(
        &self,
        _block_count: U256,
        _newest_block: BlockNumber,
        _reward_percentiles: Option<Vec<f64>>,
    ) -> RpcResult<FeeHistory> {
        Ok(FeeHistory {
            oldest_block: U256::zero(),
            base_fee_per_gas: vec![U256::zero()],
            gas_used_ratio: vec![0.],
            reward: Some(vec![vec![U256::zero()]]),
        })
    }

    pub fn is_mining(&self) -> RpcResult<bool> {
        Ok(false)
    }

    pub fn max_priority_fee_per_gas(&self) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    pub fn hashrate(&self) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    pub fn protocol_version(&self) -> RpcResult<u64> {
        Ok(0)
    }

    pub fn submit_hashrate(&self, _hashrate: U256, _id: H256) -> RpcResult<bool> {
        Ok(false)
    }

    pub fn submit_work(&self, _nonce: H64, _pow_hash: H256, _mix_digest: H256) -> RpcResult<bool> {
        Ok(false)
    }

    pub fn block_uncles_count_by_number(&self, _number: BlockNumber) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    pub fn block_uncles_count_by_hash(&self, _hash: H256) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    pub fn uncle_by_block_hash_and_index(
        &self,
        _hash: H256,
        _index: Index,
    ) -> RpcResult<Option<RichBlock>> {
        Ok(None)
    }

    pub fn uncle_by_block_number_and_index(
        &self,
        _number: BlockNumber,
        _index: Index,
    ) -> RpcResult<Option<RichBlock>> {
        Ok(None)
    }

    pub fn work(&self) -> RpcResult<Work> {
        Ok(Work::default())
    }
}
