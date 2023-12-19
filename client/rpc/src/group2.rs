use super::*;
use mappings::{EthBlock, SubBlock};

impl<B, C, P> Duck<B, C, P>
where
    B: BlockT<Hash = ethereum_types::H256>,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + BlockBackend<B> + 'static,
    C::Api: ETHRuntimeRPC<B>,
{
    pub async fn block_by_hash(&self, hash: H256, _full: bool) -> RpcResult<Option<RichBlock>> {
        Ok(self
            .client
            .block(hash)
            .map_err(|err| internal_err(format!("Failed fetching block: {:?}", err)))?
            .map(|b| EthBlock::from(SubBlock(b.block)))
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
        // TODO
        // 1. fetch current substrate block
        //    see how it's done here: https://github.com/paritytech/polkadot-sdk/blob/73c2bca9cdb17f1fdc2afd7aed826d0c55b8640a/substrate/client/rpc/src/chain/chain_full.rs#L66
        //
        // in short,
        // + get block body with Backend::body
        //   https://github.com/paritytech/polkadot-sdk/blob/73c2bca9cdb17f1fdc2afd7aed826d0c55b8640a/substrate/client/rpc/src/chain/chain_full.rs#L66
        //
        // + get block header with HeaderBackend::header

        // TODO make map_err(Into::into)
        // like in
        // https://github.com/paritytech/polkadot-sdk/blob/73c2bca9cdb17f1fdc2afd7aed826d0c55b8640a/substrate/client/rpc/src/chain/mod.rs#L136
        // TODO: put to utils?
        let hash = match number {
            BlockNumber::Num(num) => {
                // block num in substrate db is u32
                // https://github.com/paritytech/polkadot-sdk/blob/73c2bca9cdb17f1fdc2afd7aed826d0c55b8640a/substrate/client/rpc/src/chain/mod.rs#L75
                let number = <NumberFor<B>>::try_from(num)
                    .map_err(|_| internal_err(format!("Error converting block: {:?}", num)))?;
                self.client
                    .hash(number)
                    .map_err(|err| internal_err(format!("Failed fetching block: {:?}", err)))?
            }
            BlockNumber::Earliest => self
                .client
                .hash(0u32.into())
                .map_err(|err| internal_err(format!("Failed fetching block: {:?}", err)))?,
            BlockNumber::Latest | BlockNumber::Pending => Some(self.client.info().best_hash),
            BlockNumber::Safe | BlockNumber::Finalized => Some(self.client.info().finalized_hash),
            BlockNumber::Hash { hash, .. } => Some(hash.into()),
        }
        .unwrap_or(self.client.info().best_hash);

        // 2. translate it into ethereum block (this logic tbd in mappings crate)
        // 3. create RichBlock and return it
        Ok(self
            .client
            .block(hash)
            .map_err(|err| internal_err(format!("Failed fetching block body: {:?}", err)))?
            .map(|b| EthBlock::from(SubBlock(b.block)))
            .map(|b| RichBlock {
                inner: b,
                extra_info: BTreeMap::new(),
            }))
    }

    pub async fn block_transaction_count_by_hash(&self, hash: H256) -> RpcResult<Option<U256>> {
        Ok(self
            .client
            .block(hash)
            .map_err(|err| internal_err(format!("Failed fetching block body: {:?}", err)))?
            .map(|b| b.block.extrinsics().iter().count().into()))
    }

    pub async fn block_transaction_count_by_number(
        &self,
        number: BlockNumber,
    ) -> RpcResult<Option<U256>> {
        // TODO: put to utils?
        let hash = match number {
            BlockNumber::Num(num) => {
                // block num in substrate db is u32
                // https://github.com/paritytech/polkadot-sdk/blob/73c2bca9cdb17f1fdc2afd7aed826d0c55b8640a/substrate/client/rpc/src/chain/mod.rs#L75
                let number = <NumberFor<B>>::try_from(num)
                    .map_err(|err| internal_err(format!("Error converting block: {:?}", num)))?;
                self.client
                    .hash(number)
                    .map_err(|err| internal_err(format!("Failed fetching block: {:?}", err)))?
            }
            BlockNumber::Earliest => self
                .client
                .hash(0u32.into())
                .map_err(|err| internal_err(format!("Failed fetching block: {:?}", err)))?,
            BlockNumber::Latest | BlockNumber::Pending => Some(self.client.info().best_hash),
            BlockNumber::Safe | BlockNumber::Finalized => Some(self.client.info().finalized_hash),
            BlockNumber::Hash { hash, .. } => Some(hash.into()),
        }
        .unwrap_or(self.client.info().best_hash);
        Ok(self
            .client
            .block(hash)
            .map_err(|err| internal_err(format!("Failed fetching block body: {:?}", err)))?
            .map(|b| b.block.extrinsics().iter().count().into()))
    }
}
