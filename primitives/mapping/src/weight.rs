use sp_core::U256;
use sp_weights::Weight;

/// Substrate Weight, convertible to U256
pub struct SubstrateWeight(Weight);

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
