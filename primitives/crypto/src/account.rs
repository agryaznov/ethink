//! Ethereum account primitives
use scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{ecdsa, H160, H256};
use sp_io::hashing::keccak_256;

/// A fully Ethereum-compatible `AccountId`.
/// Conforms to H160 address and ECDSA key standards.
/// Alternative to H256->H160 mapping.
#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default, Encode, Decode, MaxEncodedLen, TypeInfo,
)]
pub struct AccountId20(pub [u8; 20]);

impl_serde::impl_fixed_hash_serde!(AccountId20, 20);

impl AsRef<[u8]> for AccountId20 {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsRef<[u8; 20]> for AccountId20 {
    fn as_ref(&self) -> &[u8; 20] {
        &self.0
    }
}

#[cfg(feature = "std")]
impl std::str::FromStr for AccountId20 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        H160::from_str(s)
            .map(Into::into)
            .map_err(|_| "invalid hex address.")
    }
}

#[cfg(feature = "std")]
impl std::fmt::Display for AccountId20 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let address = hex::encode(self.0).trim_start_matches("0x").to_lowercase();
        let address_hash = hex::encode(keccak_256(address.as_bytes()));

        let checksum: String =
            address
                .char_indices()
                .fold(String::from("0x"), |mut acc, (index, address_char)| {
                    let n = u16::from_str_radix(&address_hash[index..index + 1], 16)
                        .expect("Keccak256 hashed; qed");

                    if n > 7 {
                        // make char uppercase if ith character is 9..f
                        acc.push_str(&address_char.to_uppercase().to_string())
                    } else {
                        // already lowercased
                        acc.push(address_char)
                    }

                    acc
                });
        write!(f, "{checksum}")
    }
}

impl sp_std::fmt::Debug for AccountId20 {
    fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
        write!(f, "{:?}", H160(self.0))
    }
}

impl From<[u8; 20]> for AccountId20 {
    fn from(bytes: [u8; 20]) -> Self {
        Self(bytes)
    }
}

impl From<AccountId20> for [u8; 20] {
    fn from(val: AccountId20) -> Self {
        val.0
    }
}

impl From<H160> for AccountId20 {
    fn from(h160: H160) -> Self {
        Self(h160.0)
    }
}

impl From<AccountId20> for H160 {
    fn from(val: AccountId20) -> Self {
        H160(val.0)
    }
}

impl From<ecdsa::Public> for AccountId20 {
    fn from(pk: ecdsa::Public) -> Self {
        let decompressed = libsecp256k1::PublicKey::parse_compressed(&pk.0)
            .expect("Wrong compressed public key provided")
            .serialize();
        let mut m = [0u8; 64];
        m.copy_from_slice(&decompressed[1..65]);
        let account = H160::from(H256::from(keccak_256(&m)));
        Self(account.into())
    }
}

#[cfg(test)]
mod tests {
    use super::{account::*, signing::*};
    use sp_core::{ecdsa, Pair, H160, H256};
    use sp_io::hashing::keccak_256;
    use sp_runtime::traits::IdentifyAccount;

    #[test]
    fn derive_from_secret_key() {
        let sk = hex::decode("eb3d6b0b0c794f6fd8964b4a28df99d4baa5f9c8d33603c4cc62504daa259358")
            .unwrap();
        let hex_acc: [u8; 20] = hex::decode("98fa2838ee6471ae87135880f870a785318e6787")
            .unwrap()
            .try_into()
            .unwrap();
        let acc = AccountId20::from(hex_acc);

        let pk = ecdsa::Pair::from_seed_slice(&sk).unwrap().public();
        let signer: EthereumSigner = pk.into();

        assert_eq!(signer.into_account(), acc);
    }

    #[test]
    fn from_h160() {
        let m = hex::decode("28490327ff4e60d44b8aadf5478266422ed01232cc712c2d617e5c650ca15b85")
            .unwrap();
        let old: AccountId20 = H160::from(H256::from(keccak_256(&m))).into();
        let new: AccountId20 = H160::from_slice(&keccak_256(&m)[12..32]).into();
        assert_eq!(new, old);
    }

    #[test]
    fn display() {
        let pk = ecdsa::Pair::from_string("//Alice", None)
            .expect("static values are valid; qed")
            .public();
        let signer: EthereumSigner = pk.into();
        let account: AccountId20 = signer.into_account();
        let account_fmt = format!("{}", account);
        assert_eq!(account_fmt, "0xE04CC55ebEE1cBCE552f250e85c57B70B2E2625b");
    }
}
