# High-Level Design

In a nutshell, _ethink!_ is like _Frontier_, but for _pallet-contracts_ instead of _pallet-evm_.

Here is the simplified (non-exhaustive) components map to the compatibility layers introduced in the beginning of this post:

![components map](/images/ethink-post-1_2.png)

There are 3 main pieces of it:

-   `RPC` The RPC "frontier", exposing the RPC endpoint which looks just like a normal Ethereum RPC.

    Its function is to accept requests and decode them, then call an appropriate method of the API exposed by our Substrate runtime, and response back to the caller.
    By adding this piece to our node, we make it look like an Ethereum node to the caller, which is normally a dapp's frontend.

-   `GLUE` The Runtime of our node, which provides special methods in its exposed API, gets calls from the RPC "frontier" and routes them further.

    Some of them, like e.g. account balance checks or contract "dry-run" calls, does not bring any state changes. For such cases it just calls the corresponding pallet's API method (like `pallet_balances::free_balance()`).

    For the ones that do alter state (called _transactions_ in Ethereum and _extrinsics_ in Polkadot), the mechanics is as follows. The incoming Ethereum transaction is being wrapped into an [UncheckedExtrinsic](https://paritytech.github.io/polkadot-sdk/master/sp_runtime/generic/struct.UncheckedExtrinsic.html) with the call to `pallet-ethink::transact(),` which decodes passed-in Ethereum transaction, and routes the call further (based on type of the destination account) to a specific destination pallet. Specifically, for the calls addressed to an account which belongs to a contract, the destination module is _pallet-contracts_. If the callee address is a user account, the destination module is _pallet-balances_, as the call is considered to be just a balance transfer. (This logic is inherited from Ethereum).

    The wrapper extrinsic is being first put to the transaction pool. Just like any other extrinsic, it has its special logic for checking its validity. Upon such validation, the Ethereum signature is being checked, and the caller account is being extracted from it. The further way for the extrinsic is no different from any other extrinsic on its way to execution and inclusion into a block.

-   `EXEC` Execution of the contract happens in `pallet-contracts` module as usual.


### No Upstream Changes {#no-upstream-changes}

For all this stuff to work, no customization is required neither for _pallet-contracts_ nor for _ink!_. Polkadot-sdk overall, and its smart contracts stuff in particular, was designed with quite a good level of abstraction, allowing building such compatibility layers on top of it, with no tightly coupling to a particular execution environment. _(And this is what makes ethink! different to Frontier, which is tightly coupled to pallet-evm.)_

When it comes to tooling, in particular [cargo-contract](https://github.com/agryaznov/cargo-contract/tree/ethink) and [subxt](https://github.com/agryaznov/subxt/tree/ethink) have got some customization which makes it work with Ethereum accounts and signing, as well as to speak with _ethink!_ node in the same language (for that chain metadata was updated).

> Worth mentioning here, as "_ethink!_" name could be a little confusing, that this solution is not solely for ink! contracts.
> It is basically language-agnostic, and low coupled with the executor (meaning it should work both with Wasm and RISC-V -flavored pallet-contracts, and could be adopted to other embedders\executors if someday we will have new options for that). Still the most mature contracts language in our ecosystem is ink!, and it's basically the only feasible one for production use so far. That's why the project naming was made with an accent on it.

