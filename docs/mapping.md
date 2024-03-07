# RPC methods mapping 

For reference,

- look into `ethink-rpc-core` for [Definitions](https://github.com/agryaznov/ethink/blob/master/client/rpc-core/src/eth.rs#L30),
- look into `polkamas-rpc` for [Implementations](https://github.com/agryaznov/ethink/blob/master/client/rpc/src/lib.rs#L69).

In the table below all the methods required to be exposed are grouped into the 5 categories, with the expected implementation difficulty descending from the top group to the bottom one.

```
| ☑ | method                                    | desc                                                                                 | can mock/impl? | caveats                                       |
|----+-------------------------------------------+--------------------------------------------------------------------------------------+----------------+-----------------------------------------------|
|    | Group 1                                   |                                                                                      |                |                                               |
|----+-------------------------------------------+--------------------------------------------------------------------------------------+----------------+-----------------------------------------------|
|    | call                                      | dry-run a contract call via pallet_evm::Runner:call();                               |                | Major problems: Caling conventions,           |
|    |                                           | basically this is what pallet_contracts::bare_call() does;                           | yes/yes        | ABI compatibility                             |
| x  | send_raw_transaction                      | submits on-chain transaction call by sending tx via pallet_ethereum [[https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/frame/ethereum/src/lib.rs#L283][dispatchable]]     |                |                                               |
|    |                                           | expects raw bytes of the encoded signed tx                                           |                |                                               |
|    |                                           | this is the core thing to implement so that it calls pallet_contracts [[https://github.com/paritytech/substrate/blob/70fb25ad8a78c8a87f78dcb9055f548548275a4b/frame/contracts/src/lib.rs#L653][dispatchable]]   | yes/yes        |                                               |
|    |                                           | see [[https://ethereum.stackexchange.com/a/770][this SE]] for perfect explanation on call and send difference;                     |                |                                               |
|    | send_transaction                          | same as above but expects tx Object (instead of raw data) and implies                | yes/?          |                                               |
|    |                                           | signing the tx on the node side (by in-node stored signer)                           |                |                                               |
|    | estimate_gas [async]                      | basically make a dry-run and get the gas comsumed, or system::dry_run()              | yes/yes        |                                               |
|----+-------------------------------------------+--------------------------------------------------------------------------------------+----------------+-----------------------------------------------|
| x  | Group 2                                   |                                                                                      |                |                                               |
|----+-------------------------------------------+--------------------------------------------------------------------------------------+----------------+-----------------------------------------------|
| x  | block_by_hash [async]                     | get Eth block by its hash                                                            | yes/yes        | Major question:                               |
| x  | block_by_number [async]                   | same by block number                                                                 | yes/yes        | Should we store Eth block in pallet-contracts |
| x  | block_transaction_count_by_hash [async]   | same but returns number of txs in the block                                          | yes/?          | storage as well, or is it possible to         |
| x  | block_transaction_count_by_number [async] | same but returns number of txs in the block                                          | yes/?          | emulate it on the fly?                        |
|----+-------------------------------------------+--------------------------------------------------------------------------------------+----------------+-----------------------------------------------|
|    | Group 3                                   |                                                                                      |                |                                               |
|----+-------------------------------------------+--------------------------------------------------------------------------------------+----------------+-----------------------------------------------|
|    | transaction_by_hash [async]               | we'd need to transform the tx data so that is looks like Eth tx                      | yes/yes        | Tx data should be easily encodable between    |
|    | transaction_by_block_hash_and_index       | same                                                                                 | yes/yes        | Eth and Substrate formats                     |
|    | transaction_by_block_number_and_index     | same                                                                                 | yes/yes        |                                               |
| x  | transaction_receipt                       |                                                                                      | yes/yes        | See [[https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/frame/ethereum/src/lib.rs#L559][this fn]] on how to build a reciept         |
|----+-------------------------------------------+--------------------------------------------------------------------------------------+----------------+-----------------------------------------------|
|    | Group 4                                   |                                                                                      |                |                                               |
|----+-------------------------------------------+--------------------------------------------------------------------------------------+----------------+-----------------------------------------------|
|    | accounts                                  | accounts owned by the client, stored in fc_rpc::Eth.signers, Vec<[[https://paritytech.github.io/frontier/rustdocs/fc_rpc/trait.EthSigner.html][EthSigner]]>          | yes/?          | Mehods in this group are expected to be       |
|    | author                                    | in Eth RPC it's coinbase address, but fc_rpc just sends current block beneficiary    | yes/?          | easily provided by the pallet_contracts       |
| x  | balance [async]                           | get the balance of an account                                                        | yes/yes        | existing means, with some possible data       |
| x  | chain_id                                  | handled by pallet_evm_chain_id, this is a constant defined in runtime                | yes/yes        | transformations where needed.                 |
|    | version                                   | protocl version, mocked with constant                                                | yes/yes        |                                               |
|    | code_at [async]                           | uses pallet_evm, we could impl the same or just return code_hash                     | yes/yes        |                                               |
| x  | block_number                              | get the current block number; basically we pass the substrate's one                  | yes/yes        |                                               |
|    | gas_price                                 | can return this with [[https://github.com/paritytech/substrate/blob/70fb25ad8a78c8a87f78dcb9055f548548275a4b/frame/transaction-payment/rpc/runtime-api/src/lib.rs#L51][tx_payment_rpc::query_weight_to_fee()]]                           | yes/yes        |                                               |
|    | storage_at [async]                        | uses pallet_evm; we can do it with [[https://github.com/paritytech/substrate/blob/70fb25ad8a78c8a87f78dcb9055f548548275a4b/frame/contracts/src/storage.rs#L138][ContractInfo::read()]]                              | yes/yes        |                                               |
|    | syncing                                   | gets it from [[https://paritytech.github.io/substrate/master/sc_network_sync/service/chain_sync/struct.SyncingService.html#method.is_major_syncing][sc_network_sync::service::chain_sync::SyncingService]]                    | yes/yes        |                                               |
| x  | transaction_count [async]                 | bascially returns account's nonce; do-able via [[https://paritytech.github.io/substrate/master/frame_system/pallet/struct.Pallet.html#method.account_nonce][frame_system::pallet::account_nonce()]] | yes/yes        |                                               |
|----+-------------------------------------------+--------------------------------------------------------------------------------------+----------------+-----------------------------------------------|
|    | Group 5                                   |                                                                                      |                |                                               |
|----+-------------------------------------------+--------------------------------------------------------------------------------------+----------------+-----------------------------------------------|
|    | fee_history [async]                       | can be mocked with a constant, at least in PoC                                       | yes/?          | Methods in this group apparently could be     |
|    | is_mining                                 | I think we can mock it with const false, at least at PoC stage                       | yes/yes        | just mocked                                   |
|    | max_priority_fee_per_gas                  | weird calc based on historic block rewards, probably can always return 0             | yes/?          |                                               |
|    | hashrate                                  | just returns 0                                                                       | yes/no         |                                               |
|    | protocol_version                          | just returns 1                                                                       | yes/no         |                                               |
|    | net_verison                               |                                                                                      |                |                                               |
|    | submit_hashrate                           | just returns false                                                                   | yes/no         |                                               |
|    | submit_work                               | just returns false                                                                   | yes/no         |                                               |
|    | block_uncles_count_by_number              | just returns 0                                                                       | yes/no         |                                               |
|    | block_uncles_count_by_hash                | just returns 0                                                                       | yes/no         |                                               |
|    | uncle_by_block_hash_and_index             | just returns None                                                                    | yes/no         |                                               |
|    | uncle_by_block_number_and_index           | just returns None                                                                    | yes/no         |                                               |
|    | work                                      | just returns constant default                                                        | yes/no         |                                               |

```
