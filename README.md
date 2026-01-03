> [!WARNING]
> **Deprecation note:**
> 
> This project proved the viability of the idea of making Polkadot smart contracts compatible with tooling and wallets form the Ethereum world. It also provided working prototype. At that point, and following community discussion on the [Polkadot Forum](https://forum.polkadot.network/t/ethereum-rpc-compatibility-for-polkadot-smart-contracts/7375), Parity&rsquo;s internal team took over the initiative and started to work on the [revive](https://forum.polkadot.network/t/contracts-on-assethub-roadmap/9513), having a wider scope and [support](https://forum.polkadot.network/t/openzeppelin-support-for-polkadot-hub-and-polkadot-cloud-in-2025/11854) from OpenZeppelin. Thus this project has completed its task and now archived.


<div align="center">
    <img src=".images/ink+mm.png" alt="ink! + MetaMask logo" />
</div>

# The ethink!

This project is an **experimental** add-on to Polkadot SDK's [pallet-contracts](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/contracts) to make it Ethereum RPC -compatible.

> [!TIP]
> **In a nutshell**:
> 
> + it allows your parachain users to **interact with <a href="https://use.ink/" target="_blank">_ink!_</a> smart contracts via <a href="https://metamask.io/" target="_blank">_MetaMask_**</a>. 
> + it allows polkadot smart contract developers to **use Ethereum tooling**, like <a href="https://use.ink/" target="_blank">Foundry</a>.

## Quickstart 

Install customized `cargo-contract` tool:

<details>
<summary>
Why?
</summary>

Our 🦆-chain has _pallet-contracts_ on board and at the same time works with _Ethereum_ 20-bytes _Account_ format. The latter fact is required so that our node can understand *MetaMask*-signed transactions. But for the existing _ink!_ contracts tooling this is an unusual setting, as they're expected to work with 32-bytes long _Accounts_.  

For this reason, to work with our *ink!* contracts on this chain, we use a fork of _cargo-contract_ tool which speaks with our node the same language! Run this command to install it: 

</details>


``` bash
cargo install --git https://github.com/agryaznov/cargo-contract --branch v4-ethink --force
```


Run tests:

```bash
cargo test
```

Inject a well-known keypair of [Baltathar](docs/ethink-book/src/developer/known-accounts.md) into *ethink!* node's keystore:  
<mark>*(This step is needed only of you want to sign transactions on the node side)*</mark>  

```bash
cargo run -- key insert --dev --key-type "ethi" -d tmp --scheme ecdsa
```


Start the *ethink!* development node: 

```bash
cargo run -- --dev
```

Open your MetaMask and add a new network:

+ **Network name**: Duck 🦆 
+ **New RPC URL**: http://localhost:9944
+ **Chain ID**: 42
+ **Currency symbol**: 🥚

Import a couple of pre-funded *well-known* [development accounts](docs/ethink-book/src/developer/known-accounts.md) into your MetaMask in order to be able to sign and send transactions. 

> [!CAUTION]
> It is **highly recommended** to use a separate MetaMask instance for this (e.g. in a dedicated <a href="https://support.mozilla.org/en-US/kb/profiles-where-firefox-stores-user-data" target="_blank">browser profile</a>), not to mix the development accounts (whose private keys **are compromised** by design) with your real money-holding accounts. 

**That's it!** You should right away be able to communicate with the Duck 🦆 chain using your MetaMask. Let's see it in action, as described in the [Demo](#demo-) section below. 

## End-to-end Tests 

_ethink!_ comes with e2e integration tests, grouped into test suites:

+ [flipper](/template/node/tests/flipper.rs): basic tests for the RPC methods;  
+ [erc20](/template/node/tests/.rs): ERC20 contract tests.
+ _(more to be added later)_

Use this command to run the integration tests (at the project root): 

```bash
cargo test --test "*"
```

## Documentation 

### rustdoc

Build documentation for the project crates:

```bash
cargo doc --document-private-items --open
```


### The ethink! Book 

The book is written with [mdBook](https://rust-lang.github.io/mdBook/index.html) tool.  
To install it, run: 

```bash 
cargo install mdbook
```

Then build and open the book:

```bash
cd docs/ethink-book
mdbook serve --open
```

Happy reading!

## Demo 🧐

A comprehensive step-by-step Demo is described in the book [chapter](docs/ethink-book/src/user/demo.md). You will be able to deploy an _ink!_ contract to _ethink-node_ and use the demo dApp via MetaMask. Go ahead and give it a try!

## Useful Links 

- The [table](docs/mapping.md) with all Ethereum RPC methods needed along with their description and implementation status.
- Collection of *curl* composed [request templates](docs/rpc_requests.md) to Ethereum RPC exposed by ethink! node. 
