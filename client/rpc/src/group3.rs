use super::*;

impl<B, C, P> Duck<B, C, P>
where
    B: BlockT,
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
    pub async fn transaction_receipt(&self, _hash: H256) -> RpcResult<Option<Receipt>> {
        // Ok(Some(Receipt {
        //      transaction_hash: Option<H256>,
        //      transaction_index: Option<U256>,
        //      block_hash: Option<H256>,
        //      from: Option<H160>,
        //      to: Option<H160>,
        //      block_number: Option<U256>,
        //      state_root: Default::default(),//?
        //      transaction_type: Default::default(),//?
        //      contract_address: Default::default(),
        //      cumulative_gas_used: Default::default(),
        //      gas_used: Default::default(),
        //      logs: Default::default(),
        //      logs_bloom: Default::default(),
        //      status_code: Default::default(),
        //      effective_gas_price: Default::default(),
        // }))
        Ok(None)
    }
}
