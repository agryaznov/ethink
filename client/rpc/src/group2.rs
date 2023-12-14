use super::*;

impl<B, C> Duck<B, C>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + Backend<B> + 'static,
    C::Api: ETHRuntimeRPC<B>,
{
    pub async fn block_by_hash(&self, _hash: H256, _full: bool) -> RpcResult<Option<RichBlock>> {
        Ok(None)
    }

    pub async fn block_by_number(
        &self,
        number: BlockNumber,
        _full: bool,
    ) -> RpcResult<Option<RichBlock>> {
        // TODO
        // 1. fetch current substrate block
        //    see how it's done here: https://github.com/paritytech/polkadot-sdk/blob/73c2bca9cdb17f1fdc2afd7aed826d0c55b8640a/substrate/client/rpc/src/chain/chain_full.rs#L66
        //
        // in short,
        // + get block body with Backend::body
        //   https://github.com/paritytech/polkadot-sdk/blob/73c2bca9cdb17f1fdc2afd7aed826d0c55b8640a/substrate/client/rpc/src/chain/chain_full.rs#L66
        //
        // + get block header with HeaderBackend::header

        // TODO: hash by number instead, and fallback to current?
        let hash = self.client.info().best_hash;
        let substrate_header = self
            .client
            .header(hash)
            .map_err(|err| internal_err(format!("Failed fetching block header: {:?}", err)))?;
        let substrate_block = self
            .client
            .body(hash)
            .map_err(|err| internal_err(format!("Failed fetching block body: {:?}", err)))?;
        // 2. translate it into ethereum block (this logic tbd in mappings crate)
        // 3. create RichBlock and return it

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
