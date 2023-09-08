mod group1;
mod group2;
mod group3;
mod group4;
mod group5;

use ethereum_types::{H160, H256, H64, U256, U64};
use fc_rpc_core::types::*;
pub use fc_rpc_core::EthApiServer;
use jsonrpsee::core::{async_trait, RpcResult};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::{Block as BlockT, PhantomData};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Eth RPC implementation.
pub struct Duck<B: BlockT, C> {
    client: Arc<C>,
    _phantom: PhantomData<B>,
}

impl<B, C> Duck<B, C>
where
    B: BlockT,
    C: ProvideRuntimeApi<B>,
{
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _phantom: PhantomData::default(),
        }
    }
}

#[async_trait]
impl<B, C> EthApiServer for Duck<B, C>
where
    B: BlockT + 'static,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + 'static,
    //C::Api: EthereumRuntimeRPCApi<B>,
{
    // ########################################################################
    // Group 5: Mocked
    // ########################################################################

    async fn fee_history(
        &self,
        block_count: U256,
        newest_block: BlockNumber,
        reward_percentiles: Option<Vec<f64>>,
    ) -> RpcResult<FeeHistory> {
        self.fee_history(block_count, newest_block, reward_percentiles)
            .await
    }

    fn is_mining(&self) -> RpcResult<bool> {
        self.is_mining()
    }

    fn max_priority_fee_per_gas(&self) -> RpcResult<U256> {
        self.max_priority_fee_per_gas()
    }

    fn hashrate(&self) -> RpcResult<U256> {
        self.hashrate()
    }

    fn protocol_version(&self) -> RpcResult<u64> {
        self.protocol_version()
    }

    fn submit_hashrate(&self, hashrate: U256, id: H256) -> RpcResult<bool> {
        self.submit_hashrate(hashrate, id)
    }

    fn submit_work(&self, nonce: H64, pow_hash: H256, mix_digest: H256) -> RpcResult<bool> {
        self.submit_work(nonce, pow_hash, mix_digest)
    }

    fn block_uncles_count_by_number(&self, number: BlockNumber) -> RpcResult<U256> {
        self.block_uncles_count_by_number(number)
    }

    fn block_uncles_count_by_hash(&self, hash: H256) -> RpcResult<U256> {
        self.block_uncles_count_by_hash(hash)
    }

    fn uncle_by_block_hash_and_index(
        &self,
        hash: H256,
        index: Index,
    ) -> RpcResult<Option<RichBlock>> {
        self.uncle_by_block_hash_and_index(hash, index)
    }

    fn uncle_by_block_number_and_index(
        &self,
        number: BlockNumber,
        index: Index,
    ) -> RpcResult<Option<RichBlock>> {
        self.uncle_by_block_number_and_index(number, index)
    }

    fn work(&self) -> RpcResult<Work> {
        self.work()
    }

    // ########################################################################
    // Group 4: State
    // ########################################################################

    fn accounts(&self) -> RpcResult<Vec<H160>> {
        self.accounts()
    }

    fn author(&self) -> RpcResult<H160> {
        self.author()
    }

    async fn balance(&self, address: H160, number: Option<BlockNumber>) -> RpcResult<U256> {
        self.balance(address, number).await
    }

    fn chain_id(&self) -> RpcResult<Option<U64>> {
        self.chain_id()
    }

    async fn code_at(&self, address: H160, number: Option<BlockNumber>) -> RpcResult<Bytes> {
        self.code_at(address, number).await
    }

    fn block_number(&self) -> RpcResult<U256> {
        self.block_number()
    }

    fn gas_price(&self) -> RpcResult<U256> {
        self.gas_price()
    }

    async fn storage_at(
        &self,
        address: H160,
        index: U256,
        number: Option<BlockNumber>,
    ) -> RpcResult<H256> {
        self.storage_at(address, index, number).await
    }

    fn syncing(&self) -> RpcResult<SyncStatus> {
        self.syncing()
    }

    async fn transaction_count(
        &self,
        address: H160,
        number: Option<BlockNumber>,
    ) -> RpcResult<U256> {
        self.transaction_count(address, number).await
    }

    // ########################################################################
    // Group 3: Transaction
    // ########################################################################

    async fn transaction_by_hash(&self, hash: H256) -> RpcResult<Option<Transaction>> {
        self.transaction_by_hash(hash).await
    }

    async fn transaction_by_block_hash_and_index(
        &self,
        hash: H256,
        index: Index,
    ) -> RpcResult<Option<Transaction>> {
        self.transaction_by_block_hash_and_index(hash, index).await
    }

    async fn transaction_by_block_number_and_index(
        &self,
        number: BlockNumber,
        index: Index,
    ) -> RpcResult<Option<Transaction>> {
        self.transaction_by_block_number_and_index(number, index)
            .await
    }

    async fn transaction_receipt(&self, hash: H256) -> RpcResult<Option<Receipt>> {
        self.transaction_receipt(hash).await
    }

    // ########################################################################
    // Group 2: Block
    // ########################################################################

    async fn block_by_hash(&self, hash: H256, full: bool) -> RpcResult<Option<RichBlock>> {
        self.block_by_hash(hash, full).await
    }

    async fn block_by_number(
        &self,
        number: BlockNumber,
        full: bool,
    ) -> RpcResult<Option<RichBlock>> {
        self.block_by_number(number, full).await
    }

    async fn block_transaction_count_by_hash(&self, hash: H256) -> RpcResult<Option<U256>> {
        self.block_transaction_count_by_hash(hash).await
    }

    async fn block_transaction_count_by_number(
        &self,
        number: BlockNumber,
    ) -> RpcResult<Option<U256>> {
        self.block_transaction_count_by_number(number).await
    }

    // ########################################################################
    // Group 1: Execute
    // ########################################################################

    async fn call(
        &self,
        request: CallRequest,
        number: Option<BlockNumber>,
        state_overrides: Option<BTreeMap<H160, CallStateOverride>>,
    ) -> RpcResult<Bytes> {
        self.call(request, number, state_overrides).await
    }

    async fn send_transaction(&self, request: TransactionRequest) -> RpcResult<H256> {
        self.send_transaction(request).await
    }

    async fn send_raw_transaction(&self, bytes: Bytes) -> RpcResult<H256> {
        self.send_raw_transaction(bytes).await
    }

    async fn estimate_gas(
        &self,
        request: CallRequest,
        number: Option<BlockNumber>,
    ) -> RpcResult<U256> {
        self.estimate_gas(request, number).await
    }
}
