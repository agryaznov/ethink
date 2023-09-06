use super::*;

impl<B, C> Duck<B, C>
where
    B: BlockT,
    C: ProvideRuntimeApi<B>,
{
    async fn fee_history(
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

    fn is_mining(&self) -> RpcResult<bool> {
        Ok(false)
    }

    fn max_priority_fee_per_gas(&self) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    fn hashrate(&self) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    fn protocol_version(&self) -> RpcResult<u64> {
        Ok(0)
    }

    fn submit_hashrate(&self, hashrate: U256, id: H256) -> RpcResult<bool> {
        Ok(false)
    }

    fn submit_work(&self, nonce: H64, pow_hash: H256, mix_digest: H256) -> RpcResult<bool> {
        Ok(false)
    }

    fn block_uncles_count_by_number(&self, number: BlockNumber) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    fn block_uncles_count_by_hash(&self, hash: H256) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    fn uncle_by_block_hash_and_index(
        &self,
        hash: H256,
        index: Index,
    ) -> RpcResult<Option<RichBlock>> {
        Ok(None)
    }

    fn uncle_by_block_number_and_index(
        &self,
        number: BlockNumber,
        index: Index,
    ) -> RpcResult<Option<RichBlock>> {
        Ok(None)
    }

    fn work(&self) -> RpcResult<Work> {
        Ok(Work::default())
    }
}
