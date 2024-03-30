use crate::{
    ecdsa, EthTransaction, EthereumSignature, LegacyTransaction, LegacyTransactionMessage,
    SubstrateWeight, TransactionSignature, U256,
};

pub fn compose_and_sign_eth_tx(
    nonce: u64,
    gas_price: u64,
    gas_limit: SubstrateWeight,
    action: ethereum::TransactionAction,
    value: u64,
    input: Vec<u8>,
    chain_id: Option<u64>,
    signer: ecdsa::Pair,
) -> EthTransaction {
    let nonce = nonce.into();
    let gas_price = gas_price.into();
    let gas_limit: U256 = gas_limit.into();
    let value = value.into();

    let msg = LegacyTransactionMessage {
        nonce,
        gas_price,
        gas_limit: gas_limit.clone(),
        action,
        value,
        input: input.clone(),
        chain_id,
    };

    let sig = EthereumSignature::new(signer.sign_prehashed(&msg.hash().into()));
    let sig: Option<TransactionSignature> = sig.into();
    let signature = sig.expect("signer generated no signature");

    EthTransaction::Legacy(LegacyTransaction {
        nonce,
        gas_price,
        gas_limit,
        action,
        value,
        input,
        signature,
    })
}
