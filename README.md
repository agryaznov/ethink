> [!WARNING]
> This is an **Early Prototype**. It works, but not (*yet*) intended for production use!!

<div align="center">
    <img src=".images/ink+mm.png" alt="ink! + MetaMask logo" />
</div>

# The ethink!

This project is an **experimental** add-on to Polkadot SDK's [pallet-contracts](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/contracts) to make it Ethereum RPC -compatible.

> [!TIP]
> **In a nutshell**, it allows your parachain users to **call <a href="https://use.ink/" target="_blank">_ink!_</a> smart contracts via <a href="https://metamask.io/" target="_blank">_MetaMask_**</a>. 

## Quickstart 

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
cargo test --test *
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

Then build and open the book

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
