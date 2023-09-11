# The 🦆 Duck 

## What's this?

This project is an attempt to make pallet-contracts Ethereum RPC -compatible.

## Why duck?

This is just a working title.

If some reasoning is still required, then here it is. As we build something that should "look" (same UI: MetaMask), "swim" (same account handling\signing: Metamask) and "quack" (communicates through the same RPC) like an Ethereum ~~Duck~~ DApp, it is supposed then, according to the [Duck Test](https://en.wikipedia.org/wiki/Duck_test), to be fairly enough considered (by its users at least) as an Ethereum DApp. Also, some [rubberducking](https://en.wikipedia.org/wiki/Rubber_duck_debugging) would probably be required along the way of implemting this.  

## Project Goals

- PoC stage target is to be able to interact with a deployed ink! example contract through MetaMask.
- Prototype stage target is to provide means for making the same thing possible with any ink! contract (which would still require some work to be done by Dapp developer).

## Plan & Status

1. [x] Basic mocked Eth RPC + boilerplate node which you can connect your MetaMask (MM) to.
2. [ ] Ethereum block emulation, to make MM satisfied with the `eth_getBlockByNumber()` response.  
   Frontier's pallet_ethereum constructs Ethereum block in the [on_finalize](https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/frame/ethereum/src/lib.rs#L206) hook, it calls the [store_block](https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/frame/ethereum/src/lib.rs#L404) fn which basically **composes** an Ethereum block and **stores** it to the [CurrentBlock](https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/frame/ethereum/src/lib.rs#L327) storage.
   The validation\execuction of this block's txs happens in [on_initialize()](https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/frame/ethereum/src/lib.rs#L230) hook. That is, the execution of the eth block happens in the /next/ Substrate block!

   For our purposes hopefully such a hassle woudn't be needed, probably just emulating some Eth block data "on the fly" would be enough. 
   
3. [ ] Address conversion and Signing logic.  
   This seems to be simple at first glance, as this is already done in Frontier: see their [[https://github.com/paritytech/frontier/blob/master/docs/accounts.md][two approaches]] to account convertion.

   First approach is to just truncate first 20 bytes of Substrate address to get Eth address. Here the user has his Substrate account private key, but has no corresponding Eth account private key to be imported into MetaMask. This makes the whole thing unusable.

   Second approach is to set account and signature types for our runtime to be Ethereum-flavored, like it's done in the Frontier template node runtime:

   + `fp_account::EthereumSignature` [is set](https://github.com/paritytech/frontier/blob/0e487900e862bc3519014c1dbef800f200a00f6f/template/runtime/src/lib.rs#L70) to be the Signature type,
   + which also [sets](https://github.com/paritytech/frontier/blob/0e487900e862bc3519014c1dbef800f200a00f6f/template/runtime/src/lib.rs#L74) `fp_account::AccountId20` as the `frame_system::pallet::Config::AccountId`.

   Taken the fact that the whole point of the current PoC is to make experience of communicating with pallet_contracts through Eth RPC seamless from user perspective, we take the second approach here.
   Which also solves the signature verification issue: we use [`sp_io::crypto::secp256k1_ecdsa_recover`](https://github.com/paritytech/frontier/blob/0e487900e862bc3519014c1dbef800f200a00f6f/primitives/account/src/lib.rs#L162) for Verify trait implementation for EthereumSignature, just like it's done in the `fp_account` crate.

4. [ ] Call input encoding/decoding logic.  
   Bytes passed to the eth_sendRawTransaction() PRC method can be decoded as `EIP1559Transaction` as shown in [this example](docs/transfer_example.md). So the main thing here is to properly encode/decode the "input" data which goes with the call. This is some logic to be implemented by the Dapp developer: Encoding at the UI side (JS code of the Dapp), Decoding within the contract itself. We can provide some means for doing it on the language level, i.e. in ink!. 

5. [ ] Add Eth RPC -originated call runner to `pallet_contracts`,  
   which would be called from our Eth RPC to:  
   - decode the call to `EIP1559Transaction`, 
   - pass the call within pallet-contracts as usual (via `bare_call()`, which also transferres the balance value specified with the call).

## Useful stuff 

- The [table](docs/mapping.md) with all RPC methods needed and their description.
- [Here](docs/rpc_requests.md) are all RPC requests in curl (for testing). 
