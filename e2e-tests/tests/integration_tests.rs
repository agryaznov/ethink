//! Intergraion tests for ethink!

use subxt;
use e2e_tests::node_proc;

/// Default set of commonly used types by Substrate runtimes.
pub enum SubstrateConfig {}

// TODO
impl subxt::Config for SubstrateConfig {
    type Index = u32;
    type Hash = sp_core::H256;
    type Hasher = subxt::config::substrate::BlakeTwo256;
    type AccountId = subxt::config::substrate::AccountId32;
    type Address = sp_runtime::MultiAddress<Self::AccountId, u32>;
    type Header = subxt::config::substrate::SubstrateHeader<
        u32,
        subxt::config::substrate::BlakeTwo256,
    >;
    type Signature = sp_runtime::MultiSignature;
    type ExtrinsicParams = subxt::config::substrate::SubstrateExtrinsicParams<Self>;
}

/// Default set of commonly used types by Polkadot nodes.
pub type PolkadotConfig = subxt::config::WithExtrinsicParams<
    SubstrateConfig,
    subxt::config::polkadot::PolkadotExtrinsicParams<SubstrateConfig>,
>;

#[test]
fn call_works() {
    // spawn node
    let _spawn = async {
        let _node_proc =
            node_proc::TestNodeProcess::<PolkadotConfig>::build("/target/debug/ethink-node")
                .spawn()
                .await
                .unwrap_or_else(|err| ::core::panic!("Error spawning ethink-node: {:?}", err));
    };

    // deploy contract

    // make request
    let res = ureq::post("http://localhost:9944/").send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [{
                           "from": "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac",
                           "to": "0xac7da28b0a6e94dec4c9d2bfa6917ff476e6a944",
                           "value": "0x00",
                            "data": "0x102f865bd9"
                       },
                       "latest"],
        "id": 2}));

    assert!(res.is_ok());

    // check state
}
