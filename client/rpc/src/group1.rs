use super::*;

impl<B, C> Duck<B, C>
where
    B: BlockT,
    C: ProvideRuntimeApi<B>,
{
    fn call(
        &self,
        request: CallRequest,
        number: Option<BlockNumber>,
        state_overrides: Option<BTreeMap<H160, CallStateOverride>>,
    ) -> RpcResult<Bytes> {
        Ok(Bytes::default())
    }

    fn send_transaction(&self, request: TransactionRequest) -> RpcResult<H256> {
        Ok(H256::zero())
    }

    fn send_raw_transaction(&self, bytes: Bytes) -> RpcResult<H256> {
        Ok(H256::zero())
    }

    async fn estimate_gas(
        &self,
        request: CallRequest,
        number: Option<BlockNumber>,
    ) -> RpcResult<U256> {
        Ok(U256::zero())
    }
}
