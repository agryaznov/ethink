use super::*;

impl<B, C> Duck<B, C>
where
    B: BlockT,
{
    pub async fn call(
        &self,
        _request: CallRequest,
        _number: Option<BlockNumber>,
        _state_overrides: Option<BTreeMap<H160, CallStateOverride>>,
    ) -> RpcResult<Bytes> {
        Ok(Bytes::default())
    }

    pub async fn send_transaction(&self, _request: TransactionRequest) -> RpcResult<H256> {
        Ok(H256::zero())
    }

    pub async fn send_raw_transaction(&self, _bytes: Bytes) -> RpcResult<H256> {
        Ok(H256::zero())
    }

    pub async fn estimate_gas(
        &self,
        _request: CallRequest,
        _number: Option<BlockNumber>,
    ) -> RpcResult<U256> {
        Ok(1000u32.into())
    }
}
