use sp_core::serde::{Serialize, Serializer};

use super::*;

/// Substrate Weight, convertible to EVM Gas
/// NOTE: as ethink runtime is configured to charge only for
/// the ref_time() part, of Weight, we just convert it to EVM Gas 1-to-1
#[derive(Clone)]
pub struct SubstrateWeight(pub Weight);

impl SubstrateWeight {
    pub fn max() -> Self {
        Self(Weight::MAX)
    }
}

// TODO BUG the following is an experimental hacking from the initial ethink version,
// which needs to get removed
impl From<U256> for SubstrateWeight {
    fn from(u: U256) -> Self {
        Weight::from_parts(u.0[0], u.0[1]).into()
    }
}

impl From<SubstrateWeight> for U256 {
    fn from(s: SubstrateWeight) -> Self {
        U256([s.0.ref_time(), s.0.proof_size(), 0, 0])
    }
}

impl From<Weight> for SubstrateWeight {
    fn from(w: Weight) -> Self {
        Self(w)
    }
}

impl From<SubstrateWeight> for Weight {
    fn from(s: SubstrateWeight) -> Self {
        s.0
    }
}
/// For serialization, we encode Weight as [U256],
/// so that the rpc returned value comply with Ethereum RPC
impl Serialize for SubstrateWeight {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let u = Into::<U256>::into(self.clone());

        u.serialize(serializer)
    }
}
