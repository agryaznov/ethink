// SPDX-License-Identifier: Apache-2.0
// This file is part of Ethink.
//
// Copyright (c) 2023-2024 Alexander Gryaznov.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Ethereum signing facilities
use ethereum::TransactionSignature;
use ethereum_types::{H160, H256};
use scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

// Substrate
use sp_core::{ecdsa, RuntimeDebug};
use sp_io::hashing::keccak_256;
use sp_runtime_interface::pass_by::PassByInner;

use super::account::AccountId20;

#[derive(Eq, PartialEq, Clone, Encode, Decode, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EthereumSignature(pub ecdsa::Signature);

impl From<sp_core::ecdsa::Signature> for EthereumSignature {
    fn from(s: sp_core::ecdsa::Signature) -> Self {
        Self(s)
    }
}

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

    fn to_vrs(&self, chain_id: Option<u64>) -> (u64, H256, H256) {
        // Some Ethereum-specific signature magic
        let v = match chain_id {
            None => 27,
            Some(chain_id) => 2 * chain_id + 35,
        } + self.0 .0[64] as u64;
        let r = H256::from_slice(&self.0 .0[0..32]);
        let s = H256::from_slice(&self.0 .0[32..64]);

        (v, r, s)
    }
}

impl From<EthereumSignature> for Option<TransactionSignature> {
    fn from(s: EthereumSignature) -> Self {
        let (v, r, s) = s.to_vrs(None);

        TransactionSignature::new(v, r, s)
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
