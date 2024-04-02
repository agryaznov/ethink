#![cfg_attr(not(feature = "std"), no_std)]

mod weight;

use sp_core::U256;
pub use sp_weights::Weight;
pub use weight::SubstrateWeight;
