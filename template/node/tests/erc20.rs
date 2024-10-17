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

use alloy::{
    contract::{ContractInstance, Interface},
    network::{Ethereum, TransactionBuilder},
    primitives::{address, Address, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    transports::http::{Client, Http},
};
use common::{codegen::*, consts::*, *};
use ep_eth::{AccountId20, EnvelopedEncodable, EthTxInput, TransactionAction};
use ep_mapping::SubstrateWeight;
use serde_json::Deserializer;
use sp_core::{ecdsa, Pair};
use std::sync::Once;
use subxt::{Config, OnlineClient};
use futures::StreamExt;

const ERC20_PATH: &'static str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../dapp/contracts/erc20abi.ink/Cargo.toml"
);
// Sync primitive to build contract only once per test suite run
static ONCE: Once = Once::new();

#[tokio::test]
async fn transfer_works() {
    // Spawn node and deploy contract
    // let mut env: Env<PolkadotConfig> =
    //     prepare_node_and_contract!(ONCE, ERC20_PATH, vec!["1_230_000_000"], BALTATHAR_KEY);
    // (ERC20 is deployed with 10_000 supply)
    // Make ETH RPC request (to transfer 2_000 to Alith)
    // TODO ABI encode
    //    let _call_data = encode!(ERC20_PATH, "transfer", vec![ALITH_ADDRESS, "2_000"]);

    //    let contract_addr = Address::from(env.contract_address().0);
    let contract_addr = address!("99393F7eADdc5Ff70b13379986e9d7fc1485A669");
    let rpc = ProviderBuilder::new().on_http(
        //        env.http_url()
        "http://localhost:9944"
            .parse()
            .expect("failed to build alloy provider"),
    );
    let bal = rpc
        .get_balance(contract_addr)
        .await
        .expect("can't get balance");
    println!("Contract BALANCE is {bal}");
    // TODO AccountId20 -> Address
    println!("CONTRACT AccountId is {:?}", hex::encode(contract_addr));
    println!("CONTRACT ADR is {:?}", Address::from(contract_addr));
    // TODO alloy
    let contract = IERC20::new(contract_addr, rpc);
    let cal = contract.balanceOf(ALITH_ADDR);
    let bal = cal.call().await.unwrap();

    println!("Alith ERC20 BEFORE balance is: {bal:?}");
    // println!("Contract instance: {:#?}", &contract);
    let _tx_hash = contract
        .transfer(ALITH_ADDR, U256::from(100_000u64))
        .from(BALTATHAR_ADDR)
        .gas(u64::MAX)
        .send()
        .await
        .unwrap();
    // TODO: make rpc return proper receipt, and take tx_hash like this:
    //        .watch()
    //        .await
    //        .unwrap();
    //  println!("TX_HASH: {:?}", &tx_hash);
    // TODO replace with .watch() see above
    let subclient = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:9945")
        .await
        .unwrap();
    let mut blocks_sub = subclient
        .blocks()
        .subscribe_finalized()
        .await
        .expect("can't subscribe to finalized blocks")
        .take(3);

    while let Some(b) = blocks_sub.next().await {
        let block = b.expect("can't get next finalized block");
        println!("GOT NEW BLOCK: {}", &block.number());
    }
    //let _ = &env.wait_for_event("ethink.TxExecuted", 3).await;

    // Get Alith token balance
    let cal = contract.balanceOf(ALITH_ADDR);
    let bal = cal.call().await.unwrap();

    println!("Alith ERC20 AFTER balance is: {bal:?}");

    let cal = contract.balanceOf(BALTATHAR_ADDR);
    let bal = cal.call().await.unwrap();

    println!("Baltathar ERC20 balance is: {bal:?}");

    todo!()
}

//#[tokio::test]
async fn allowances_work() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> =
        prepare_node_and_contract!(ONCE, ERC20_PATH, vec!["10_000"], BALTATHAR_KEY);
    // (ERC20 is deployed with 10_000 supply)
    // Make ETH RPC request (to unauthorized transfer 2_000 to Alith)
    let input = EthTxInput {
        signer: ecdsa::Pair::from_string(ALITH_KEY, None).unwrap(),
        action: TransactionAction::Call(env.contract_address().into()),
        data: encode!(
            ERC20_PATH,
            "transfer_from",
            vec![BALTATHAR_ADDRESS, ALITH_ADDRESS, "2_000"]
        ),
        ..Default::default()
    };
    let tx = ep_eth::compose_and_sign_tx(input);
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
