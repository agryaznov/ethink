use crate::common::*;
use std::process;

/// Deploy contract to the node exposed via `url`, and return the output
pub fn deploy(manifest_path: &str, url: &str) -> process::Output {
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
pub fn call(url: &str, manifest_path: &str, address: &str, msg: &str) -> process::Output {
    let surl_arg = format!("-s={ALITH_KEY}");
    let manifest_arg = format!("--manifest-path={manifest_path}");
    let url_arg = format!("--url={url}");
    let contract_arg = format!("--contract={address}");
    let msg_arg = format!("--message={msg}");

    process::Command::new("cargo")
        .arg("contract")
        .arg("call")
        .arg(&surl_arg)
        .arg(&contract_arg)
        .arg(&msg_arg)
        .arg("--output-json")
        .arg(&manifest_arg)
        .arg(&url_arg)
        .output()
        .expect("failed to call with cargo-contract")
}
