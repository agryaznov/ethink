use super::*;
use sp_api::HeaderT;
use sp_runtime::traits::UniqueSaturatedInto;

impl<B, C, P> EthRPC<B, C, P>
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

    // NOTE: tx_hash in Polkadot is not unique... block_hash ++ tx_hash is unique.
    // Anyway for now we generate a fake receipt just to provide some response to MetaMask queries.
    pub async fn transaction_receipt(&self, hash: H256) -> RpcResult<Option<Receipt>> {
        let transaction_hash = Some(hash);
        // Always answer as if:
        // the tx was successfully included
        let status_code = Some(1u64.into());
        // into the recent block
        let block_hash = Some(self.client.info().best_hash);
        let block_number = Some(
            UniqueSaturatedInto::<u128>::unique_saturated_into(self.client.info().best_number)
                .into(),
        );
        // with tx_id = 1
        let transaction_index = Some(1.into());

        Ok(Some(Receipt {
            transaction_hash,
            transaction_index,
            status_code,
            block_hash,
            block_number,
            ..Default::default()
        }))
    }
}
