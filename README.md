> [!WARNING]
> This is a **Proof Of Concept**, not intended to be used in production!!
# The Polka🎭Mask

## What's this?

This project is an **experimental** add-on to Polkadot SDK's [pallet-contracts](https://github.com/paritytech/polkadot-sdk/tree/master/substrate/frame/contracts) to make it Ethereum RPC -compatible.

In a nutshell, it allows your parachain users to call <a href="https://use.ink/" target="_blank">**ink!**</a> smart contracts via <a href="https://metamask.io/" target="_blank">**MetaMask**</a>. 

## Quickstart 

Start the Polkamask development node 

```bash

cargo run -- --dev
```

Open your MetaMask and add a new network:

+ **Network name**: 🦆 
+ **New RPC URL**: http://localhost:9944
+ **Chain ID**: 42
+ **Currency symbol**: 🥚

Also import add the *well-known* development accounts: 

- *Alith*: 

   + AccountId: `0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac`
   + Private Key: `0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133`

- *Goliath*: 

   + AccountId: `0x7BF369283338E12C90514468aa3868A551AB2929`
   + Private Key: `0x96b8a38e12e1a31dee1eab2fffdf9d9990045f5b37e44d8cc27766ef294acf18`

> [!CAUTION]
> It is **highly recommended** to use a separate MetaMask instance for this (e.g. in a dedicated [browser profile](https://support.mozilla.org/en-US/kb/profiles-where-firefox-stores-user-data)), not to mix the development accounts (whose private keys **are compromised** by design) with your real money-holding accounts. 

That's it! You should right away be able to communicate with the Duck chain using your MetaMask. Let's try it as described in the following section. 

## Demo Case 

Our little demo consists of three basic actions we complete on our Substrate-based 🦆 network using *MetaMask*:

1. **Send tokens around with the MetaMask UI controls solely.**

   This is the simplest one as we already have everything set up to do this. 
   Once launched the Polkamask node with `cargo run -- --dev`, just open your MetaMask and make sure it is connected to our 🦆 network. You should see *Alith** account holding `10000000 🥚`. Go ahead and send some amount of eggs to *Goliath* or any other account you'd like to (set gas limit to `21000` as requested by MetaMask). 

2. **Send tokens via web3js-based dApp used with MetaMask for signing transactions**.
3. **Call ink! smart contract via web3js-based dApp used with MetaMask for signing transactions**.

For the actions 2,3 we have a simple [dApp](/dapp) which consists of a static web page ([index.html](/dapp/index.html)) and of course our **ink!** [contract](/dapp/contracts/flipper.ink/). This needs a bit of preparatory job to be done to set things up first.


### Set Up 

`TBD`

### Run 

`TBD`



`TODO` put this to links: 
You can also make requests to the exposed Ethereum RPC using provided [request templates](docs/rpc_requests.md). 

## Project Goals

- PoC stage target is to be able to interact with a deployed ink! example contract through MetaMask.
- Prototype stage target is to provide means for making the same thing possible with any ink! contract (which would still require some work to be done by Dapp developer).

## Plan & Status

`TODO` update 

1. [x] Basic mocked Eth RPC + boilerplate node which you can connect your MetaMask (MM) to.
2. [ ] Ethereum block emulation, to make MM satisfied with the `eth_getBlockByNumber()` response.  
   Frontier's pallet_ethereum constructs Ethereum block in the [on_finalize](https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/frame/ethereum/src/lib.rs#L206) hook, it calls the [store_block](https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/frame/ethereum/src/lib.rs#L404) fn which basically **composes** an Ethereum block and **stores** it to the [CurrentBlock](https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/frame/ethereum/src/lib.rs#L327) storage.
   The validation\execuction of this block's txs happens in [on_initialize()](https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/frame/ethereum/src/lib.rs#L230) hook. That is, the execution of the Eth block happens in the __next_ Substrate block!

   For our purposes hopefully such a hassle woudn't be needed, probably just emulating some Eth block data "on the fly" would be enough. 
   
3. [ ] Address conversion and Signing logic.  
   This seems to be simple at first glance, as this is already done in Frontier: see their [two approaches](https://github.com/paritytech/frontier/blob/master/docs/accounts.md) to account convertion.

   First approach is to just truncate first 20 bytes of Substrate address to get Eth address. Here the user has his Substrate account private key, but has no corresponding Eth account private key to be imported into MetaMask. This makes the whole thing unusable.

   Second approach is to set account and signature types for our runtime to be Ethereum-flavored, like it's done in the Frontier template node runtime:

   + `fp_account::EthereumSignature` [is set](https://github.com/paritytech/frontier/blob/0e487900e862bc3519014c1dbef800f200a00f6f/template/runtime/src/lib.rs#L70) to be the Signature type,
   + which also [sets](https://github.com/paritytech/frontier/blob/0e487900e862bc3519014c1dbef800f200a00f6f/template/runtime/src/lib.rs#L74) `fp_account::AccountId20` as the `frame_system::pallet::Config::AccountId`.

   Taken the fact that the whole point of the current PoC is to make experience of communicating with pallet_contracts through Eth RPC seamless from user perspective, we take the second approach here.
   Which also solves the signature verification issue: we use [`sp_io::crypto::secp256k1_ecdsa_recover`](https://github.com/paritytech/frontier/blob/0e487900e862bc3519014c1dbef800f200a00f6f/primitives/account/src/lib.rs#L162) for Verify trait implementation for EthereumSignature, just like it's done in the `fp_account` crate.

4. [ ] Call input encoding/decoding logic.  
   Bytes passed to the `eth_sendRawTransaction()` PRC method can be decoded as `EIP1559Transaction` as shown in [this example](docs/transfer_example.md). So the main thing here is to properly encode/decode the "input" data which goes with the call. This is some logic to be implemented by the Dapp developer: Encoding at the UI side (JS code of the Dapp), Decoding within the contract itself. We can provide some means for helping developers with it on the language level, i.e. in ink!. 

5. [ ] Add Eth RPC -originated call runner to `pallet_contracts`,  
   
   It will
   
   - decode the call to `EIP1559Transaction`, 
   - pass the call within pallet-contracts as usual (via `bare_call()`, which also transferres the balance value specified with the call).

## Useful stuff 

- The [table](docs/mapping.md) with all RPC methods needed and their description.
- [Here](docs/rpc_requests.md) are all RPC requests in curl (for testing). 
