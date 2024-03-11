use super::*;
use ep_rpc::ETHRuntimeRPC;
use sp_runtime::traits::UniqueSaturatedInto;

impl<B, C, P> EthRPC<B, C, P>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + 'static,
    C::Api: ETHRuntimeRPC<B>,
{
    pub fn accounts(&self) -> RpcResult<Vec<H160>> {
        Ok(vec![H160::zero()])
    }

    pub fn author(&self) -> RpcResult<H160> {
        Ok(H160::zero())
    }

    pub async fn balance(&self, address: H160, _number: Option<BlockNumber>) -> RpcResult<U256> {
        let hash = self.client.info().best_hash;
        Ok(self
            .client
            .runtime_api()
            .account_free_balance(hash, address)
            .map_err(|err| internal_err(format!("Fetching runtime CHAIN_ID failed: {:?}", err)))?
            .into())
    }

    pub fn chain_id(&self) -> RpcResult<Option<U64>> {
        let hash = self.client.info().best_hash;
        Ok(Some(
            self.client
                .runtime_api()
                .chain_id(hash)
                .map_err(|err| {
                    internal_err(format!("Fetching runtime CHAIN_ID failed: {:?}", err))
                })?
                .into(),
        ))
    }

    pub async fn code_at(&self, _address: H160, _number: Option<BlockNumber>) -> RpcResult<Bytes> {
        Ok(Bytes::default())
    }

    pub fn block_number(&self) -> RpcResult<U256> {
        Ok(U256::from(
            UniqueSaturatedInto::<u128>::unique_saturated_into(self.client.info().best_number),
        ))
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
        address: H160,
        _number: Option<BlockNumber>,
    ) -> RpcResult<U256> {
        let hash = self.client.info().best_hash;
        let nonce = self
            .client
            .runtime_api()
            .nonce(hash, address)
            .map_err(|err| {
                internal_err(format!("fetch runtime account nounce failed: {:?}", err))
            })?;

        Ok(nonce)
    }
}
