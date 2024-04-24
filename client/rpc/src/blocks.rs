use super::*;
use crate::types::SubstrateBlock;

impl<B, C, P> EthRPC<B, C, P>
where
    B: BlockT<Hash = ep_eth::H256>,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + BlockBackend<B> + 'static,
    C::Api: EthinkAPI<B>,
{
    /// Fetch Substrate block hash by its number
    fn substrate_block_hash_by_number(&self, number: BlockNumber) -> RpcResult<B::Hash> {
        Ok(match number {
            BlockNumber::Num(num) => {
                // block num in substrate db is u32
                // https://github.com/paritytech/polkadot-sdk/blob/73c2bca9cdb17f1fdc2afd7aed826d0c55b8640a/substrate/client/rpc/src/chain/mod.rs#L75
                let number = <NumberFor<B>>::try_from(num)
                    .map_err(|_| rpc_err!("Error converting block number: {:?}", num))?;
                self.client
                    .hash(number)
                    .map_err(|err| rpc_err!("Failed fetching block hash by number: {:?}", err))?
            }
            BlockNumber::Earliest => self
                .client
                .hash(0u32.into())
                .map_err(|err| rpc_err!("Failed fetching earliest block: {:?}", err))?,
            BlockNumber::Latest | BlockNumber::Pending => Some(self.client.info().best_hash),
            BlockNumber::Safe | BlockNumber::Finalized => Some(self.client.info().finalized_hash),
            BlockNumber::Hash { hash, .. } => Some(hash.into()),
        }
        .unwrap_or(self.client.info().best_hash))
    }

    /// Fetch Substrate block by its hash
    fn substrate_block_by_hash(&self, hash: H256) -> RpcResult<Option<SubstrateBlock<B>>> {
        self.client
            .block(hash)
            .map_err(|err| rpc_err!("Failed fetching block: {:?}", err))
            .map(|r| r.map(SubstrateBlock::new))
    }

    // TODO: e2e tests for this group

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
        let hash = self.substrate_block_hash_by_number(number)?;
        Ok(self.substrate_block_by_hash(hash)?.map(RichBlock::from))
    }

    /// Get number of transactions in a block fetched by its hash
    pub async fn block_transaction_count_by_hash(&self, hash: H256) -> RpcResult<Option<U256>> {
        Ok(self
            .substrate_block_by_hash(hash)?
            .map(|b| b.extrinsic_count()))
    }

    /// Get number of transactions in a block fetched by its number
    pub async fn block_transaction_count_by_number(
        &self,
        number: BlockNumber,
    ) -> RpcResult<Option<U256>> {
        let hash = self.substrate_block_hash_by_number(number)?;

        Ok(self
            .substrate_block_by_hash(hash)?
            .map(|b| b.extrinsic_count()))
    }
}
