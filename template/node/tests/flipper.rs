// SPDX-License-Identifier: Apache-2.0
//
// This file is part of Ethink.
//
// Copyright (c) 2023-2024 Alexander Gryaznov.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Integraion tests for ethink!
#![allow(non_snake_case)]

mod common;

use common::{consts::*, eth::EthTxInput, *};
use ep_crypto::{AccountId20, EthereumSignature};
use ep_mapping::{SubstrateWeight, Weight};
use ep_rpc::EthTransaction;
use ethereum::{
    EnvelopedEncodable, LegacyTransaction, LegacyTransactionMessage, TransactionSignature,
};
use serde_json::{value::Serializer, Deserializer};
use sp_core::{ecdsa, Pair, U256};
use sp_runtime::Serialize;
use ureq::json;

pub const FLIPPER_PATH: &'static str = env!("FLIPPER_PATH");

#[tokio::test]
async fn eth_sendRawTransaction() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> = prepare_node_and_contract!(FLIPPER_PATH, vec!["false"]);
    // (Flipper is deployed with `false` state)
    let input = EthTxInput {
        signer: ecdsa::Pair::from_string(ALITH_KEY, None).unwrap(),
        action: ethereum::TransactionAction::Call(env.contract_address().into()),
        data: encode!(FLIPPER_PATH, "flip"),
        ..Default::default()
    };
    let tx = eth::compose_and_sign_tx(input);
    let tx_hex = format!("0x{:x}", &tx.encode());
    // Make ETH RPC request (to switch flipper to `true`)
    let rs = rpc_rq!(env,
    {
      "jsonrpc": "2.0",
      "method": "eth_sendRawTransaction",
      "params": [ &tx_hex ],
      "id": 0
     });
    // Handle response
    let json = to_json_val!(rs);
    ensure_no_err!(&json);
    let _tx_hash = extract_result!(&json);
    // Wait until tx gets executed
    let _ = &env.wait_for_event("ethink.EthTransactionExecuted", 3).await;
    // Check state
    let output = call!(env, "get");
    let rs = Deserializer::from_slice(&output.stdout);
    // Should be flipped to `true`
    assert!(json_get!(rs["data"]["Tuple"]["values"][0]["Bool"])
        .as_bool()
        .expect("can't parse cargo contract output"));
}

