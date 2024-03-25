use super::*;
use crate::{signer::EthereumSigner, CallRequest};
use ep_crypto::AccountId20;
use ethereum::{LegacyTransaction, LegacyTransactionMessage};
use futures::future::TryFutureExt;
use sp_runtime::{generic::BlockId, transaction_validity::TransactionSource};

impl<B, C, P> EthRPC<B, C, P>
where
    B: BlockT,
    C: ProvideRuntimeApi<B> + HeaderBackend<B> + 'static,
    P: TransactionPool<Block = B> + 'static,
    C::Api: ETHRuntimeRPC<B>,
{
    pub async fn send_raw_transaction(&self, bytes: Bytes) -> RpcResult<H256> {
        let hash = self.client.info().best_hash;
        // TODO refactor
        let slice = &bytes.0[..];
        if slice.is_empty() {
            return Err(internal_err("transaction data is empty"));
        }

        log::debug!(target: "ethink:rpc", "eth_sendRawTx encoded: 0x{}", hex::encode(&slice));

        let tx: EthTx = match ethereum::EnvelopedDecodable::decode(slice) {
            Ok(tx) => tx,
            Err(_) => return Err(internal_err("decode transaction failed")),
        };

        log::debug!(target: "ethink:rpc", "eth_sendRawTx decoded: {:#?}", &tx);

        // TODO: DRY (this is used in several places)
        let tx_hash = tx.hash();
        // Compose extrinsic for submission
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

    /// Signs and submits a tx.
    /// Signigning is performed with the key from the node's keystorage, if there is a key for the sender account.
    /// If not, raises an error.
    pub async fn send_transaction(&self, request: TransactionRequest) -> RpcResult<H256> {
        let hash = self.client.info().best_hash;
        let TransactionRequest { from, .. } = request.clone();
        let from: AccountId20 = from
            .ok_or(internal_err("no origin account provided for tx"))?
            .into();
        let msg = TxMessage::from(request).0;

        // Lookup keystore for a proper key for signing
        let signer =
            EthereumSigner::try_from((self.keystore.clone(), from)).map_err(internal_err)?;
        // and sign the transaction
        let signature = signer.try_sign(msg.clone()).map_err(internal_err)?;

        // TODO refactor via From<(msg,sig)>
        let LegacyTransactionMessage {
            nonce,
            gas_price,
            gas_limit,
            action,
            value,
            input,
            ..
        } = msg;

        let tx: EthTx = LegacyTransaction {
            // TODO put to sg calc step above
            signature,
            nonce,
            gas_price,
            gas_limit,
            action,
            value,
            input,
        }
        .into();

        log::debug!(target: "ethink:rpc", "eth_sendTx: {:#?}", &tx);

        // TODO: DRY
        let tx_hash = tx.hash();
        // Compose extrinsic for submission
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
            ..
        } = request;

        log::debug!(target: "ethink:rpc", "call(): from: {:?} to: {:?} value: {:02x?} data: {:02x?}", &from, &to, &value, &data);

        let result = self
            .client
            .runtime_api()
            .call(
                hash,
                from.unwrap(),
                to.unwrap(),
                data.unwrap_or_default().0,
                value.unwrap_or(0.into()),
                U256::MAX,
            )
            .map_err(|err| internal_err(format!("execution fatal: {:?}", err)))?
            .map_err(|err| internal_err(format!("runtime error on call: {:?}", err)))?;

        Ok(result.into())
    }

    pub async fn estimate_gas(
        &self,
        _request: CallRequest,
        _number: Option<BlockNumber>,
    ) -> RpcResult<U256> {
        Ok(1000u32.into())
    }
}
