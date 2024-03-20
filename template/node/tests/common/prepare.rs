//! Prelude actions needed in most of the tests
use super::{node::*, *};
use serde_json::Deserializer;

/// Spawn a node and deploy a contract to it
pub async fn node_and_contract<R: subxt::Config>(contract_manifest_path: &str) -> Env<R> {
    let node = TestNodeProcess::<R>::build(NODE_BIN)
        .spawn()
        .await
        .unwrap_or_else(|err| ::core::panic!("Error spawning ethink-node: {:?}", err));

    // deploy contract
    let output = contracts::deploy(contract_manifest_path, node.url(Protocol::WS).as_str());

    assert!(output.status.success());
    // Look for contract address in the json output
    let res = Deserializer::from_slice(&output.stdout);
    let contract_address = json_get!(res["contract"].as_str()).to_string();

    Env {
        node,
        contract_address,
    }
}
