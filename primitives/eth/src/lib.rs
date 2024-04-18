use ep_crypto::EthereumSignature;
use ep_mapping::{SubstrateWeight, Weight};
use sp_core::{U256, ecdsa, Pair};

pub use ethereum::{
    TransactionV2 as EthTransaction,
    LegacyTransaction,
    LegacyTransactionMessage,
};

// TODO: move ethereum-types re-exports here

#[derive(Clone)]
pub struct ContractInput(Vec<u8>);

impl From<Vec<u8>> for ContractInput {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl Into<Vec<u8>> for ContractInput {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

#[derive(Clone)]
/// Ethereum transaction input, used for transaciton building in tests
pub struct EthTxInput {
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: SubstrateWeight,
    pub action: ethereum::TransactionAction,
    pub value: u64,
    pub data: ContractInput,
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
        let gas_limit: U256 = v.gas_limit.into();
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
