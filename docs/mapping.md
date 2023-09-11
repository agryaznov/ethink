# RPC methods mapping 

For fast reference:

-   see [Definitions](https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc-core/src/eth.rs#L30)
-   see [Implementations](https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/mod.rs#L172)


In the table below all the methods required to be exposed are grouped into the 5 categories, with the expected implementation difficulty descending from the top group to the bottom one.


<table border="2" cellspacing="0" cellpadding="6" rules="groups" frame="hsides">


<colgroup>
<col  class="org-left" />

<col  class="org-left" />

<col  class="org-left" />
</colgroup>
<thead>
<tr>
<th scope="col" class="org-left">method</th>
<th scope="col" class="org-left">desc</th>
<th scope="col" class="org-left">caveats</th>
</tr>
</thead>

<tbody>
<tr>
<td class="org-left">Group 1</td>
<td class="org-left">&#xa0;</td>
<td class="org-left">&#xa0;</td>
</tr>
</tbody>

<tbody>
<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/execute.rs#L76">call</a></td>
<td class="org-left">dry-run a contract call via pallet_evm::Runner:call();</td>
<td class="org-left">Major problems: Caling conventions,</td>
</tr>


<tr>
<td class="org-left">&#xa0;</td>
<td class="org-left">basically this is what pallet_contracts::bare_call() does;</td>
<td class="org-left">ABI compatibility</td>
</tr>


<tr>
<td class="org-left">&#xa0;</td>
<td class="org-left">see <a href="https://ethereum.stackexchange.com/a/770">this SE</a> for perfect explanation on call and send difference</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/submit.rs#L52">send_transaction</a></td>
<td class="org-left">exec on-chain a contract call by sending tx via pallet_ethereum <a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/frame/ethereum/src/lib.rs#L283">dispatchable</a>;</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">&#xa0;</td>
<td class="org-left">this is the core thing to implement so that it calls pallet_contracts <a href="https://github.com/paritytech/substrate/blob/70fb25ad8a78c8a87f78dcb9055f548548275a4b/frame/contracts/src/lib.rs#L653">dispatchable</a></td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">send_raw_transaction</td>
<td class="org-left">same as above but takes encoded tx in bytes</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/execute.rs#L408">estimate_gas</a> [async]</td>
<td class="org-left">basically make a dry-run and get the gas comsumed, or system::dry_run()</td>
<td class="org-left">&#xa0;</td>
</tr>
</tbody>

<tbody>
<tr>
<td class="org-left">Group 2</td>
<td class="org-left">&#xa0;</td>
<td class="org-left">&#xa0;</td>
</tr>
</tbody>

<tbody>
<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/block.rs#L49">block_by_hash</a> [async]</td>
<td class="org-left">get Eth block by its hash</td>
<td class="org-left">Major question:</td>
</tr>


<tr>
<td class="org-left">block_by_number [async]</td>
<td class="org-left">same by block number</td>
<td class="org-left">Should we store Eth block in pallet-contracts</td>
</tr>


<tr>
<td class="org-left">block_transaction_count_by_hash [async]</td>
<td class="org-left">same but returns number of txs in the block</td>
<td class="org-left">storage as well, or is it possible to</td>
</tr>


<tr>
<td class="org-left">block_transaction_count_by_number [async]</td>
<td class="org-left">same but returns number of txs in the block</td>
<td class="org-left">emulate it on the fly?</td>
</tr>
</tbody>

<tbody>
<tr>
<td class="org-left">Group 3</td>
<td class="org-left">&#xa0;</td>
<td class="org-left">&#xa0;</td>
</tr>
</tbody>

<tbody>
<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/transaction.rs#L50">transaction_by_hash</a> [async]</td>
<td class="org-left">we&rsquo;d need to transform the tx data so that is looks like Eth tx</td>
<td class="org-left">Tx data should be easily encodable between</td>
</tr>


<tr>
<td class="org-left">transaction_by_block_hash_and_index</td>
<td class="org-left">same</td>
<td class="org-left">Eth and Substrate formats</td>
</tr>


<tr>
<td class="org-left">transaction_by_block_number_and_index</td>
<td class="org-left">same</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/transaction.rs#L270">transaction_receipt</a></td>
<td class="org-left">&#xa0;</td>
<td class="org-left">See <a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/frame/ethereum/src/lib.rs#L559">this fn</a> on how to build a reciept</td>
</tr>
</tbody>

<tbody>
<tr>
<td class="org-left">Group 4</td>
<td class="org-left">&#xa0;</td>
<td class="org-left">&#xa0;</td>
</tr>
</tbody>

<tbody>
<tr>
<td class="org-left">accounts</td>
<td class="org-left">accounts owned by the client, stored in fc_rpc::Eth.signers, Vec&lt;<a href="https://paritytech.github.io/frontier/rustdocs/fc_rpc/trait.EthSigner.html">EthSigner</a>&gt;</td>
<td class="org-left">Mehods in this group are expected to be</td>
</tr>


<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/client.rs#L69">author</a></td>
<td class="org-left">in Eth RPC it&rsquo;s coinbase address, but fc_rpc just sends current block beneficiary</td>
<td class="org-left">easily provided by the palle_contracts</td>
</tr>


<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/state.rs#L49">balance</a> [async]</td>
<td class="org-left">get the balance of an account</td>
<td class="org-left">existing means, with some possible data</td>
</tr>


<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/client.rs#L98">chain_id</a></td>
<td class="org-left">handled by pallet_evm_chain_id, we can mock it w constant at least in PoC</td>
<td class="org-left">transformations where needed.</td>
</tr>


<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/state.rs#L171">code_at</a> [async]</td>
<td class="org-left">uses pallet_evm, we could impl the same or just return code_hash</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">block_number</td>
<td class="org-left">get the current block number; basically we pass the substrate&rsquo;s one</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">gas_price</td>
<td class="org-left">can return this with <a href="https://github.com/paritytech/substrate/blob/70fb25ad8a78c8a87f78dcb9055f548548275a4b/frame/transaction-payment/rpc/runtime-api/src/lib.rs#L51">tx_payment_rpc::query_weight_to_fee()</a></td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/state.rs#L80">storage_at</a> [async]</td>
<td class="org-left">uses pallet_evm; we can do it with <a href="https://github.com/paritytech/substrate/blob/70fb25ad8a78c8a87f78dcb9055f548548275a4b/frame/contracts/src/storage.rs#L138">ContractInfo::read()</a></td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/client.rs#L49">syncing</a></td>
<td class="org-left">gets it from <a href="https://paritytech.github.io/substrate/master/sc_network_sync/service/chain_sync/struct.SyncingService.html#method.is_major_syncing">sc_network_sync::service::chain_sync::SyncingService</a></td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/state.rs#L116">transaction_count</a> [async]</td>
<td class="org-left">bascially returns account&rsquo;s nonce; do-able via <a href="https://paritytech.github.io/substrate/master/frame_system/pallet/struct.Pallet.html#method.account_nonce">frame_system::pallet::account_nonce()</a></td>
<td class="org-left">&#xa0;</td>
</tr>
</tbody>

<tbody>
<tr>
<td class="org-left">Group 5</td>
<td class="org-left">&#xa0;</td>
<td class="org-left">&#xa0;</td>
</tr>
</tbody>

<tbody>
<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/fee.rs#L53">fee_history</a> [async]</td>
<td class="org-left">can be mocked with a constant, at least in PoC</td>
<td class="org-left">Methods in this group apparently could be</td>
</tr>


<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/mining.rs#L30">is_mining</a></td>
<td class="org-left">I think we can mock it with const false, at least at PoC stage</td>
<td class="org-left">just mocked</td>
</tr>


<tr>
<td class="org-left"><a href="https://github.com/paritytech/frontier/blob/22aaafe089218f6cee625898fff7b953cc793228/client/rpc/src/eth/fee.rs#L185">max_priority_fee_per_gas</a></td>
<td class="org-left">weird calc based on historic block rewards, probably can always return 0</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">hashrate</td>
<td class="org-left">just returns 0</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">protocol_version</td>
<td class="org-left">just returns 1</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">submit_hashrate</td>
<td class="org-left">just returns false</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">submit_work</td>
<td class="org-left">just returns false</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">block_uncles_count_by_number</td>
<td class="org-left">just returns 0</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">block_uncles_count_by_hash</td>
<td class="org-left">just returns 0</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">uncle_by_block_hash_and_index</td>
<td class="org-left">just returns None</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">uncle_by_block_number_and_index</td>
<td class="org-left">just returns None</td>
<td class="org-left">&#xa0;</td>
</tr>


<tr>
<td class="org-left">work</td>
<td class="org-left">just returns constant default</td>
<td class="org-left">&#xa0;</td>
</tr>
</tbody>
</table>






