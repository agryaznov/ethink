use super::*;

impl<B, C> Duck<B, C>
where
    B: BlockT,
    C: ProvideRuntimeApi<B>,
{
    fn accounts(&self) -> RpcResult<Vec<H160>> {
        Ok(vec![H160::zero()])
    }

    fn author(&self) -> RpcResult<H160> {
        Ok(H160::zero())
    }

    async fn balance(&self, address: H160, number: Option<BlockNumber>) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    fn chain_id(&self) -> RpcResult<Option<U64>> {
        Ok(None)
    }

    async fn code_at(&self, address: H160, number: Option<BlockNumber>) -> RpcResult<Bytes> {
        Ok(Bytes::default())
    }

    fn block_number(&self) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    fn gas_price(&self) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    fn storage_at(
        &self,
        address: H160,
        index: U256,
        number: Option<BlockNumber>,
    ) -> RpcResult<H256> {
        Ok(H256::zero())
    }

    fn syncing(&self) -> RpcResult<SyncStatus> {
        Ok(SyncStatus::None)
    }

    async fn transaction_count(
        &self,
        address: H160,
        number: Option<BlockNumber>,
    ) -> RpcResult<U256> {
        Ok(42u8.into())
    }
}
