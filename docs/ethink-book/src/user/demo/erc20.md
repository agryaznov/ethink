# ERC20

> In this section we go through simple steps of deploying our **ERC20 abi-compatible *ink!* smart contract**, as well as using it with: 
> 
> + [cast](https://book.getfoundry.sh/cast/) cli from [Foundry](https://book.getfoundry.sh/) toolchain for Ethereum,
> + **MetaMask** browser extension.

### Prepare

Take just the same steps as described in the [flipper demo](flipper.md#prepare)

### Set Up 

#### Build contract


```bash 
cd dapp/contracts/erc20abi.ink
cargo contract build 
```

#### Deploy contract

Make sure you've started our network template node:

```bash
cargo run -- --dev
```

Now deploy the contract: 

```bash 
cd dapp/contracts/erc20abi.ink
cargo contract instantiate -s 0x8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b --args=1230000000 --config=Ecdsachain -x
```

(Notice we use [*Baltathar's*](/developer/known-accounts.md) private key here for transaction signing).

You should get the contract's code hash and address upon the successful completion of the transaction: 

``` bash
Code hash 0x9a055c40b591cada60986ef74815a66d4cce17ba7d3bef535f72deda190fb8df
Contract 0xB9213e3aE172BD011c58647C4AC8A64e3321683b
```

### 🚀 Run It! 

#### cast 

Let's run the `approve()` and `transfer_from()` flow using Foundry's [`cast`](https://book.getfoundry.sh/cast/) tool.

*Baltathar* should have all token supply on his account as token's contract owner: 

```bash 
cast call 0xB9213e3aE172BD011c58647C4AC8A64e3321683b "balanceOf(address)(uint256)" 0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0 -r localhost:9944
```

Result is `1230000000`.

*Alith* should have no tokens so far:

```bash 
cast call 0xFec75beb93b48945c15aDfcc8bf257567C1D7E25 "balanceOf(address)(uint256)" 0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac -r localhost:9944
```

Result is `0`.

Now let's allow *Alith* to spend `100_000` tokens on behalf of *Baltathar*:

```bash 
cast send -f 0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0 --private-key 0x8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b 0xFec75beb93b48945c15aDfcc8bf257567C1D7E25 "approve(address,uint256)(bool)" 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac 100000 -r localhost:9944 --gas-limit 17446744073709551615 --legacy --async
```

We get dispatched transaction hash as the command's output.
Once the transaction gets included into the next block, *Alith* should get the allowance to spend the tokens. 
Let's check this with this command:

```bash 
cast call 0xFec75beb93b48945c15aDfcc8bf257567C1D7E25 "allowance(address,address)(uint256)" 0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac -r localhost:9944
```

Result is `100000` which means the allowance came into force.
Now let's transfer some tokens from *Baltathar* to *Alith*.
The next transaction is sent by *Alith*:

```bash 
cast send -f 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac --private-key 0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133 0xFec75beb93b48945c15aDfcc8bf257567C1D7E25  "transferFrom(address,address,uint256)(bool)" 0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac 35000 -r localhost:9944 --gas-limit 17446744073709551615 --legacy --async
```
We get dispatched transaction hash as the command's output.
Once the transaction gets included into the next block, *Alith* should get the tokens to her account:

```bash 
cast call 0xFec75beb93b48945c15aDfcc8bf257567C1D7E25 "balanceOf(address)(uint256)" 0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac -r localhost:9944
```

Result is `35000`.

Now that 35000 out of 100000 is spent, new allowance balance should have been updated:

```bash 
cast call 0xFec75beb93b48945c15aDfcc8bf257567C1D7E25 "allowance(address,address)(uint256)" 0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac -r localhost:9944
```

Result is `65000` as expected.


> This flow is automated in the `erc20` e2e [test](https://github.com/agryaznov/ethink/blob/master/template/node/tests/erc20.rs) with the help of [`alloy.rs`](https://alloy.rs/). 

#### MetaMask 

Now let's add our token to Metamask and transfer it using its built-in support for ERC20 tokens.


At `Tokens` tab, click on `+ Import` and insert our deployed ERC20 contract address: `0xB9213e3aE172BD011c58647C4AC8A64e3321683b`:

![Import Token](/images/erc20_mm_flow-1.png)

*Baltathar* should have `1230` *ink!* balance as ERC20 contract owner:

![Balance](/images/erc20_mm_flow-2.png)

Let's now transfer some tokens to *Alith* using built-in MetaMask support for ERC20: 

![Transfer1](/images/erc20_mm_flow-3.png)
![Transfer2](/images/erc20_mm_flow-4.png)

Shortly after being dispatched, our transfer transaction makes it into the block:

![Transfer3](/images/erc20_mm_flow-5.png)

If you switch MetaMask account to *Alith* you should see her balance increased: 

![Transfer4](/images/erc20_mm_flow-6.png)


> We thereby demonstrated how our *ink!* ERC20 token deployed to *ethink!* network is fully compatible with MetaMask built-in token support.
