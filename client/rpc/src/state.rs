use super::*;
use ep_eth::AccountId20;
use pallet_ethink::EthinkAPI;
use sc_network::SyncState;
use sp_runtime::traits::UniqueSaturatedInto;

impl<B, C, P> EthRPC<B, C, P>
where
    B: BlockT,
    B::Header: HeaderT,
    <<B as BlockT>::Header as HeaderT>::Number: Into<U256>,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + 'static,
    C::Api: EthinkAPI<B>,
{
    pub fn author(&self) -> RpcResult<H160> {
        Ok(H160::zero())
    }

    pub async fn balance(&self, address: H160, _number: Option<BlockNumber>) -> RpcResult<U256> {
        let hash = self.client.info().best_hash;
        Ok(self
            .client
            .runtime_api()
            .account_free_balance(hash, address)
            .map_err(|err| rpc_err!("Fetching runtime account balance failed: {:?}", err))?
            .into())
    }

    pub fn chain_id(&self) -> RpcResult<Option<U64>> {
        let hash = self.client.info().best_hash;
        Ok(Some(
            self.client
                .runtime_api()
                .chain_id(hash)
                .map_err(|err| rpc_err!("Fetching runtime chain_id failed: {:?}", err))?
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
            .map_err(|err| rpc_err!("Fetching runtime account nounce failed: {:?}", err))?;

        Ok(nonce)
    }

    // This relates to node and not to chain state,
    // just keeping it here for now as it's too little for a separate module
    pub fn accounts(&self) -> RpcResult<Vec<H160>> {
        // Get signing accounts from the "ethi" keystore
        let accounts = self
            .keystore
            .ecdsa_public_keys(ETHINK_KEYTYPE_ID)
            .iter()
            .map(|k| H160::from(AccountId20::from(*k)))
            .collect::<Vec<_>>();

        Ok(accounts)
    }

    // TODO put to client
    // This relates to node and not to chain state,
    // just keeping it here now as it's too little for a separate module
    pub async fn syncing(&self) -> RpcResult<SyncStatus> {
   		match self.sync.status().await {
            Ok(s) => {
                let current_block = match s.state {
                   SyncState::<<B::Header as HeaderT>::Number>::Idle => return Ok(SyncStatus::None),
                   SyncState::<<B::Header as HeaderT>::Number>::Downloading{target} | SyncState::<<B::Header as HeaderT>::Number>::Importing{target} => target.into(),
                };
              Ok(SyncStatus::Info(SyncInfo {
				starting_block: U256::zero(),
				current_block,
				highest_block: s.best_seen_block.map(Into::into).unwrap_or(current_block),
				warp_chunks_amount: None,
				warp_chunks_processed: None,
			}))},
            Err(e) =>
                return Err(rpc_err!("Failed getting syncyncing status: {:?}", e))
        }
    }
}
