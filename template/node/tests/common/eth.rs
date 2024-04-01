use crate::{
    ecdsa, Bytes, EthTransaction, EthereumSignature, LegacyTransaction, LegacyTransactionMessage,
    SubstrateWeight, TransactionSignature, U256,
};

#[derive(Clone)]
pub struct EthTxInput {
    nonce: U256,
    gas_price: U256,
    gas_limit: U256,
    action: ethereum::TransactionAction,
    value: U256,
    input: Vec<u8>,
    chain_id: Option<u64>,
    signer: ecdsa::Pair,
}

impl EthTxInput {
    pub fn new(
        nonce: u64,
        gas_price: u64,
        gas_limit: SubstrateWeight,
        action: ethereum::TransactionAction,
        value: u64,
        input: Vec<u8>,
        chain_id: Option<u64>,
        signer: ecdsa::Pair,
    ) -> Self {
        let nonce = nonce.into();
        let gas_price = gas_price.into();
        let gas_limit: U256 = gas_limit.into();
        let value = value.into();

        Self {
            nonce,
            gas_price,
            gas_limit,
            action,
            value,
            input,
            chain_id,
            signer,
        }
    }
}

impl From<EthTxInput> for LegacyTransactionMessage {
    fn from(v: EthTxInput) -> Self {
        Self {
            nonce: v.nonce,
            gas_price: v.gas_price,
            gas_limit: v.gas_limit,
            action: v.action,
            value: v.value,
            input: v.input,
            chain_id: v.chain_id,
        }
    }
}

pub fn compose_and_sign_eth_tx(i: EthTxInput) -> EthTransaction {
    let msg: LegacyTransactionMessage = i.clone().into();
    let sig = EthereumSignature::new(i.signer.sign_prehashed(&msg.hash().into()));
    let sig: Option<TransactionSignature> = sig.into();
    let signature = sig.expect("signer generated no signature");

    EthTransaction::Legacy(LegacyTransaction {
        nonce: i.nonce,
        gas_price: i.gas_price,
        gas_limit: i.gas_limit,
        action: i.action,
        value: i.value,
        input: i.input,
        signature,
    })
}
