//! Ethereum signing facilities
use scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
// Substrate
use sp_core::{ecdsa, RuntimeDebug, H160, H256};
use sp_io::hashing::keccak_256;
use sp_runtime_interface::pass_by::PassByInner;

use super::account::AccountId20;

#[derive(Eq, PartialEq, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EthereumSignature(pub ecdsa::Signature);

impl sp_std::fmt::Debug for EthereumSignature {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
        write!(f, "{:02x?}", &self.0 .0)
    }
}

impl sp_runtime::traits::Verify for EthereumSignature {
    type Signer = EthereumSigner;
    fn verify<L: sp_runtime::traits::Lazy<[u8]>>(&self, mut msg: L, signer: &AccountId20) -> bool {
        let m = keccak_256(msg.get());
        match sp_io::crypto::secp256k1_ecdsa_recover(self.0.as_ref(), &m) {
            Ok(pubkey) => {
                let a = AccountId20(H160::from(H256::from(keccak_256(&pubkey))).0);
                let r = a == *signer;
                if !r {
                    log::error!(target: "runtime", "SIGNER ACCOUNT EXPECTED: {:?}", &signer);
                    log::error!(target: "runtime", "SIGNER ACCOUNT EXTRACTED: {:?}", &a);
                };
                r
            }
            Err(sp_io::EcdsaVerifyError::BadRS) => {
                log::error!(target: "runtime", "Error recovering: Incorrect value of R or S");
                false
            }
            Err(sp_io::EcdsaVerifyError::BadV) => {
                log::error!(target: "runtime", "Error recovering: Incorrect value of V");
                false
            }
            Err(sp_io::EcdsaVerifyError::BadSignature) => {
                log::error!(target: "runtime", "Error recovering: Invalid signature");
                false
            }
        }
    }
}

impl EthereumSignature {
    pub fn new(s: ecdsa::Signature) -> Self {
        EthereumSignature(s)
    }

    pub fn from_raw(d: [u8; 65]) -> Self {
        Self::new(ecdsa::Signature::from_raw(d))
    }

    // TODO remove, this is a dbg helper
    pub fn dummy() -> Self {
        let mut bytes = [0u8; 65];
        hex::decode_to_slice("8e8d7354591bd8010e62ee99027944049d48a62be08cad8c38252ea437310c744a937786126cf471758131b9f48df1e51303b965bbeb3ad28bcffe6eb96635e001",
             &mut bytes as &mut [u8]
        ).unwrap();
        let s = ecdsa::Signature::from_slice(&bytes).unwrap();
        EthereumSignature(s)
    }
}

// TODO this thing does not sign anything; hence needs renaming.
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    RuntimeDebug,
    Encode,
    Decode,
    MaxEncodedLen,
    TypeInfo,
    PassByInner,
)]
pub struct EthereumSigner([u8; 20]);

impl From<[u8; 20]> for EthereumSigner {
    fn from(x: [u8; 20]) -> Self {
        EthereumSigner(x)
    }
}

impl sp_runtime::traits::IdentifyAccount for EthereumSigner {
    type AccountId = AccountId20;
    fn into_account(self) -> AccountId20 {
        AccountId20(self.0)
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for EthereumSigner {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", H160::from(self.0))
    }
}

impl From<ecdsa::Public> for EthereumSigner {
    fn from(pk: ecdsa::Public) -> Self {
        let decompressed = libsecp256k1::PublicKey::parse_compressed(&pk.0)
            .expect("Wrong compressed public key provided")
            .serialize();
        let mut m = [0u8; 64];
        m.copy_from_slice(&decompressed[1..65]);
        let account = H160::from(H256::from(keccak_256(&m)));
        EthereumSigner(account.into())
    }
}
