#![allow(non_snake_case)]
//! Intergraion tests for ethink!
mod common;

const NODE_BIN: &'static str = env!("CARGO_BIN_EXE_ethink-node");
const ALITH_KEY: &'static str = env!("ALITH_KEY");
const ALITH_ADDRESS: &'static str = env!("ALITH_ADDRESS");
const FLIPPER_PATH: &'static str = env!("FLIPPER_PATH");

#[tokio::test]
async fn eth_call() {
    use common::*;
    use node_proc::*;
    use std::process;
    // spawn node
    let node_proc = TestNodeProcess::<PolkadotConfig>::build(NODE_BIN)
        .spawn()
        .await
        .unwrap_or_else(|err| ::core::panic!("Error spawning ethink-node: {:?}", err));

    // deploy contract
    // (flipped to false)
    let surl_arg = format!("-s={ALITH_KEY}");
    let manifest_arg = format!("--manifest-path={FLIPPER_PATH}");
    let url_arg = format!("--url={}", node_proc.url(Protocol::WS));
    let output = process::Command::new("cargo")
        .arg("contract")
        .arg("instantiate")
        .arg(&surl_arg)
        .arg("--args=false")
        .arg("-x")
        .arg("--skip-confirm")
        .arg("--output-json")
        .arg(&manifest_arg)
        .arg(&url_arg)
        .output()
        .expect("failed to instantiate with cargo-contract");

    assert!(output.status.success());
    let contract_address = find_address_from_instantiate(&output.stdout);

    // make ETH RPC request
    // (flip to true)
    let res = ureq::post(node_proc.url(Protocol::HTTP).as_str()).send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [{
                           "from": ALITH_ADDRESS,
                           "to": contract_address,
                           "value": "0x00",
                           "data": "0x2f865bd9"
                       },
                       "latest"],
            "id": 1}));

    use serde_json::{Deserializer, Value};

    let obj = Deserializer::from_reader(res.expect("ETH RPC request failed").into_reader())
        .into_iter::<Value>()
        .next()
        .expect("blank json output")
        .expect("can't decode json output");

    // TODO construct these magic return values explicitly here
    assert_eq!(obj["result"].as_str().unwrap(), "0x00000000080000");

    // TODO change state and re-test
}

#[tokio::test]
async fn eth_sendRawTransaction() {
    use common::*;
    use node_proc::*;
    use std::process;
    // spawn node
    let node_proc = TestNodeProcess::<PolkadotConfig>::build(NODE_BIN)
        .spawn()
        .await
        .unwrap_or_else(|err| ::core::panic!("Error spawning ethink-node: {:?}", err));

    // deploy contract
    // (flipped to false)
    let surl_arg = format!("-s={ALITH_KEY}");
    let manifest_arg = format!("--manifest-path={FLIPPER_PATH}");
    let url_arg = format!("--url={}", node_proc.url(Protocol::WS));
    let output = process::Command::new("cargo")
        .arg("contract")
        .arg("instantiate")
        .arg(&surl_arg)
        .arg("--args=false")
        .arg("-x")
        .arg("--skip-confirm")
        .arg("--output-json")
        .arg(&manifest_arg)
        .arg(&url_arg)
        .output()
        .expect("failed to instantiate with cargo-contract");

    assert!(output.status.success());
    let contract_address = find_address_from_instantiate(&output.stdout);

    // make ETH RPC request
    // (flip to true)
    let res = ureq::post(node_proc.url(Protocol::HTTP).as_str()).send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "eth_sendRawTransaction",
            "params": ["0xf86508808405f5e10094ac7da28b0a6e94dec4c9d2bfa6917ff476e6a9448084cde4efa978a09fda452d7a17d1a7cc98cf88343394f02627d079ef881fc36fc1361769c15a07a0112514d3a2e44ed85fc8c632e044239a17e83db41a99f253d63b3281aa3dd5ab"],
            "id": 1}));

    assert!(res.is_ok());

    use serde_json::{Deserializer, Value};

    let obj = Deserializer::from_reader(res.expect("ETH RPC request failed").into_reader())
        .into_iter::<Value>()
        .next()
        .expect("blank json output")
        .expect("can't decode json output");

    let _tx_hash = obj["result"]
        .as_str()
        .expect("tx_hash should have been returned");

    use futures::StreamExt;

    // Wait until tx gets executed
    let mut blocks_sub = node_proc
        .client()
        .blocks()
        .subscribe_finalized()
        .await
        .expect("can't subscribe to finalized blocks")
        .take(3);

    while let Some(block) = blocks_sub.next().await {
        let block = block.expect("can't get next finalized block");
        let events = block.events().await.expect("can't get events from block");

        if let Some(_) = events.iter().find(|e| {
            e.as_ref()
                .expect("failed to read event")
                .variant_name()
                .eq("EthTxExecuted")
        }) {
            break;
        }
    }

    // check state
    let contract_arg = format!("--contract={contract_address}");
    let output = process::Command::new("cargo")
        .arg("contract")
        .arg("call")
        .arg(&surl_arg)
        .arg(&contract_arg)
        .arg("--message=get")
        .arg("--output-json")
        .arg(&manifest_arg)
        .arg(&url_arg)
        .output()
        .expect("failed to call with cargo-contract");

    assert!(output.status.success());
    // (should be flipped to true)
    assert!(find_bool_value_from_call(&output.stdout));
}
