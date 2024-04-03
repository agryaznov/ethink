//! Integraion tests for ethink!
#![allow(non_snake_case)]

mod common;

use common::{contracts::ContractInput, eth::EthTxInput, *};
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

#[tokio::test]
async fn eth_sendTransaction() {
    // Spawn node and deploy contract
    let mut env: Env<PolkadotConfig> =
        prepare_node_and_contract!(ERC20_PATH, Some("10_000"), BALTATHAR_KEY);
    // (ERC20 is deployed with 10_000 supply)
    // Make ETH RPC request (to transfer 2_000 to Alith)
    let rs = rpc_rq!(env,
    {
      "jsonrpc": "2.0",
      "method": "eth_sendTransaction",
      "params": [{
                  "from": BALTATHAR_ADDRESS,
                  "to": &env.contract_address(),
                  "data": contracts::encode(ERC20_PATH, "transfer"),
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
    let output = contracts::call(&env, "balance_of", Some(ALITH_ADDRESS), false);
    let rs = Deserializer::from_slice(&output.stdout);

    let res = to_json_val!(rs);
    println!("RES = {:#?}", &res);
    panic!()
    // Should be flipped to `true`
    // assert!(json_get!(rs["data"]["Tuple"]["values"][0]["Bool"])
    //     .as_bool()
    //     .expect("can't parse cargo contract output"));
}
