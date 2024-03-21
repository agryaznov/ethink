// Copyright 2018-2022 Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    ffi::{OsStr, OsString},
    io::{BufRead, BufReader, Read},
    process,
};
use subxt::{Config, OnlineClient};

/// Spawn a local substrate node for testing.
pub struct TestNodeProcess<R: Config> {
    proc: process::Child,
    client: OnlineClient<R>,
    port: u16,
}

pub enum Protocol {
    WS,
    HTTP,
}

impl<R> Drop for TestNodeProcess<R>
where
    R: Config,
{
    fn drop(&mut self) {
        let _ = self.kill();
    }
}

impl<R> TestNodeProcess<R>
where
    R: Config,
{
    /// Construct a builder for spawning a test node process.
    pub fn build<S>(program: S) -> TestNodeProcessBuilder<R>
    where
        S: AsRef<OsStr> + Clone,
    {
        TestNodeProcessBuilder::new(program)
    }

    /// Attempt to kill the running substrate process.
    pub fn kill(&mut self) -> Result<(), String> {
        log::info!("Killing node process {}", self.proc.id());
        if let Err(err) = self.proc.kill() {
            let err = format!("Error killing node process {}: {}", self.proc.id(), err);
            log::error!("{}", err);
            return Err(err);
        }
        Ok(())
    }

    /// Returns the `subxt` client connected to the running node.
    pub fn client(&self) -> OnlineClient<R> {
        self.client.clone()
    }

    /// Returns the URL of the running node.
    pub fn url(&self, proto: Protocol) -> String {
        let (scheme, port) = match proto {
            Protocol::WS => ("ws", &self.port),
            Protocol::HTTP => ("http", &self.port),
        };
        format!("{scheme}://127.0.0.1:{port}")
    }
}

/// Construct a test node process.
pub struct TestNodeProcessBuilder<R> {
    node_path: OsString,
    signer: Option<String>,
    marker: std::marker::PhantomData<R>,
}

impl<R> TestNodeProcessBuilder<R>
where
    R: Config,
{
    pub fn new<P>(node_path: P) -> TestNodeProcessBuilder<R>
    where
        P: AsRef<OsStr>,
    {
        Self {
            node_path: node_path.as_ref().into(),
            signer: None,
            marker: Default::default(),
        }
    }

    /// Insert a ecdsa key for signing into node's keystore
    pub fn with_signer(&mut self, key: &str) -> &mut Self {
        self.signer = Some(key.to_string());
        self
    }

    /// Spawn the substrate node at the given path, and wait for RPC to be initialized.
    pub async fn spawn(&self) -> Result<TestNodeProcess<R>, String> {
        let mut cmd = process::Command::new(&self.node_path);
        cmd.env("RUST_LOG", "info")
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .arg("--dev")
            .arg("--port=0")
            .arg("--rpc-port=0");

        let mut proc = cmd.spawn().map_err(|e| {
            format!(
                "Error spawning substrate node '{}': {}",
                self.node_path.to_string_lossy(),
                e
            )
        })?;

        let stderr = proc.stderr.take().unwrap();
        // Wait for RPC port and DB path to be logged (it's logged to stderr):
        let (path, port) = find_path_and_port_from_output(stderr);
        // expect to have a number here (the chars after '127.0.0.1:') and parse them
        // into a u16.
        let port = port
            .parse()
            .unwrap_or_else(|_| panic!("valid port expected for log line, got '{port}'"));
        let ws_url = format!("ws://127.0.0.1:{port}");

        if let Some(signer) = &self.signer {
            let base_path_arg = format!("-d={path}");
            let surl_arg = format!("--suri={signer}");
            let inserted = process::Command::new(&self.node_path)
                .arg("key")
                .arg("insert")
                .arg("--dev")
                .arg(&base_path_arg)
                // TODO
                .arg("--key-type=ethi")
                .arg("--scheme=ecdsa")
                .arg(&surl_arg)
                .output()
                .map_or(false, |o| o.status.success());

            assert!(inserted, "failed to insert signer key into keystore");
        }

        // Connect to the node with a `subxt` client:
        let client = OnlineClient::from_url(ws_url.clone()).await;
        match client {
            Ok(client) => Ok(TestNodeProcess { proc, client, port }),
            Err(err) => {
                let err = format!("Failed to connect to node rpc at {ws_url}: {err}");
                log::error!("{}", err);
                proc.kill().map_err(|e| {
                    format!("Error killing substrate process '{}': {}", proc.id(), e)
                })?;
                Err(err)
            }
        }
    }
}

// Consume a stderr reader from a spawned substrate node command and
// locate the data strings needed that should be logged out to it.
fn find_path_and_port_from_output(r: impl Read + Send) -> (String, String) {
    let mut buf = BufReader::new(r);
    let (mut base_path, mut rpc_port) = (None, None);

    while base_path.is_none() || rpc_port.is_none() {
        let mut line = String::new();
        let _ = buf
            .read_line(&mut line)
            .expect("failed to obtain next line from stdout for port and base path discovery");

        if rpc_port.is_none() {
            // does the line contain our port (we expect this specific output from
            // substrate).
            if let Some(port) = line
                .rsplit_once("Running JSON-RPC server: addr=127.0.0.1:")
                .map(|(_, s)| s)
                // trim non-numeric chars from the end of the port part of the line.
                .and_then(|s| s.split_once(","))
                .map(|s| s.0.to_string())
            {
                rpc_port = Some(port);
                continue;
            }
        }

        if base_path.is_none() {
            // does the line contain database path (we expect this specific output from
            // substrate).
            if let Some(path) = line
                .rsplit_once("Database: RocksDb at ")
                .map(|(_, s)| s)
                // extract base-path from db-path
                .and_then(|s| s.split_once("/chains/"))
                .map(|(p, _)| p.to_string())
            {
                base_path = Some(path);
            }
        }
    }

    (
        base_path.expect("we should find a base path before the reader ends"),
        rpc_port.expect("we should find a port before the reader ends"),
    )
}
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use subxt::PolkadotConfig as SubxtConfig;

//     #[tokio::test]
//     #[allow(unused_assignments)]
//     async fn spawning_and_killing_nodes_works() {
//         let mut client1: Option<OnlineClient<SubxtConfig>> = None;
//         let mut client2: Option<OnlineClient<SubxtConfig>> = None;

//         {
//             let node1 = TestNodeProcess::<SubxtConfig>::build("substrate-contracts-node")
//                 .spawn()
//                 .await
//                 .unwrap();
//             client1 = Some(node1.client());

//             let node2 = TestNodeProcess::<SubxtConfig>::build("substrate-contracts-node")
//                 .spawn()
//                 .await
//                 .unwrap();
//             client2 = Some(node2.client());

//             let res1 = node1.client().rpc().block_hash(None).await;
//             let res2 = node1.client().rpc().block_hash(None).await;

//             assert!(res1.is_ok());
//             assert!(res2.is_ok());
//         }

//         // node processes should have been killed by `Drop` in the above block.
//         let res1 = client1.unwrap().rpc().block_hash(None).await;
//         let res2 = client2.unwrap().rpc().block_hash(None).await;

//         assert!(res1.is_err());
//         assert!(res2.is_err());
//     }
// }
