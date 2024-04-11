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
use serde_json::Deserializer;
use sp_core::{ecdsa, Pair, U256};
use sp_runtime::Serialize;
use std::sync::Once;

const ERC20_PATH: &'static str = env!("ERC20_PATH");
// Sync primitive to build contract only once per test suite run
static ONCE: Once = Once::new();

#[tokio::test]
async fn transfer_works() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> =
        prepare_node_and_contract!(ONCE, ERC20_PATH, vec!["10_000"], BALTATHAR_KEY);
    // (ERC20 is deployed with 10_000 supply)
    // Make ETH RPC request (to transfer 2_000 to Alith)
    let call_data = encode!(ERC20_PATH, "transfer", vec![ALITH_ADDRESS, "2_000"]);
    let rs = rpc_rq!(env,
    {
      "jsonrpc": "2.0",
      "method": "eth_sendTransaction",
      "params": [{
                  "from": BALTATHAR_ADDRESS,
                  "to": &env.contract_address(),
                  "data": call_data,
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
    let output = call!(env, "balance_of", vec![ALITH_ADDRESS]);
    let rs = Deserializer::from_slice(&output.stdout);
    // Alith balance should become 2_000
    assert_eq!(
        json_get!(rs["data"]["Tuple"]["values"][0]["UInt"])
            .as_number()
            .expect("can't parse cargo contract output")
            .as_u64(),
        Some(2_000u64)
    );
}

#[tokio::test]
async fn allowances_work() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> =
        prepare_node_and_contract!(ONCE, ERC20_PATH, vec!["10_000"], BALTATHAR_KEY);
    // (ERC20 is deployed with 10_000 supply)
    // Make ETH RPC request (to unauthorized transfer 2_000 to Alith)
    let input = EthTxInput {
        signer: ecdsa::Pair::from_string(ALITH_KEY, None).unwrap(),
        action: ethereum::TransactionAction::Call(env.contract_address().into()),
        data: encode!(
            ERC20_PATH,
            "transfer_from",
            vec![BALTATHAR_ADDRESS, ALITH_ADDRESS, "2_000"]
        ),
        ..Default::default()
    };
    let tx = eth::compose_and_sign_tx(input);
    let tx_hex = format!("0x{:x}", &tx.clone().encode());
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
    let _ = &env.wait_for_event("ethink.EthTransactionExecuted", 2).await;
    // Check state
    let output = call!(env, "balance_of", vec![ALITH_ADDRESS]);
    let rs = Deserializer::from_slice(&output.stdout);
    // Alith balance should stay 0
    assert_eq!(
        json_get!(rs["data"]["Tuple"]["values"][0]["UInt"])
            .as_number()
            .expect("can't parse cargo contract output")
            .as_u64(),
        Some(0u64)
    );
    // Make ETH RPC request (to approve transfer 2_000 to Alith)
    let call_data = encode!(ERC20_PATH, "approve", vec![ALITH_ADDRESS, "2_000"]);
    let rs = rpc_rq!(env,
    {
      "jsonrpc": "2.0",
      "method": "eth_sendTransaction",
      "params": [{
                  "from": BALTATHAR_ADDRESS,
                  "to": &env.contract_address(),
                  "data": call_data,
                  "gas": SubstrateWeight::max()
                 },
                 "latest"],
      "id": 1
    });
    // Handle response
    let json = to_json_val!(rs);
    ensure_no_err!(&json);
    let _tx_hash = extract_result!(&json);
    // Wait until tx gets executed
    let _ = &env.wait_for_event("ethink.EthTransactionExecuted", 3).await;
    // Make ETH RPC request (to authorized transfer 2_000 to Alith)
    let tx_hex = format!("0x{:x}", &tx.encode());
    let rs = rpc_rq!(env,
    {
      "jsonrpc": "2.0",
      "method": "eth_sendRawTransaction",
      "params": [ &tx_hex ],
      "id": 2
     });
    // Handle response
    let json = to_json_val!(rs);
    ensure_no_err!(&json);
    let _tx_hash = extract_result!(&json);
    // Wait until tx gets executed
    let _ = &env.wait_for_event("ethink.EthTransactionExecuted", 4).await;
    // Check state
    let output = call!(env, "balance_of", vec![ALITH_ADDRESS]);
    let rs = Deserializer::from_slice(&output.stdout);
    // Alith balance should become 2_000
    assert_eq!(
        json_get!(rs["data"]["Tuple"]["values"][0]["UInt"])
            .as_number()
            .expect("can't parse cargo contract output")
            .as_u64(),
        Some(2_000u64)
    );
}
