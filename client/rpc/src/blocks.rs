use super::*;
use crate::types::{EthereumBlock, SubstrateBlock};

impl<B, C, P> EthRPC<B, C, P>
where
    B: BlockT<Hash = ep_eth::H256>,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + BlockBackend<B> + 'static,
    C::Api: EthinkAPI<B>,
{
    fn substrate_hash_by_number(&self, number: BlockNumber) -> RpcResult<B::Hash> {
        Ok(match number {
            BlockNumber::Num(num) => {
                // block num in substrate db is u32
                // https://github.com/paritytech/polkadot-sdk/blob/73c2bca9cdb17f1fdc2afd7aed826d0c55b8640a/substrate/client/rpc/src/chain/mod.rs#L75
                let number = <NumberFor<B>>::try_from(num)
                    .map_err(|_| rpc_err!("Error converting block: {:?}", num))?;
                self.client
                    .hash(number)
                    .map_err(|err| rpc_err!("Failed fetching block: {:?}", err))?
            }
            BlockNumber::Earliest => self
                .client
                .hash(0u32.into())
                .map_err(|err| rpc_err!("Failed fetching block: {:?}", err))?,
            BlockNumber::Latest | BlockNumber::Pending => Some(self.client.info().best_hash),
            BlockNumber::Safe | BlockNumber::Finalized => Some(self.client.info().finalized_hash),
            BlockNumber::Hash { hash, .. } => Some(hash.into()),
        }
        .unwrap_or(self.client.info().best_hash))
    }

    // TODO: e2e tests for this group

    pub async fn block_by_hash(&self, hash: H256, _full: bool) -> RpcResult<Option<RichBlock>> {
        Ok(self
            .client
            .block(hash)
            .map_err(|err| rpc_err!("Failed fetching block: {:?}", err))?
            .map(|b| EthereumBlock::from(SubstrateBlock(b.block)))
            .map(|b| RichBlock {
                inner: b,
                extra_info: BTreeMap::new(),
            }))
    }

    pub async fn block_by_number(
        &self,
        number: BlockNumber,
        _full: bool,
    ) -> RpcResult<Option<RichBlock>> {
        let hash = self.substrate_hash_by_number(number)?;
        // fetch current Substrate block,
        // translate it into Ethereum block,
        // then into RichBlock and return it.
        Ok(self
            .client
            .block(hash)
            .map_err(|err| rpc_err!("Failed fetching block body: {:?}", err))?
            .map(|b| EthereumBlock::from(SubstrateBlock(b.block)))
            .map(|b| RichBlock {
                inner: b,
                extra_info: BTreeMap::new(),
            }))
    }

    pub async fn block_transaction_count_by_hash(&self, hash: H256) -> RpcResult<Option<U256>> {
        Ok(self
            .client
            .block(hash)
            .map_err(|err| rpc_err!("Failed fetching block body: {:?}", err))?
            .map(|b| b.block.extrinsics().iter().count().into()))
    }

    pub async fn block_transaction_count_by_number(
        &self,
        number: BlockNumber,
    ) -> RpcResult<Option<U256>> {
        let hash = self.substrate_hash_by_number(number)?;

        Ok(self
            .client
            .block(hash)
            .map_err(|err| rpc_err!("Failed fetching block body: {:?}", err))?
            .map(|b| b.block.extrinsics().iter().count().into()))
    }
}
