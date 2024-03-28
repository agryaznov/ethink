// ETH RPC methods grouped according to mapping table
mod group1;
mod group2;
mod group3;
mod group4;
mod group5;

// auxilinary utils
mod signer;

use ep_mapping;
use ep_rpc::ETHRuntimeRPC;
use ethereum::TransactionV2 as EthTx;
use ethereum_types::{H160, H256, H64, U256, U64};
use ethink_rpc_core::types::*;
use futures::future::TryFutureExt;
use jsonrpsee::core::{async_trait, RpcResult};
use sc_client_api::BlockBackend;
use sc_transaction_pool_api::TransactionPool;
use sp_api::{HeaderT, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_core::crypto::KeyTypeId;
use sp_keystore::Keystore;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, NumberFor, PhantomData},
    transaction_validity::TransactionSource,
};
use std::{collections::BTreeMap, sync::Arc};

pub use ethink_rpc_core::{types::Transaction as Tx, EthApiServer};

pub const ETHINK_KEYTYPE_ID: KeyTypeId = KeyTypeId(*b"ethi");

pub fn err<T: ToString>(code: i32, message: T, data: Option<&[u8]>) -> jsonrpsee::core::Error {
    jsonrpsee::core::Error::Call(jsonrpsee::types::error::CallError::Custom(
        jsonrpsee::types::error::ErrorObject::owned(
            code,
            message.to_string(),
            data.map(|bytes| {
                jsonrpsee::core::to_json_raw_value(&format!("0x{}", hex::encode(bytes)))
                    .expect("fail to serialize data")
            }),
        ),
    ))
}

pub fn internal_err<T: ToString>(message: T) -> jsonrpsee::core::Error {
    err(jsonrpsee::types::error::INTERNAL_ERROR_CODE, message, None)
}

/// Eth RPC implementation.
pub struct EthRPC<B: BlockT, C, P> {
    client: Arc<C>,
    pool: Arc<P>,
    keystore: Arc<dyn Keystore>,
    _phantom: PhantomData<B>,
}

impl<B, C, P> EthRPC<B, C, P>
where
    B: BlockT<Hash = sp_core::H256>,
    B::Header: HeaderT<Number = u32>,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + BlockBackend<B> + 'static,
    P: TransactionPool<Block = B> + 'static,
    C::Api: ETHRuntimeRPC<B>,
{
    pub fn new(client: Arc<C>, pool: Arc<P>, keystore: Arc<dyn Keystore>) -> Self {
        Self {
            client,
            pool,
            keystore,
            _phantom: PhantomData::default(),
        }
    }
}

impl<B, C, P> EthRPC<B, C, P>
where
    B: BlockT<Hash = sp_core::H256>,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + 'static,
    P: TransactionPool<Block = B> + 'static,
    C::Api: ETHRuntimeRPC<B>,
{
    async fn compose_extrinsic_and_submit(&self, hash: H256, tx: EthTx) -> RpcResult<H256> {
        let tx_hash = tx.hash();
        let extrinsic = self
            .client
            .runtime_api()
            .convert_transaction(hash, tx)
            .map_err(|_| internal_err("cannot access runtime api"))?;
        // Submit extrinsic to pool
        self.pool
            .submit_one(&BlockId::Hash(hash), TransactionSource::Local, extrinsic)
            .map_ok(move |_| tx_hash)
            .map_err(internal_err)
            .await
    }
}

#[async_trait]
impl<B, C, P> EthApiServer for EthRPC<B, C, P>
where
    B: BlockT<Hash = ethereum_types::H256>,
    B::Header: HeaderT<Number = u32>,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + BlockBackend<B> + 'static,
    P: TransactionPool<Block = B> + 'static,
    C::Api: ETHRuntimeRPC<B>,
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

    fn version(&self) -> RpcResult<String> {
        Ok(String::from("1703871830822"))
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
