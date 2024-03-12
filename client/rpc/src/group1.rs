use super::*;
use crate::CallRequest;
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
        let tx: EthTx = match ethereum::EnvelopedDecodable::decode(slice) {
            Ok(tx) => tx,
            Err(_) => return Err(internal_err("decode transaction failed")),
        };
        log::debug!(target: "ethink:rpc", "SendRawTx REQUEST: {:?}", &tx);

        let tx_hash = tx.hash();
        // Compose extrinsic for submission
        let extrinsic = match self.client.runtime_api().convert_transaction(hash, tx) {
            Ok(extrinsic) => extrinsic,
            Err(_) => return Err(internal_err("cannot access runtime api")),
        };
        // Submit extrinsic to pool
        self.pool
            .submit_one(&BlockId::Hash(hash), TransactionSource::Local, extrinsic)
            .map_ok(move |_| tx_hash)
            .map_err(internal_err)
            .await
    }

    // TODO: dry-run implementation
    pub async fn call(
        &self,
        request: CallRequest,
        _number: Option<BlockNumber>,
        _state_overrides: Option<BTreeMap<H160, CallStateOverride>>,
    ) -> RpcResult<Bytes> {
        // ensure_signer(origin)?
        let hash = self.client.info().best_hash;

        let CallRequest {
            from,
            to,
            value,
            data,
            ..
        } = request;

        log::debug!(target: "ethink", "CALL: {:?} to {:?}! data: {:?}", &value, &to, &data);

        // TODO this is currently mocked with dbg output
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

    // TODO
    pub async fn send_transaction(&self, request: TransactionRequest) -> RpcResult<H256> {
        // let hash = self.client.info().best_hash;

        // let TransactionRequest {
        //     from, to, value, ..
        // } = request;

        // // need to make sure our runtime uses Eth signatures
        // let signer_addr = from;
        // let signature = ;

        // // For now we just sending some tokens
        // // In the future, the pallet_contracts call will be constructed here
        // let extrinsic = UncheckedExtrinsic::new_signed(
        // 	pallet_balances::Call::<Runtime>::transfer_allow_death { dest: to }.into(),
        //     signer_addr,
        //     signature,
        // 	);

        // submit tx to the TransactionPool, get tx_hash in response
        // self.pool
        // 	.submit_one(
        // 		&BlockId::Hash(block_hash),
        // 		TransactionSource::Local,
        // 		extrinsic,
        // 	)
        // 	.map_ok(move |_| transaction_hash)
        // 	.map_err(|err| internal_err(format::Geth::pool_error(err)))
        // 	.await

        Ok(H256::zero())
    }

    pub async fn estimate_gas(
        &self,
        _request: CallRequest,
        _number: Option<BlockNumber>,
    ) -> RpcResult<U256> {
        Ok(1000u32.into())
    }
}
