pub mod node;
#[macro_use]
pub mod macros;
pub mod contracts;
pub mod eth;
pub mod prepare;

pub const NODE_BIN: &'static str = env!("CARGO_BIN_EXE_ethink-node");
pub const FLIPPER_PATH: &'static str = env!("FLIPPER_PATH");
pub const ERC20_PATH: &'static str = env!("ERC20_PATH");
pub const ALITH_ADDRESS: &'static str = env!("ALITH_ADDRESS");
pub const ALITH_KEY: &'static str = env!("ALITH_KEY");
pub const BALTATHAR_ADDRESS: &'static str = env!("BALTATHAR_ADDRESS");
pub const BALTATHAR_KEY: &'static str = env!("BALTATHAR_KEY");

use crate::AccountId20;
use node::{Protocol, TestNodeProcess};

struct Contract {
    pub manifest_path: String,
    pub address: AccountId20,
}

// Testing environment, consisting of a node with a deployed cont   qract
pub struct Env<R: subxt::Config> {
    pub node: TestNodeProcess<R>,
    contract: Contract,
}

impl<R: subxt::Config> Env<R> {
    pub fn new(node: TestNodeProcess<R>, manifest_path: String, address: AccountId20) -> Self {
        let contract = Contract {
            manifest_path,
            address,
        };

        Env { node, contract }
    }

    pub fn contract_address(&self) -> AccountId20 {
        self.contract.address
    }

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
            let blocks_sub = &mut self
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
