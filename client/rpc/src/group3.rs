use super::*;
use sp_api::HeaderT;
use sp_runtime::traits::UniqueSaturatedInto;

impl<B, C, P> Duck<B, C, P>
where
    B: BlockT<Hash = ethereum_types::H256>,
    B::Header: HeaderT<Number = u32>,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + BlockBackend<B> + 'static,
{
    pub async fn transaction_by_hash(&self, _hash: H256) -> RpcResult<Option<Transaction>> {
        Ok(None)
    }

    pub async fn transaction_by_block_hash_and_index(
        &self,
        _hash: H256,
        _index: Index,
    ) -> RpcResult<Option<Transaction>> {
        Ok(None)
    }

    pub async fn transaction_by_block_number_and_index(
        &self,
        _number: BlockNumber,
        _index: Index,
    ) -> RpcResult<Option<Transaction>> {
        Ok(None)
    }

    // NOTE: !!! tx_hash in polkadot is not unique... block_hash ++ tx_hash is unique
    // so as for now we will just return first (of possible many) found receipt for the provided tx_hash
    pub async fn transaction_receipt(&self, hash: H256) -> RpcResult<Option<Receipt>> {
        // For now we generate fake receipt just to provide some response to MM queries
        let block_hash = Some(self.client.info().best_hash);
        let block_number = Some(
            UniqueSaturatedInto::<u128>::unique_saturated_into(self.client.info().best_number)
                .into(),
        );

        Ok(Some(Receipt {
            transaction_hash: Some(hash),
            transaction_index: Some(1.into()),
            block_hash,
            block_number,
            from: Default::default(),
            to: Default::default(),
            state_root: Default::default(),
            transaction_type: Default::default(),
            contract_address: Default::default(),
            cumulative_gas_used: Default::default(),
            gas_used: Default::default(),
            logs: Default::default(),
            logs_bloom: Default::default(),
            status_code: Default::default(),
            effective_gas_price: Default::default(),
        }))
    }
}
