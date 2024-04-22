//! We keep custom types here and not in rpc-core,
//! for we might later switch to fc-rpc-core.
use ep_eth::{AccountId20, EthereumSignature, LegacyTransactionMessage, TransactionSignature};

// Substrate
use sp_core::ecdsa;
use sp_keystore::KeystorePtr;
use sp_runtime::traits::{Block as BlockT, Header, UniqueSaturatedInto};

use crate::ETHINK_KEYTYPE_ID;
use ethereum_types::{H256, U256};
use ethink_rpc_core::types::Header as EthHeader;

pub use ethink_rpc_core::types::Block as EthereumBlock;

/// Substrate block, convertible to Ethereum block
pub struct SubstrateBlock<B>(pub B);

impl<B: BlockT<Hash = H256>> From<SubstrateBlock<B>> for EthereumBlock {
    // Generate dumb ETH block with empty tx list,
    // from the given substrate block
    fn from(b: SubstrateBlock<B>) -> Self {
        let h = b.0.header();

        let header = EthHeader {
            hash: (h.hash()).into(),
            parent_hash: (*h.parent_hash()).into(),
            state_root: (*h.state_root()).into(),
            transactions_root: (*h.extrinsics_root()).into(),
            number: Some(U256::from(
                UniqueSaturatedInto::<u128>::unique_saturated_into(*h.number()),
            )),
            ..Default::default()
        };

        Self {
            header,
            ..Default::default()
        }
    }
}

/// Ethereum transaction signer with keypair stored in node's keystore
pub struct EthereumSigner {
    keystore: KeystorePtr,
    pub_key: ecdsa::Public,
}
impl EthereumSigner {
    pub fn try_sign(&self, msg: LegacyTransactionMessage) -> Result<TransactionSignature, String> {
        let sig = self
            .keystore
            .ecdsa_sign_prehashed(
                ETHINK_KEYTYPE_ID,
                &self.pub_key,
                msg.hash().as_fixed_bytes(),
            )
            .transpose()
            .expect("we checked that keystore contains needed secret upon signer construction; qed")
            .map_err(|_| "Failed to sign tx".to_string())?;

        let sig: Option<TransactionSignature> = EthereumSignature::new(sig).into();

        sig.ok_or("signer generated invalid signature".to_string())
    }
}

impl TryFrom<(KeystorePtr, AccountId20)> for EthereumSigner {
    type Error = String;

    fn try_from(val: (KeystorePtr, AccountId20)) -> Result<Self, Self::Error> {
        let keystore = val.0;
        let pub_key = *keystore
            .ecdsa_public_keys(ETHINK_KEYTYPE_ID)
            .iter()
            .find(|&pk| AccountId20::from(pk.clone()) == val.1)
            .ok_or("No key for signer in keystore".to_string())?;

        Ok(EthereumSigner { keystore, pub_key })
    }
}

impl TryFrom<(KeystorePtr, ecdsa::Public)> for EthereumSigner {
    type Error = String;

    fn try_from(val: (KeystorePtr, ecdsa::Public)) -> Result<Self, Self::Error> {
        let keystore = val.0;
        let pub_key = *keystore
            .ecdsa_public_keys(ETHINK_KEYTYPE_ID)
            .iter()
            .find(|&pk| *pk == val.1)
            .ok_or("No key for signer in keystore")?;

        Ok(EthereumSigner { keystore, pub_key })
    }
}
