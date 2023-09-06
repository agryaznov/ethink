use super::*;

impl<B, C> Duck<B, C>
where
    B: BlockT,
{
    pub async fn block_by_hash(&self, _hash: H256, _full: bool) -> RpcResult<Option<RichBlock>> {
        Ok(None)
    }

    pub async fn block_by_number(
        &self,
        _number: BlockNumber,
        _full: bool,
    ) -> RpcResult<Option<RichBlock>> {
        Ok(None)
    }

    pub async fn block_transaction_count_by_hash(&self, _hash: H256) -> RpcResult<Option<U256>> {
        Ok(None)
    }

    pub async fn block_transaction_count_by_number(
        &self,
        _number: BlockNumber,
    ) -> RpcResult<Option<U256>> {
        Ok(None)
    }
}
