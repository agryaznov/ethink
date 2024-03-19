//! Integraion tests for ethink!
#![allow(non_snake_case)]

mod common;

#[tokio::test]
async fn eth_call() {
    use common::*;
    // spawn node and deploy contract
    let env: Env<PolkadotConfig> = prepare::node_and_contract(FLIPPER_PATH).await;
    // make ETH RPC request
    // (flip to true)
    let res = ureq::post(env.http_url().as_str()).send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "eth_call",
            "params": [{
                           "from": ALITH_ADDRESS,
                           "to": env.contract_address,
                           "value": "0x00",
                           "data": "0x2f865bd9"
                       },
                       "latest"],
            "id": 1}));

    use serde_json::{Deserializer, Value};
    // TODO put to macro
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
    // spawn node and deploy contract
    let mut env: Env<PolkadotConfig> = prepare::node_and_contract(FLIPPER_PATH).await;
    // make ETH RPC request
    // (flip to true)
    // TODO explicitly build this magic binary value
    let res = ureq::post(env.http_url().as_str()).send_json(ureq::json!({
            "jsonrpc": "2.0",
            "method": "eth_sendRawTransaction",
            "params": ["0xf86508808405f5e10094ac7da28b0a6e94dec4c9d2bfa6917ff476e6a9448084cde4efa978a09fda452d7a17d1a7cc98cf88343394f02627d079ef881fc36fc1361769c15a07a0112514d3a2e44ed85fc8c632e044239a17e83db41a99f253d63b3281aa3dd5ab"],
            "id": 1}));

    assert!(res.is_ok());

    use serde_json::{Deserializer, Value};
    // TODO put to macro
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
    // TODO put to common
    let mut blocks_sub = &mut env
        .node_proc
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
    let output = contracts::call(
        FLIPPER_PATH,
        env.ws_url().as_str(),
        &env.contract_address,
        "get",
    );
    assert!(output.status.success());
    // (should be flipped to true)
    assert!(get!(&output.stdout, ["data"]["Tuple"]["values"][0]["Bool"])
        .as_bool()
        .expect("can't find bool in output data"));
}
