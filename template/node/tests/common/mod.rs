pub mod node;
#[macro_use]
pub mod contracts;
pub mod prepare;

pub const ALITH_KEY: &'static str = env!("ALITH_KEY");
pub const NODE_BIN: &'static str = env!("CARGO_BIN_EXE_ethink-node");
pub const ALITH_ADDRESS: &'static str = env!("ALITH_ADDRESS");
pub const FLIPPER_PATH: &'static str = env!("FLIPPER_PATH");

use node::{Protocol, TestNodeProcess};

// Testing environment, consisting of a node with a deployed contract
pub struct Env<R: subxt::Config> {
    pub node: TestNodeProcess<R>,
    pub contract_address: String,
}

impl<R: subxt::Config> Env<R> {
    pub fn ws_url(&self) -> String {
        self.node.url(Protocol::WS)
    }

    pub fn http_url(&self) -> String {
        self.node.url(Protocol::HTTP)
    }
}
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
