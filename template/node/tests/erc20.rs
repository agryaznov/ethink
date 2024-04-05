//! Integraion tests for ethink!
#![allow(non_snake_case)]

mod common;

use common::{consts::*, *};
use ep_crypto::{AccountId20, EthereumSignature};
use ep_mapping::{SubstrateWeight, Weight};
use ep_rpc::EthTransaction;
use ethereum::{LegacyTransaction, LegacyTransactionMessage, TransactionSignature};
use serde_json::Deserializer;
use sp_core::{ecdsa, U256};
use sp_runtime::Serialize;

pub const ERC20_PATH: &'static str = env!("ERC20_PATH");

#[tokio::test]
async fn transfer_works() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> =
        prepare_node_and_contract!(ERC20_PATH, vec!["10_000"], BALTATHAR_KEY);
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
