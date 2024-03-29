//! Integraion tests for ethink!
#![allow(non_snake_case)]

mod common;

use common::*;
use ep_mapping::{SubstrateWeight, Weight};
use serde_json::{value::Serializer, Deserializer};
use sp_core::U256;
use sp_runtime::Serialize;
use ureq::json;

#[tokio::test]
async fn eth_sendRawTransaction() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> = prepare_node_and_contract!(FLIPPER_PATH);
    // (Flipper is deployed with `false` state)
    // Make ETH RPC request (to flip it to `true`)
    // TODO pass gas_limit
    let rs = rpc_rq!(env,
    {
      "jsonrpc": "2.0",
      "method": "eth_sendRawTransaction",
      "params": ["0xf86508808405f5e10094ac7da28b0a6e94dec4c9d2bfa6917ff476e6a944\
                  8084cde4efa978a09fda452d7a17d1a7cc98cf88343394f02627d079ef881f\
                  c36fc1361769c15a07a0112514d3a2e44ed85fc8c632e044239a17e83db41a\
                  99f253d63b3281aa3dd5ab"],  // TODO call data encoding logic
      "id": 0
     });
    // Handle response
    let json = to_json_val!(rs);
    ensure_no_err!(&json);
    let _tx_hash = extract_result!(&json);
    // Wait until tx gets executed
    let _ = &env.wait_for_event("ethink.EthTransactionExecuted", 3).await;
    // Check state
    let output = contracts::call(&env, "get", false);
    let rs = Deserializer::from_slice(&output.stdout);
    // Should be flipped to `true`
    assert!(json_get!(rs["data"]["Tuple"]["values"][0]["Bool"])
        .as_bool()
        .expect("can't parse contract output"));
}
#[tokio::test]
async fn eth_sendTransaction() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> = prepare_node_and_contract!(FLIPPER_PATH, BALTATHAR_KEY);
    // (Flipper is deployed with `false` state)
    // Make ETH RPC request (to flip it to `true`)
    let rs = rpc_rq!(env,
    {
      "jsonrpc": "2.0",
      "method": "eth_sendTransaction",
      "params": [{
                  "from": BALTATHAR_ADDRESS,
                  "to": &env.contract_address(),
                  "data": contracts::encode(FLIPPER_PATH, "flip"),
                  "gas": Into::<U256>::into(SubstrateWeight::from(Weight::MAX)).to_string()
                 },
                 "latest"],
      "id": 0
    });
    // Handle response
    let json = to_json_val!(rs);
    ensure_no_err!(&json);
    let _tx_hash = extract_result!(&json);
    // Wait until tx gets executed
    let _ = &env.wait_for_event("ethink.EthTransactionExecuted", 3).await;
    // Check state
    let output = contracts::call(&env, "get", false);
    let rs = Deserializer::from_slice(&output.stdout);
    // Should be flipped to `true`
    assert!(json_get!(rs["data"]["Tuple"]["values"][0]["Bool"])
        .as_bool()
        .expect("can't parse contract output"));
}

#[tokio::test]
async fn gas_limit_is_respected() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> = prepare_node_and_contract!(FLIPPER_PATH, BALTATHAR_KEY);
    // (Flipper is deployed with `false` state)
    // Make ETH RPC request (to flip it to `true`)
    // Insufficient gas_limit (0)
    let rs = rpc_rq!(env,
    {
      "jsonrpc": "2.0",
      "method": "eth_sendTransaction",
      "params": [{
                  "from": BALTATHAR_ADDRESS,
                  "to": &env.contract_address(),
                  "data": contracts::encode(FLIPPER_PATH, "flip")
                 },
                 "latest"],
      "id": 0
    });
    // Handle response
    let json = to_json_val!(rs);
    ensure_no_err!(&json);
    let _tx_hash = extract_result!(&json);
    // Wait until tx gets executed (or timeout)
    // TODO shuold there be emitted event on falure?
    let _ = &env.wait_for_event("ethink.EthTransactionExecuted", 3).await;
    // Check state
    let output = contracts::call(&env, "get", false);
    let rs = Deserializer::from_slice(&output.stdout);
    // Should stay flipped to `false` as tx should have failed with insuficcient gas
    assert!(!json_get!(rs["data"]["Tuple"]["values"][0]["Bool"])
        .as_bool()
        .expect("can't parse contract output"));
}

#[tokio::test]
async fn eth_call() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> = prepare_node_and_contract!(FLIPPER_PATH);
    // (Flipper is deployed with `false` state)
    // Make eth_call rpc request to get() flipper state
    let rq = json!({
       "jsonrpc": "2.0",
       "method": "eth_call",
       "params": [{
                      "from": ALITH_ADDRESS,
                      "to": &env.contract_address(),
                      "data": contracts::encode(FLIPPER_PATH, "get"),
                      "gas": Into::<U256>::into(SubstrateWeight::from(Weight::MAX)).to_string()
                  },
                  "latest"],
       "id": 0
    });
    let rs = rpc_rq!(env, rq);
    // Handle response
    let json = to_json_val!(rs);
    ensure_no_err!(&json);
    // Should return `false` as flipper state
    let result = extract_result!(&json);
    assert_eq!(*result, "0x00000000080000");
    // Flip it via contract call
    let _ = contracts::call(&env, "flip", true);
    // Wait until tx gets executed
    let _ = &env.wait_for_event("contracts.Called", 2).await;
    // Make eth_call rpc request again
    let rs = rpc_rq!(env, rq);
    let json = to_json_val!(rs);
    ensure_no_err!(&json);
    // Should now return `true` as flipper state
    let result = extract_result!(&json);
    assert_eq!(*result, "0x00000000080001");
}

#[tokio::test]
async fn eth_estimateGas() {
    // Spawn node and deploy contract
    let env: Env<PolkadotConfig> = prepare_node_and_contract!(FLIPPER_PATH);
    // Retrieve gas estimation via cargo-contract dry-run
    let output = contracts::call(&env, "flip", false);
    let rs = Deserializer::from_slice(&output.stdout);
    let gas_consumed = json_get!(rs["gas_consumed"])
        .as_object()
        .expect("contract address not returned")
        .to_owned();
    let weight = contracts::Weight::from(&gas_consumed);
    let json_val = sp_core::U256::from(weight).serialize(Serializer).unwrap();
    let weight_str_expected = json_val.as_str().unwrap();
    // Make ETH rpc request
    let rq = json!({
       "jsonrpc": "2.0",
       "method": "eth_estimateGas",
       "params": [{
                      "from": ALITH_ADDRESS,
                      "to": &env.contract_address(),
                      "data": contracts::encode(FLIPPER_PATH, "flip")
                  },
                  "latest"],
       "id": 0
    });
    let rs = rpc_rq!(env, rq);
    // Handle response
    let json = to_json_val!(rs);
    ensure_no_err!(&json);
    // Should return gas spent value equal to the retrieved above
    let weight_str_returned = extract_result!(&json);
    assert_eq!(weight_str_returned, &weight_str_expected);
}
