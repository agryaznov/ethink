use super::*;
use crate::types::SubstrateBlock;

impl<B, C, P> EthRPC<B, C, P>
where
    B: BlockT<Hash = ep_eth::H256>,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + BlockBackend<B> + 'static,
    C::Api: EthinkAPI<B>,
{
    /// Fetch Substrate block hash by its number
    async fn substrate_block_hash_by_number(
        &self,
        number: BlockNumber,
    ) -> RpcResult<Option<B::Hash>> {
        block_hash::<B, C>(&self.client, Some(number))
            .await
            .map(|h| Some(h))
    }

    /// Fetch Substrate block by its hash
    fn substrate_block_by_hash(&self, hash: H256) -> RpcResult<Option<SubstrateBlock<B>>> {
        self.client
            .block(hash)
            .map_err(|err| rpc_err!("Failed fetching block: {:?}", err))
            .map(|r| r.map(SubstrateBlock::new))
    }

    /// Fetch Substrate block by its hash and convert it to RichBlock.
    pub async fn block_by_hash(&self, hash: H256, _full: bool) -> RpcResult<Option<RichBlock>> {
        Ok(self.substrate_block_by_hash(hash)?.map(RichBlock::from))
    }

    /// Fetch Substrate block by its number and convert it to RichBlock.
    pub async fn block_by_number(
        &self,
        number: BlockNumber,
        _full: bool,
    ) -> RpcResult<Option<RichBlock>> {
        Ok(
            if let Some(hash) = self.substrate_block_hash_by_number(number).await? {
                self.substrate_block_by_hash(hash)?.map(RichBlock::from)
            } else {
                None
            },
        )
    }

    /// Get number of transactions in a block fetched by its hash.
    pub async fn block_transaction_count_by_hash(&self, hash: H256) -> RpcResult<Option<U256>> {
        Ok(self
            .substrate_block_by_hash(hash)?
            .map(|b| b.extrinsic_count()))
    }

    /// Get number of transactions in a block fetched by its number.
    pub async fn block_transaction_count_by_number(
        &self,
        number: BlockNumber,
    ) -> RpcResult<Option<U256>> {
        Ok(
            if let Some(hash) = self.substrate_block_hash_by_number(number).await? {
                self.substrate_block_by_hash(hash)?
                    .map(|b| b.extrinsic_count())
            } else {
                None
            },
        )
    }
}
