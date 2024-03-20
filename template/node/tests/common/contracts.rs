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