#[tokio::test]
async fn eth_sendTransaction() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> =
        prepare_node_and_contract!(FLIPPER_PATH, vec!["false"], BALTATHAR_KEY);
    // (Flipper is deployed with `false` state)
    // Make ETH RPC request (to flip it to `true`)
    let rs = rpc_rq!(env,
    {
      "jsonrpc": "2.0",
      "method": "eth_sendTransaction",
      "params": [{
                  "from": BALTATHAR_ADDRESS,
                  "to": &env.contract_address(),
                  "data": encode!(FLIPPER_PATH, "flip"),
                  "gas": SubstrateWeight::max()
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
    let output = call!(env, "get");
    let rs = Deserializer::from_slice(&output.stdout);
    // Should be flipped to `true`
    assert!(json_get!(rs["data"]["Tuple"]["values"][0]["Bool"])
        .as_bool()
        .expect("can't parse cargo contract output"));
}

#[tokio::test]
async fn gas_limit_is_respected() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> =
        prepare_node_and_contract!(FLIPPER_PATH, vec!["false"], BALTATHAR_KEY);
    // (Flipper is deployed with `false` state)
    // Make ETH RPC request (to flip it to `true`)
    // Insufficient gas_limit (None => 0)
    let rs = rpc_rq!(env,
    {
      "jsonrpc": "2.0",
      "method": "eth_sendTransaction",
      "params": [{
                  "from": BALTATHAR_ADDRESS,
                  "to": &env.contract_address(),
                  "data": encode!(FLIPPER_PATH, "flip")
                 },
                 "latest"],
      "id": 0
    });
    // Handle response
    let json = to_json_val!(rs);
    ensure_no_err!(&json);
    let _tx_hash = extract_result!(&json);
    // Wait until tx fails (or timeout)
    let _ = &env.wait_for_event("system.ExtrinsicFailed", 2).await;
    // Check state
    let output = call!(env, "get");
    let rs = Deserializer::from_slice(&output.stdout);
    // Should stay flipped to `false` as tx should have failed with insuficcient gas
    assert!(!json_get!(rs["data"]["Tuple"]["values"][0]["Bool"])
        .as_bool()
        .expect("can't parse cargo contract output"));

    // Now let's set gas_limit to be half of the amount estimated
    // Retrieve gas estimation via cargo-contract dry-run
    let output = call!(env, "flip");
    let rs = Deserializer::from_slice(&output.stdout);
    let gas_consumed = json_get!(rs["gas_consumed"]).to_owned();
    let half_weight_consumed = serde_json::from_value::<Weight>(gas_consumed)
        .map(|x| x.div(2))
        .map(SubstrateWeight::from)
        .unwrap();
    // (Flipper is still at thdeployed with `false` state)
    // Make ETH RPC request (to flip it to `true`)
    // Insufficient gas_limit (half of estimated)
    let rs = rpc_rq!(env,
    {
      "jsonrpc": "2.0",
      "method": "eth_sendTransaction",
      "params": [{
                  "from": BALTATHAR_ADDRESS,
                  "to": &env.contract_address(),
                  "data": encode!(FLIPPER_PATH, "flip"),
                  "gas": half_weight_consumed,
                 },
                 "latest"],
      "id": 1
    });
    // Handle response
    let json = to_json_val!(rs);
    ensure_no_err!(&json);
    let _tx_hash = extract_result!(&json);
    // Wait until tx fails (or timeout)
    let _ = &env.wait_for_event("system.ExtrinsicFailed", 2).await;
    // Check state
    let output = call!(env, "get");
    let rs = Deserializer::from_slice(&output.stdout);
    // Should stay flipped to `false` as tx should have failed with insuficcient gas
    assert!(!json_get!(rs["data"]["Tuple"]["values"][0]["Bool"])
        .as_bool()
        .expect("can't parse cargo contract output"));
}

#[tokio::test]
async fn eth_call() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> = prepare_node_and_contract!(FLIPPER_PATH, vec!["false"]);
    // (Flipper is deployed with `false` state)
    // Make eth_call rpc request to get() flipper state
    let rq = json!({
       "jsonrpc": "2.0",
       "method": "eth_call",
       "params": [{
                      "from": ALITH_ADDRESS,
                      "to": &env.contract_address(),
                      "data": encode!(FLIPPER_PATH, "get"),
                      "gas": SubstrateWeight::max()
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
    let _ = call!(env, "flip", vec![], true);
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
    let env: Env<PolkadotConfig> = prepare_node_and_contract!(FLIPPER_PATH, vec!["false"]);
    // Retrieve gas estimation via cargo-contract dry-run
    let output = call!(env, "flip");
    let rs = Deserializer::from_slice(&output.stdout);
    let gas_consumed = json_get!(rs["gas_consumed"]).to_owned();
    let weight = serde_json::from_value::<Weight>(gas_consumed)
        .map(SubstrateWeight::from)
        .unwrap();
    let weight_str_expected = weight.serialize(Serializer).unwrap().to_owned();
    // Make ETH rpc request
    let rq = json!({
       "jsonrpc": "2.0",
       "method": "eth_estimateGas",
       "params": [{
                      "from": ALITH_ADDRESS,
                      "to": &env.contract_address(),
                      "data": encode!(FLIPPER_PATH, "flip")
                  },
                  "latest"],
       "id": 0
    });
    let rs = rpc_rq!(env, rq);
    // Handle response
    let json = to_json_val!(rs);
    ensure_no_err!(&json);
    // Should return gas spent value equal to the one retrieved above
    let weight_str_returned = extract_result!(&json);
    assert_eq!(weight_str_returned, &weight_str_expected);
}
