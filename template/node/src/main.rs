//! Substrate Node Template equipped with Ethereum RPC compatibility layer.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
    command::run()
}
