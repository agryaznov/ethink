# Frontier: how balance transfer works

Sending `1050 🥚` from Alith (`0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac`) to Dorothy (`0x773539d4Ac0e786233D90A233654ccEE26a613D9`):

```json
{
 "id": 8396274673272,
 "jsonrpc": "2.0",
 "method": "eth_sendRawTransaction",
 "params": [     "0x02f8732a808459682f00847d2b750082520894773539d4ac0e786233d90a233654ccee26a613d98938ebad5cdc9028000080c080a03860e7f2ba8bd770af364736a2e5e941f2b55606a56003bd4d2198d49b5f4f07a02710b02
    a6e9ae0733a51459a9a8c8b9ad5f4dee041bb1f58ccc38b4231841cee"
  ]
}
```
Here is how this tx is being seen when decoded, from the pallet_ethereum's `execute()` and `transact()`:

```
2023-09-11 15:46:30 Executing TX: EIP1559(EIP1559Transaction { chain_id: 42, nonce: 0, max_priority_fee_per_gas: 1500000000, max_fee_per_gas: 2100000000, gas_limit: 21000, action: Call(0x773539d4ac0e786233d90a233654ccee26a613d9), value: 105000000000000000000, input: [], access_list: [], odd_y_parity: true, r: 0xbde32001b7e8ccfd9f845f4f3a3950f492dfeac4e7da199c45c88994b09e98b2, s: 0x7ba13a225690dd8232ad6113b6f267d73133efb5fb04561cbf27ff35ea31b51c })
```

The decoding is being done for us by [ethereum::EnvelopedDecodable::decode()](https://docs.rs/ethereum/0.14.0/ethereum/trait.EnvelopedDecodable.html) function upon request processing in [send_raw_transaction()](https://github.com/paritytech/frontier/blob/0e487900e862bc3519014c1dbef800f200a00f6f/client/rpc/src/eth/submit.rs#L214).
Now regardless is it a contract call or just a balance transfer done through `call()` (looks like MM sends transfers exactly that way), we should transfer balance (specified in &ldquo;value:&rdquo;) from the caller to the callee.
In Frontier, this is done by _SputnikEVM_ under the hood: [`evm::executor::stack::StackExecutor::transact_call()`](https://docs.rs/evm/0.39.1/evm/executor/stack/struct.StackExecutor.html#method.transact_call), which does the [transfer](https://docs.rs/evm/0.39.1/src/evm/executor/stack/executor.rs.html#581) when required. In pallet_contracts the balance transfer is also being done as part of the call.


