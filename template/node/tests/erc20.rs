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

//! Integraion tests for ethink! ERC20
#![allow(non_snake_case)]
use alloy::{
    network::EthereumWallet,
    primitives::{address, Address, U256},
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
};
use serde_json::Deserializer;
use sp_core::{ecdsa, Pair};
use std::sync::Once;

use common::{codegen::*, consts::*, *};
use ep_eth::{AccountId20, EnvelopedEncodable, EthTxInput, TransactionAction};
use ethink_runtime::{Weight, ED};

mod common;

const ERC20_PATH: &'static str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../dapp/contracts/erc20abi.ink/Cargo.toml"
);
// Sync primitive to build contract only once per test suite run
// ERC20 is being deployed with 1230 *10^6 supply
const ERC20_SUPPLY: u128 = 1_230_000_000;

static ONCE: Once = Once::new();

#[tokio::test]
async fn transfer_works() {
    // SUBSTRATE RPC: Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> = prepare_node_and_contract!(
        ONCE,
        ERC20_PATH,
        vec![&ERC20_SUPPLY.to_string()],
        BALTATHAR_KEY
    );
    // TODO put to Env
    // Build alloy ETH RPC provider
    // NOTE here we use
    // BALTATHAR key which is inserted into node's keystore
    // hence for his transactions we build provider with no wallet
    // we need this to cover send_transaction() rpc method working
    let rpc = ProviderBuilder::new().on_http(
        env.http_url()
            .parse()
            .expect("failed to build alloy provider"),
    );
    // ETH RPC: query contract balance
    let contract_bal = rpc
        .get_balance(env.contract_addr())
        .await
        .expect("can't get balance");
    // Deployed contract should have ED balance
    assert_eq!(contract_bal, U256::from(ED));
    // Get our ink! contract instance as Solidity contract
    let contract = IERC20::new(env.contract_addr(), rpc);
    // ETH RPC: query ERC20 token balances
    let (cal_a, cal_b) = (contract.balanceOf(ALITH), contract.balanceOf(BALTATHAR));
    let (a_bal, b_bal) = (
        cal_a.call().await.unwrap()._0,
        cal_b.call().await.unwrap()._0,
    );
    // Alith ERC20 token balance should be zero
    assert_eq!(a_bal, U256::ZERO);
    // Baltathar ERC20 token balance should be total supply
    assert_eq!(b_bal, U256::from(ERC20_SUPPLY));

    // ETH RPC: send tx to transfer 100k of ERC20 to Alith
    // NOTE BALTATHAR's key is inserted into the node's keystore
    let _tx_hash = contract
        .transfer(ALITH, U256::from(100_000))
        .from(BALTATHAR)
        .gas(u64::MAX)
        .send()
        .await
        .unwrap();
    // TODO: make rpc return proper receipt, and wait for tx execution completeness like this:
    //        .watch()
    //        .await
    //        .unwrap();

    // Wait tx to be included into block
    let _ = &env.wait_for_event("Ethink.TxExecuted", 3).await;
    // ETH RPC: query ERC20 token balances
    let (a_bal, b_bal) = (
        cal_a.call().await.unwrap()._0,
        cal_b.call().await.unwrap()._0,
    );
    // Alith ERC20 token balance should become 100k
    assert_eq!(a_bal, U256::from(100_000));
    // Baltathar ERC20 token balance should be total_supply - 100k
    assert_eq!(b_bal, U256::from(ERC20_SUPPLY - 100_000));
}

#[tokio::test]
async fn approve_allowance_transfer_from_works() {
    // SUBSTRATE RPC: Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> = prepare_node_and_contract!(
        ONCE,
        ERC20_PATH,
        vec![&ERC20_SUPPLY.to_string()],
        BALTATHAR_KEY
    );
    // TODO put to common::provider_with_key()
    // Alith signer
    let signer: PrivateKeySigner = ALITH_KEY.parse().expect("can't parse Alith key");
    let wallet = EthereumWallet::from(signer);
    // Build alloy ETH RPC provider
    let rpc = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(
            env.http_url()
                .parse()
                .expect("failed to build alloy provider"),
        );
    // BALTATHAR key is inserted into node's keystore
    // hence for his transactions we build provider with no wallet
    // TODO put to common::provider_without_key()
    let rpc_b = ProviderBuilder::new().on_http(
        env.http_url()
            .parse()
            .expect("failed to build alloy provider"),
    );
    // Get our ink! contract instance as Solidity contract
    let contract = IERC20::new(env.contract_addr(), rpc);
    // ETH RPC: query ERC20 token balances
    let (cal_a, cal_b) = (contract.balanceOf(ALITH), contract.balanceOf(BALTATHAR));
    let (a_bal, b_bal) = (
        cal_a.call().await.unwrap()._0,
        cal_b.call().await.unwrap()._0,
    );
    assert_eq!(a_bal, U256::ZERO);
    assert_eq!(b_bal, U256::from(ERC20_SUPPLY));
    // ETH RPC: send tx for (unauthorized) transfer 35k of ERC20 to Alith
    let _tx_hash = contract
        .transfer(ALITH, U256::from(35_000))
        .from(ALITH)
        .gas(u64::MAX)
        .send()
        .await
        .unwrap();
    // Wait until tx fails
    let _ = &env.wait_for_event("System.ExtrinsicFailed", 3).await;

    // Balances should stay the same
    // ETH RPC: query ERC20 token balances
    let (a_bal, b_bal) = (
        cal_a.call().await.unwrap()._0,
        cal_b.call().await.unwrap()._0,
    );
    assert_eq!(a_bal, U256::ZERO);
    assert_eq!(b_bal, U256::from(ERC20_SUPPLY));

    // ETH RPC: send tx to approve spending 100k of ERC20 to Alith
    let contract_b = IERC20::new(env.contract_addr(), rpc_b);
    let _tx_hash = contract_b
        .approve(ALITH, U256::from(100_000))
        .from(BALTATHAR)
        .gas(u64::MAX)
        .send()
        .await
        .unwrap();

    // Wait until tx gets executed
    let _ = &env.wait_for_event("Ethink.TxExecuted", 3).await;

    // Check allowance
    let cal_allowance = contract.allowance(BALTATHAR, ALITH);
    assert_eq!(cal_allowance.call().await.unwrap()._0, U256::from(100_000));

    // ETH RPC: send tx for (authorized) transfer 35k of ERC20 to Alith
    let _tx_hash = contract
        .transferFrom(BALTATHAR, ALITH, U256::from(35_000))
        .from(ALITH)
        .gas(u64::MAX)
        .send()
        .await
        .unwrap();

    // Wait until tx gets executed
    let _ = &env.wait_for_event("Ethink.TxExecuted", 3).await;

    // ETH RPC: query ERC20 token balances
    let (a_bal, b_bal) = (
        cal_a.call().await.unwrap()._0,
        cal_b.call().await.unwrap()._0,
    );
    // Alith ERC20 token balance should become 35k
    assert_eq!(a_bal, U256::from(35_000));
    // Baltathar ERC20 token balance should be total_supply - 35k
    assert_eq!(b_bal, U256::from(ERC20_SUPPLY - 35_000));
    // Remaining allowance should be
    assert_eq!(cal_allowance.call().await.unwrap()._0, U256::from(65_000));
}
