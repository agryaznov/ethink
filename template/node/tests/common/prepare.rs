//! Prelude actions needed in most of the tests
use super::{node_proc::*, *};

/// Spawn a node and deploy a contract to it
pub async fn node_and_contract<R: subxt::Config>(contract_manifest_path: &str) -> Env<R> {
    let node_proc = TestNodeProcess::<R>::build(NODE_BIN)
        .spawn()
        .await
        .unwrap_or_else(|err| ::core::panic!("Error spawning ethink-node: {:?}", err));

    // deploy contract
    let output = contracts::deploy(contract_manifest_path, node_proc.url(Protocol::WS).as_str());

    assert!(output.status.success());
    // Look for contract address in the json output
    let contract_address = get!(&output.stdout, ["contract"])
        .as_str()
        .expect("can't decode contract address from the output")
        .to_string();

    Env {
        node_proc,
        contract_address,
    }
}
