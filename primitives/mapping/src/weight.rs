use sp_core::serde::{Serialize, Serializer};

use super::*;

/// Substrate Weight, convertible to U256
#[derive(Clone)]
pub struct SubstrateWeight(Weight);

impl SubstrateWeight {
    pub fn max() -> Self {
        Self(Weight::MAX)
    }
}

impl From<U256> for SubstrateWeight {
    fn from(u: U256) -> Self {
        Weight::from_parts(u.0[0], u.0[1]).into()
    }
}

impl Into<U256> for SubstrateWeight {
    fn into(self) -> U256 {
        U256([self.0.ref_time(), self.0.proof_size(), 0, 0])
    }
}

impl From<Weight> for SubstrateWeight {
    fn from(w: Weight) -> Self {
        Self(w)
    }
}

impl Into<Weight> for SubstrateWeight {
    fn into(self) -> Weight {
        self.0
    }
}

impl Serialize for SubstrateWeight {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let u = Into::<U256>::into(self.clone());

        u.serialize(serializer)
    }
}
