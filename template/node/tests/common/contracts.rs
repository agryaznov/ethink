use crate::{
    common::{consts::*, *},
    Serialize,
};
use serde::Serializer;
use std::{
    io::{BufRead, BufReader},
    process,
};

#[derive(Clone)]
pub struct ContractInput(Vec<u8>);

impl Serialize for ContractInput {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        format!("0x{}", hex::encode(&self.0)).serialize(serializer)
    }
}

impl From<Vec<u8>> for ContractInput {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl Into<Vec<u8>> for ContractInput {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

/// Deploy contract to the node exposed via `url`, and return the output
pub fn deploy(
    url: &str,
    manifest_path: &str,
    args: Vec<&str>,
    signer: Option<&str>,
) -> process::Output {
    let surl_arg = &format!("-s={}", signer.unwrap_or(ALITH_KEY));
    let manifest_arg = format!("--manifest-path={manifest_path}");
    let url_arg = format!("--url={}", url);

    let mut cmd_args = vec![
        "contract",
        "instantiate",
        &surl_arg,
        &url_arg,
        &manifest_arg,
        "-x",
        "--skip-confirm",
        "--output-json",
    ];

    let args = args
        .iter()
        .map(|a| format!("--args={a}"))
        .collect::<Vec<_>>();
    for s in &args {
        cmd_args.push(s)
    }

    process::Command::new("cargo")
        .args(cmd_args.as_slice())
        .output()
        .expect("failed to instantiate with cargo-contract")
}

/// Call contract deployed to env, and return the output
pub fn call(
    env: &Env<PolkadotConfig>,
    msg: &str,
    args: Vec<&str>,
    execute: bool,
    signer: Option<&str>,
) -> process::Output {
    let surl_arg = &format!("-s={}", signer.unwrap_or(ALITH_KEY));
    let manifest_arg = &format!("--manifest-path={}", env.contract.manifest_path);
    let url_arg = &format!("--url={}", env.ws_url());
    let contract_arg = &format!("--contract={}", env.contract.address);
    let msg_arg = &format!("--message={msg}");

    let mut cmd_args = vec![
        "contract",
        "call",
        surl_arg,
        url_arg,
        manifest_arg,
        contract_arg,
        msg_arg,
        "--output-json",
    ];

    let args = args
        .iter()
        .map(|a| format!("--args={a}"))
        .collect::<Vec<_>>();
    for s in &args {
        cmd_args.push(s)
    }

    if execute {
        cmd_args.push("-x")
    }

    let output = process::Command::new("cargo")
        .args(cmd_args.as_slice())
        .output()
        .expect("failed to call with cargo-contract");

    assert!(output.status.success(), "err: {:#?}", &output);

    output
}

/// Encode input data for contract call
pub fn encode(manifest_path: &str, msg: &str, args: Vec<&str>) -> ContractInput {
    let manifest_arg = &format!("--manifest-path={manifest_path}");
    let msg_arg = &format!("--message={msg}");

    let mut cmd_args = vec!["contract", "encode", manifest_arg, msg_arg];

    let args = args
        .iter()
        .map(|a| format!("--args={a}"))
        .collect::<Vec<_>>();
    for s in &args {
        cmd_args.push(s)
    }

    let output = process::Command::new("cargo")
        .args(cmd_args.as_slice())
        .output()
        .expect("failed to encode with cargo-contract");

    assert!(output.status.success(), "err: {:#?}", &output);

    // parse stdout for the encoded data string
    let bytes = BufReader::new(output.stdout.as_slice())
        .lines()
        .find_map(|line| {
            let line = line.expect("failed to get next line from cargo-contract stdout");
            line.split_once("Encoded data: ")
                .map(|(_, hex)| hex.to_owned())
        })
        .map(hex::decode)
        .expect("can't find encoded data string in cargo-contract stdout")
        .expect("can't deserialize encoded data from cargo-contract stdout");

    ContractInput(bytes)
}
