use crate::{EthTransaction, EthereumSignature, LegacyTransaction, LegacyTransactionMessage};
use ep_mapping::{SubstrateWeight, Weight};
use serde::{Serialize, Serializer};
use sp_core::{ecdsa, Pair, U256};

#[derive(Clone)]
pub struct ContractInput(Vec<u8>);

impl ContractInput {
    pub fn new(b: Vec<u8>) -> Self {
        Self(b)
    }
}

impl From<Vec<u8>> for ContractInput {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl From<ContractInput> for Vec<u8> {
    fn from(s: ContractInput) -> Self {
        s.0
    }
}

impl Serialize for ContractInput {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        format!("0x{}", hex::encode(&self.0)).serialize(serializer)
    }
}

#[derive(Clone)]
/// Ethereum transaction input, used for transaciton building in tests
pub struct EthTxInput {
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: Weight,
    pub action: ethereum::TransactionAction,
    pub value: u128,
    pub data: ContractInput,
    pub chain_id: Option<u64>,
    pub signer: ecdsa::Pair,
}

impl Default for EthTxInput {
    fn default() -> Self {
        Self {
            nonce: 1u64,
            gas_price: 0u64,
            gas_limit: Weight::MAX,
            action: ethereum::TransactionAction::Call(Default::default()),
            value: 0u128,
            data: vec![0].into(),
            chain_id: None,
            signer: ecdsa::Pair::generate().0,
        }
    }
}

impl From<EthTxInput> for LegacyTransactionMessage {
    fn from(v: EthTxInput) -> Self {
        let nonce = v.nonce.into();
        let gas_price = v.gas_price.into();
        let gas_limit: U256 = v.gas_limit.ref_time().into();
        let value = v.value.into();

        Self {
            nonce,
            gas_price,
            gas_limit,
            action: v.action,
            value,
            input: v.data.into(),
            chain_id: v.chain_id,
        }
    }
}

/// Build Eth tx message, sign it and build an Eth transaction
pub fn compose_and_sign_tx(i: EthTxInput) -> EthTransaction {
    let msg: LegacyTransactionMessage = i.clone().into();
    let sig = EthereumSignature::new(i.signer.sign_prehashed(&msg.hash().into()));
    let sig: Option<ethereum::TransactionSignature> = sig.into();
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
