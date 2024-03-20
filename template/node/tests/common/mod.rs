pub mod node;
#[macro_use]
pub mod macros;
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

    /// Wait until a specified event is emitted in a finalized block,
    /// but no longer than `timeout` number of blocks.
    pub async fn wait_for_event(&mut self, fullname: &str, timeout: usize) {
        use futures::StreamExt;

        if let Some((pallet, variant)) = fullname.rsplit_once(".") {
            let mut blocks_sub = &mut self
                .node
                .client()
                .blocks()
                .subscribe_finalized()
                .await
                .expect("can't subscribe to finalized blocks")
                .take(timeout);

            while let Some(block) = blocks_sub.next().await {
                let block = block.expect("can't get next finalized block");
                let events = block.events().await.expect("can't get events from block");

                if let Some(_) = events.iter().find(|e| {
                    let event = e.as_ref().expect("failed to read event");

                    event.pallet_name().eq(pallet) && event.variant_name().eq(variant)
                }) {
                    break;
                }
            }
        }
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
