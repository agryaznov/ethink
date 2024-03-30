//! Means for signing an Ethereum transaction
use ethereum::{LegacyTransactionMessage, TransactionSignature};

// Substrate
use sp_core::ecdsa;
use sp_keystore::KeystorePtr;

use crate::ETHINK_KEYTYPE_ID;
use ep_crypto::{AccountId20, EthereumSignature};

// TODO move to ep_crypto
/// ETH transaction signer, comprised of account_id and ref to a keystore holding
/// secret key for that account_id.
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
