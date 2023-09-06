use super::*;

impl<B, C> Duck<B, C>
where
    B: BlockT,
{
    pub async fn transaction_by_hash(&self, _hash: H256) -> RpcResult<Option<Transaction>> {
        Ok(None)
    }

    pub async fn transaction_by_block_hash_and_index(
        &self,
        _hash: H256,
        _index: Index,
    ) -> RpcResult<Option<Transaction>> {
        Ok(None)
    }

    pub async fn transaction_by_block_number_and_index(
        &self,
        _number: BlockNumber,
        _index: Index,
    ) -> RpcResult<Option<Transaction>> {
        Ok(None)
    }

    pub async fn transaction_receipt(&self, _hash: H256) -> RpcResult<Option<Receipt>> {
        Ok(None)
    }
}
