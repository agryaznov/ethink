//! Prelude actions needed in most of the tests
use crate::common::{consts::*, node::*, *};
use serde_json::Deserializer;
use std::str::FromStr;

/// Spawn a node and deploy a contract to it
pub async fn node_and_contract<R: subxt::Config>(
    manifest_path: &str,
    constructor_args: Vec<&str>,
    signer: Option<&str>,
) -> Env<R> {
    let mut builder = TestNodeProcess::<R>::build(NODE_BIN);
    let node = if let Some(key) = signer {
        builder.with_signer(key)
    } else {
        &mut builder
    }
    .spawn()
    .await
    .unwrap_or_else(|err| ::core::panic!("Error spawning ethink-node: {:?}", err));

    // deploy contract
    let output = contracts::deploy(
        node.url(Protocol::WS).as_str(),
        manifest_path,
        constructor_args,
        signer,
    );
    // Look for contract address in the json output
    let rs = Deserializer::from_slice(&output.stdout);
    let contract_address = json_get!(rs["contract"])
        .as_str()
        .map(AccountId20::from_str)
        .expect("contract address not returned")
        .expect("can't decode contract address");

    Env::new(node, manifest_path.to_string(), contract_address)
}
