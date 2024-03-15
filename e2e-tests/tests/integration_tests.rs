//! Intergraion tests for ethink!

use e2e_tests::{node_proc::*, PolkadotConfig};

#[tokio::test]
async fn call_works() {
    // spawn node
    let node_proc = TestNodeProcess::<PolkadotConfig>::build(
        "/home/me/dev/polka/ethink/target/debug/ethink-node",
    )
    .spawn()
    .await
    .unwrap_or_else(|err| ::core::panic!("Error spawning ethink-node: {:?}", err));

    // deploy contract

    // make request
    let res = ureq::post(node_proc.url(Protocol::HTTP).as_str()).send_json(ureq::json!({
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
