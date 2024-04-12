use crate::{types::EthereumSigner, CallRequest, *};
use ep_crypto::AccountId20;
use ethereum::{LegacyTransaction, LegacyTransactionMessage};

impl<B, C, P> EthRPC<B, C, P>
where
    B: BlockT<Hash = sp_core::H256>,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + 'static,
    P: TransactionPool<Block = B> + 'static,
    C::Api: EthinkAPI<B>,
{
    pub async fn send_raw_transaction(&self, bytes: Bytes) -> RpcResult<H256> {
        let hash = self.client.info().best_hash;

        let slice = &bytes.0[..];
        if slice.is_empty() {
            return Err(rpc_err!("transaction data is empty"));
        }

        let tx: EthTransaction = ethereum::EnvelopedDecodable::decode(slice)
            .map_err(|_| rpc_err!("decode transaction failed"))?;

        self.compose_extrinsic_and_submit(hash, tx).await
    }

    /// Signs and submits a tx.
    /// Signing is performed with the key from the node's keystore, if there is a key for the sender account.
    /// If not, raises an error.
    pub async fn send_transaction(&self, request: TransactionRequest) -> RpcResult<H256> {
        let hash = self.client.info().best_hash;

        let TransactionRequest { from, .. } = request.clone();
        let from: AccountId20 = from
            .ok_or(rpc_err!("no origin account provided for tx"))?
            .into();
        let msg = TxMessage::from(request).0;

        // Lookup keystore for a proper key for signing
        let signer = EthereumSigner::try_from((self.keystore.clone(), from)).map_err(rpc_err)?;
        // and sign the transaction
        let signature = signer.try_sign(msg.clone()).map_err(rpc_err)?;

        // Compose Ethereum transaction
        let LegacyTransactionMessage {
            nonce,
            gas_price,
            gas_limit,
            action,
            value,
            input,
            ..
        } = msg;

        let tx: EthTransaction = LegacyTransaction {
            signature,
            nonce,
            gas_price,
            gas_limit,
            action,
            value,
            input,
        }
        .into();

        self.compose_extrinsic_and_submit(hash, tx).await
    }

    pub async fn call(
        &self,
        request: CallRequest,
        _number: Option<BlockNumber>,
        _state_overrides: Option<BTreeMap<H160, CallStateOverride>>,
    ) -> RpcResult<Bytes> {
        let hash = self.client.info().best_hash;

        let CallRequest {
            from,
            to,
            value,
            data,
            gas,
            ..
        } = request;

        self.client
            .runtime_api()
            .call(
                hash,
                from.ok_or(rpc_err!("empty `from` in call rq"))?,
                to.ok_or(rpc_err!("empty `to` in call rq"))?,
                data.unwrap_or_default().0, // No data defaults to vec![]
                value.unwrap_or(0.into()),  // No value defaults to 0
                gas.unwrap_or(0.into()), // No gas_limit defaults 0 (TODO could be changed to MAX (no limit))
            )
            .map_err(|err| rpc_err!("execution fatal: {:?}", err))?
            .map_err(|err| rpc_err!("runtime error on eth_call(): {:?}", err))
            .map(From::from)
    }

    // for this we do same as for call() but return consumed gas val
    // we encode sp_weights::Weight, which is 64*2 bytes length, into U256 value
    pub async fn estimate_gas(
        &self,
        request: CallRequest,
        _number: Option<BlockNumber>,
    ) -> RpcResult<U256> {
        let hash = self.client.info().best_hash;

        let CallRequest {
            from,
            to,
            value,
            data,
            ..
        } = request;

        self.client
            .runtime_api()
            .gas_estimate(
                hash,
                from.ok_or(rpc_err!("empty `from` in call rq"))?,
                to.ok_or(rpc_err!("empty `to` in call rq"))?,
                data.unwrap_or_default().0,
                value.unwrap_or(0.into()),
                U256::MAX,
            )
            .map_err(|err| rpc_err!("execution fatal: {:?}", err))?
            .map_err(|err| rpc_err!("runtime error on eth_call(): {:?}", err))
    }
}
