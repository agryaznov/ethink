use super::*;
use crate::CallRequest;

impl<B, C> Duck<B, C>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + 'static,
    C::Api: ETHRuntimeRPC<B>,
{
    pub async fn call(
        &self,
        request: CallRequest,
        _number: Option<BlockNumber>,
        _state_overrides: Option<BTreeMap<H160, CallStateOverride>>,
    ) -> RpcResult<Bytes> {
        // ensure_signer(origin)?
        let hash = self.client.info().best_hash;

        let CallRequest {
            from, to, value, ..
        } = request;

        let balance_left = self
            .client
            .runtime_api()
            .call(hash, from.unwrap(), to.unwrap(), value.unwrap())
     		.map_err(|err| internal_err(format!("execution fatal: {:?}", err)))?
            .map_err(|err| internal_err(format!("runtime error on call: {:?}", err)))?;


        Ok(Bytes::from(balance_left.as_u128().to_be_bytes().to_vec()))
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
