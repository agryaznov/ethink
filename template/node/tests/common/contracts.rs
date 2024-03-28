use crate::common::*;
use std::io::BufRead;
use std::io::BufReader;
use std::process;

pub struct Weight(pub sp_weights::Weight);

impl From<&serde_json::Map<std::string::String, serde_json::Value>> for Weight {
    fn from(value: &serde_json::Map<std::string::String, serde_json::Value>) -> Self {
        let ref_time = value["ref_time"]
            .as_number()
            .expect("no ref_time number in response")
            .as_u64()
            .expect("failed to parse ref_time as u64");

        let proof_size = value["proof_size"]
            .as_number()
            .expect("no proof_size number in response")
            .as_u64()
            .expect("failed to parse proof_size as u64");

        Self(sp_weights::Weight::from_parts(ref_time, proof_size))
    }
}
// How we encode Weight into U256 to comply with ETH RPC return value
impl From<Weight> for sp_core::U256 {
    fn from(value: Weight) -> sp_core::U256 {
        // TODO via Encode
        sp_core::U256([value.0.ref_time(), value.0.proof_size(), 0, 0])
    }
}

/// Deploy contract to the node exposed via `url`, and return the output
pub fn deploy(url: &str, manifest_path: &str) -> process::Output {
    let surl_arg = format!("-s={ALITH_KEY}");
    let manifest_arg = format!("--manifest-path={manifest_path}");
    let url_arg = format!("--url={}", url);

    process::Command::new("cargo")
        .arg("contract")
        .arg("instantiate")
        .arg(&surl_arg)
        .arg("--args=false")
        .arg("-x")
        .arg("--skip-confirm")
        .arg("--output-json")
        .arg(&manifest_arg)
        .arg(&url_arg)
        .output()
        .expect("failed to instantiate with cargo-contract")
}

/// Call contract on the node exposed via `url`, and return the output
pub fn call(
    url: &str,
    manifest_path: &str,
    address: &str,
    msg: &str,
    execute: bool,
) -> process::Output {
    let surl = &format!("-s={ALITH_KEY}");
    let manifest = &format!("--manifest-path={manifest_path}");
    let url = &format!("--url={url}");
    let contract = &format!("--contract={address}");
    let msg = &format!("--message={msg}");

    let mut args = vec![
        "contract",
        "call",
        surl,
        url,
        manifest,
        contract,
        msg,
        "--output-json",
    ];
    if execute {
        args.push("-x")
    }

    let output = process::Command::new("cargo")
        .args(args.as_slice())
        .output()
        .expect("failed to call with cargo-contract");

    assert!(output.status.success());

    output
}

/// Encode input data for contract call
pub fn encode(manifest_path: &str, msg: &str) -> String {
    let manifest_arg = &format!("--manifest-path={manifest_path}");
    let msg_arg = &format!("--message={msg}");

    let output = process::Command::new("cargo")
        .arg("contract")
        .arg("encode")
        .arg(&manifest_arg)
        .arg(&msg_arg)
        .output()
        .expect("failed to encode with cargo-contract");

    assert!(output.status.success());

    // perse stdout for the encoded data string
    let hex = BufReader::new(output.stdout.as_slice())
        .lines()
        .find_map(|line| {
            let line = line.expect("failed to get next line from cargo-contract stdout");
            line.split_once("Encoded data: ")
                .map(|(_, hex)| hex.to_owned())
        })
        .expect("can't find encoded data string in cargo-contract stdout");

    format!("0x{hex}")
}
