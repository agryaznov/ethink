# Compatibility Explained


Consider a typical dapp tech stack:

![layers](/images/ethink-post-1_1.png)

Simply put, there are the following layers of it:

-   UI built with _web3js_ libraries,

    which speaks to a node via its RPC;

-   `RPC` exposed by a chain node,

    which speaks to node's runtime via its API;

-   `EXEC` Contracts execution engine,

    implemented as a runtime module;

-   `CHAIN` protocol logic,

    implemented in other runtime modules.

When we want to port an Ethereum dapp to our Substrate-based chain, we need to make sure that:

1.  We expose the RPC endpoint which speaks to UI in compliance with the Ethereum RPC [spec](https://ethereum.github.io/execution-apis/api-documentation/).
2.  On the Execution layer, this implies that our engine should allow contracts to implement the same business logic as in EVM contracts,
    while keeping the same calling conventions between caller and callee (more on that in the next section).
3.  Aside from input and output of the contract being called, a dapp might rely on the underlying chain protocol data,
    such as block and transaction data, gas prices, storage state, etc.

    For our RPC to provide such data, our runtime should have logic for translating Substrate chain data to Ethereum chain data. _(Right away this point could sound like a dealbreaker. But hold on, things might not be so bad. Keep reading to the next section.)_

Now that we have pointed out what has to be done, let's dive into how we try to achieve this.


## Solving The Puzzle {#solving-the-puzzle}

As we've pointed out, just exposing the Ethereum-alike looking RPC is not enough. The underlying logic should comply with it as well (at least to a certain degree).

For that the system must be able to be compatible to how things work in EVM on the layers below the RPC. First of which is the execution engine.


### Execution Layer {#execution-layer}

Luckily for us, _pallet-contracts_ module of Substrate had been designed to be in feature-parity with EVM. Both are stack-based machines with gas metering, contract accounts are of the same nature as external (user) accounts (from the chain protocol perspective), there are constructors and call messages, payable and non-payable messages, as well as support for cross-contracts calls (both context-switching and delegated ones aka libraries), and overall its API allows implementing pretty much everything one would expect from an Ethereum smart contract.

Still, there is one quite a sound discrepancy, which albeit do not really relate to the engine level. It is the way contract input data is encoded: Solidity contracts use ABI encoding, whereas ink! contracts use SCALE. But this is a _language_ level difference. On the lower, execution level, our contracts (both Wasm and RISC-V) accept input just as a byte sequence. How it's being treated\decoded is determined by contract intrinsic logic, meaning contract developer can implement it in the way so that it works with ABI encoded data. Of course this would not be a very handy thing to do from the developer point of view, given no special tooling provided for that. But what's important is that it is not an insurmountable obstacle, and we will talk a bit on how to deal with that further in this post.

Next compatibility things refer to chain-specific data.


### Chain Layer {#chain-layer}

Here it comes to the most noticeable difference between Polkadot and Ethereum building blocks: account format and crypto primitives:

-   Normally in Polkadot we use _32-bytes-long AccountId_ accompanied with _Schnorrkel/Ristretto sr25519_ algorithm for keys and signing, and _blake2_ for hashing.
-   Ethereum standard is _20-bytes-long AccountId_ and _ECDSA secp256k1 key pairs_ in combination with _keccak256_ hash function.

There are more other discrepancies like e.g. the fact that extrinsic Id is not guaranteed to be unique between blocks in Substrate-based chains, while for Ethereum transactions that's the fact on which a good part of business logic relies upon. Also, Ethereum has just gas for measuring computational effort, whereas in Polkadot we have two-dimensional weight, counting for execution time and for implied size of the proof data used by PVF. There are more other issues, but all of those seem to be surmountable (at least of this point of the research).

As per _AccountId_, well... Polkadot was designed so that you can customize everything, and that unlocks some opportunities. First, nothing restricts your parachain from using whatever _AccountId_ and elliptic curves you want in your runtime's business logic. More to that, _polkadot-sdk_ provides ready to use crypto primitives for this.

Second, here we have to turn back to the objective we set at the beginning of this post, which is to get _existing_ Ethereum users aboard. And then we have to admit that users are not going to switch to other wallets/signing extensions right away, overnight. That means that for account format and keypairs we have to allow them keeping what they currently use.

What we have listed in this section is not an exhaustive set of possible compatibility issues on the chain level, there might be others. For now in _ethink!_ some of RPC methods related to chain data are mocked, others return Substrate chain data. With the contracts tested so far, this looks like not a problem. In general, it should not be a showstopper, as in the worst case we could construct and store a fake Ethereum block for every Substrate block, this approach seems to work fine in Frontier-based chains. But again, the research is ongoing, and this would need to be tested in practice with porting real Ethereum dapps onto this solution.

Finally, we can't just wave a magic wand and solve the problem for all parties (users and developers) at once. Then let's break it into steps, the roadmap of which could look like the one presented in the following section.


### The Road To All-Hands Compatibility {#the-road-to-all-hands-compatibility}

As a first step, in short-term we may try to change as little as possible for the user, which naturally comes with the cost of additional work to be done by the dapp developer.
In particular, to deal with ABI/SCALE encoding mismatch, as well as different message selectors, the frontend piece of the dapp would have to be modified to work with metadata of `a_contract.ink` instead of `a_contract.sol` one. Or, the contract piece of the dapp would have to be taught to deal with ABI encoding so that it takes the same input from the caller and returns the same output as the original Solidity contract does. In any case, unless we have a tooling for those things, the contract developer would have to re-write his Solidity contract in ink! by hand, which is totally feasible thing to do, as ink! was designed to have similar contract layout to Solidity, and if you look at its basic examples, you find a number of such ported contracts.

Good news are that the tools for automation of such a translation are in active development: there is [solang](https://forum.polkadot.network/t/contracts-update-solidity-on-polkavm/6949) compiler. I'm not sure if we are there yet, but once it's ready, we take the second step, and let dapp developer just re-compile a Solidity contract source to be deployed to _pallet-contracts_.

Last but not the least, some helpful tooling for developers, either for the ease of porting the frontend piece of dapp, or for making a contract deal with ABI-encoded input, could also possibly be provided. (At the end of the day, ink! contracts are basically Rust code, so chances are you could possibly use some existing crates for that).

What's being said could be summarized to the table:

![road](/images/ethink-post-1_3.png)

Currently, we are in the first column, and it feels like starting entering to the second one.

Following this narrative, we may envision the target scenario as follows:

> **Target Scenario:**
>
> 1.  Take a successful Ethereum dapp.
> 2.  Transcompile its contract from _Solidity_ to _Wasm/PolkaVM's RISC-V_.
> 3.  _[might not be needed]_ Port dapp frontend so that it deals with new metadata.
>
>     From _[ABI encoded input + selector]_ to _[SCALE encoded input + selector]_.
>
>     (As mentioned before, this step might not be needed, ideally could be solved on the previous step.)
>
> 4.  Deploy the contract to a parachain having _ethink!_ aboard.
>
> 5.  Profit!!

Now that we explained how we deal with particular compatibility issues, let's have a look on how we bring all the pieces together to a running chain node which has _pallet-contracts_ and works with Metamask.

