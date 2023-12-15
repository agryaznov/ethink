use super::*;

impl<B, C> Duck<B, C>
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

    pub async fn transaction_receipt(&self, _hash: H256) -> RpcResult<Option<Receipt>> {
        // Ok(Some(Receipt {
        //      transaction_hash: Option<H256>,
        //      transaction_index: Option<U256>,
        //      block_hash: Option<H256>,
        //      from: Option<H160>,
        //      to: Option<H160>,
        //      block_number: Option<U256>,
        //      contract_address: Default::default(),
        //      cumulative_gas_used: Default::default(),
        //      gas_used: Option<U256>,
        //      logs: Vec<Log>,
        //      state_root: Option<H256>,
        //      logs_bloom: H2048,
        //      status_code: Option<U64>,
        //      effective_gas_price: U256,
        //      transaction_type: U256,
        // }))
        Ok(None)
    }
}
