//! Stub, see tests/

pub mod node_proc;

// Default set of commonly used types by Substrate runtimes.
pub enum SubstrateConfig {}

// // TODO
impl subxt::Config for SubstrateConfig {
    type Index = u32;
    type Hash = sp_core::H256;
    type Hasher = subxt::config::substrate::BlakeTwo256;
    type AccountId = subxt::config::substrate::AccountId32;
    type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
    type Header =
        subxt::config::substrate::SubstrateHeader<u32, subxt::config::substrate::BlakeTwo256>;
    type Signature = sp_runtime::MultiSignature;
    type ExtrinsicParams = subxt::config::substrate::SubstrateExtrinsicParams<Self>;
}

/// Default set of commonly used types by Polkadot nodes.
pub type PolkadotConfig = subxt::config::WithExtrinsicParams<
    SubstrateConfig,
    subxt::config::polkadot::PolkadotExtrinsicParams<SubstrateConfig>,
>;
