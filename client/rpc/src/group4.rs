use super::*;

impl<B, C> Duck<B, C>
where
    B: BlockT,
{
    pub fn accounts(&self) -> RpcResult<Vec<H160>> {
        Ok(vec![H160::zero()])
    }

    pub fn author(&self) -> RpcResult<H160> {
        Ok(H160::zero())
    }

    pub async fn balance(&self, _address: H160, _number: Option<BlockNumber>) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    pub fn chain_id(&self) -> RpcResult<Option<U64>> {
        Ok(None)
    }

    pub async fn code_at(&self, _address: H160, _number: Option<BlockNumber>) -> RpcResult<Bytes> {
        Ok(Bytes::default())
    }

    pub fn block_number(&self) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    pub fn gas_price(&self) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    pub async fn storage_at(
        &self,
        _address: H160,
        _index: U256,
        _number: Option<BlockNumber>,
    ) -> RpcResult<H256> {
        Ok(H256::zero())
    }

    pub fn syncing(&self) -> RpcResult<SyncStatus> {
        Ok(SyncStatus::None)
    }

    pub async fn transaction_count(
        &self,
        _address: H160,
        _number: Option<BlockNumber>,
    ) -> RpcResult<U256> {
        Ok(42u8.into())
    }
}
