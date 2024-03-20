//! Integraion tests for ethink!
#![allow(non_snake_case)]

mod common;

use serde_json::Deserializer;

#[tokio::test]
async fn eth_call() {
    use common::*;
    // spawn node and deploy contract
    let env: Env<PolkadotConfig> = prepare::node_and_contract(FLIPPER_PATH).await;
    // make ETH RPC request
    // (flip to true)
    let res = Deserializer::from_reader(
        ureq::post(env.http_url().as_str())
            .send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [{
                           "from": ALITH_ADDRESS,
                           "to": env.contract_address,
                           "value": "0x00",
                           "data": "0x2f865bd9"
                       },
                       "latest"],
            "id": 1}))
            .expect("ETH RPC request failed")
            .into_reader(),
    );
    // TODO construct these magic return values explicitly here
    assert_eq!(json_get!(res["result"].as_str()), "0x00000000080000");
    // TODO change state and re-test
}

#[tokio::test]
async fn eth_sendRawTransaction() {
    use common::*;
    // spawn node and deploy contract
    let mut env: Env<PolkadotConfig> = prepare::node_and_contract(FLIPPER_PATH).await;
    // make ETH RPC request
    // (flip to true)
    // TODO explicitly build this magic binary value
    let res = Deserializer::from_reader(
     ureq::post(env.http_url().as_str()).send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "eth_sendRawTransaction",
            "params": ["0xf86508808405f5e10094ac7da28b0a6e94dec4c9d2bfa6917ff476e6a9448084cde4efa978a09fda452d7a17d1a7cc98cf88343394f02627d079ef881fc36fc1361769c15a07a0112514d3a2e44ed85fc8c632e044239a17e83db41a99f253d63b3281aa3dd5ab"],
         "id": 1}))
            .expect("ETH RPC request failed").into_reader()
    );

    let _tx_hash = json_get!(res["result"].as_str());

    // Wait until tx gets executed
    let _ = &env.wait_for_event("EthTxExecuted", 3).await;

    // check state
    let output = contracts::call(
        FLIPPER_PATH,
        env.ws_url().as_str(),
        &env.contract_address,
        "get",
    );
    assert!(output.status.success());
    let res = Deserializer::from_slice(&output.stdout);
    // (should be flipped to true)
    assert!(json_get!(
        res["data"]["Tuple"]["values"][0]["Bool"].as_bool()
    ));
}
