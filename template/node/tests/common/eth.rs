use crate::{
    ecdsa, EthTransaction, EthereumSignature, LegacyTransaction, LegacyTransactionMessage,
    SubstrateWeight, TransactionSignature, Weight, U256,
};

use sp_core::crypto::Pair;

#[derive(Clone)]
/// Ethereum transaction input, use for transaciton building in tests
pub struct EthTxInput {
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: SubstrateWeight,
    pub action: ethereum::TransactionAction,
    pub value: u64,
    pub data: Vec<u8>,
    pub chain_id: Option<u64>,
    pub signer: ecdsa::Pair,
}

impl Default for EthTxInput {
    fn default() -> Self {
        Self {
            nonce: 1u64,
            gas_price: 0u64,
            gas_limit: SubstrateWeight::from(Weight::MAX),
            action: ethereum::TransactionAction::Call(Default::default()),
            value: 0u64,
            data: vec![0],
            chain_id: None,
            signer: ecdsa::Pair::generate().0,
        }
    }
}

impl From<EthTxInput> for LegacyTransactionMessage {
    fn from(v: EthTxInput) -> Self {
        let nonce = v.nonce.into();
        let gas_price = v.gas_price.into();
        let gas_limit: U256 = v.gas_limit.into();
        let value = v.value.into();

        Self {
            nonce,
            gas_price,
            gas_limit,
            action: v.action,
            value,
            input: v.data,
            chain_id: v.chain_id,
        }
    }
}

/// Build Eth tx message, sign it and build an Eth transaction
pub fn compose_and_sign_tx(i: EthTxInput) -> EthTransaction {
    let msg: LegacyTransactionMessage = i.clone().into();
    let sig = EthereumSignature::new(i.signer.sign_prehashed(&msg.hash().into()));
    let sig: Option<TransactionSignature> = sig.into();
    let signature = sig.expect("signer generated no signature");

    EthTransaction::Legacy(LegacyTransaction {
        nonce: msg.nonce,
        gas_price: msg.gas_price,
        gas_limit: msg.gas_limit,
        action: msg.action,
        value: msg.value,
        input: msg.input,
        signature,
    })
}
