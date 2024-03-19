//! Intergraion tests for ethink!
mod common;

const NODE_BIN: &'static str = env!("CARGO_BIN_EXE_ethink-node");
const ALITH_KEY: &'static str = env!("ALITH_KEY");
const ALITH_ADDRESS: &'static str = env!("ALITH_ADDRESS");
const FLIPPER_PATH: &'static str = env!("FLIPPER_PATH");

#[tokio::test]
async fn call_works() {
    use common::*;
    use node_proc::*;
    use std::process;

    // spawn node
    let node_proc = TestNodeProcess::<PolkadotConfig>::build(NODE_BIN)
        .spawn()
        .await
        .unwrap_or_else(|err| ::core::panic!("Error spawning ethink-node: {:?}", err));

    // deploy contract
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
        .arg(&manifest_arg)
        .arg(&url_arg)
        .output()
        .expect("failed to instantiate with cargo-contract");

    assert!(output.status.success());

    let contract_address = find_contract_address_from_output(output.stdout);

    panic!("contract_address: {contract_address}");

    // make ETH RPC request
    let res = ureq::post(node_proc.url(Protocol::HTTP).as_str()).send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [{
                           "from": ALITH_ADDRESS,
                           "to": contract_address,
                           "value": "0x00",
                           "data": "0x102f865bd9"
                       },
                       "latest"],
        "id": 2}));

    assert!(res.is_ok());

    // check state
    let contract_arg = format!("--contract={contract_address}");
    let output = process::Command::new("cargo")
        .arg("contract")
        .arg("call")
        .arg(&surl_arg)
        .arg(&contract_arg)
        .arg("--message=get")
        .arg(&manifest_arg)
        .arg(&url_arg)
        .output()
        .expect("failed to call with cargo-contract");

    assert!(output.status.success());
}
