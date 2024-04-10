`outdated`

- [Frontier: RPC request templates in curl](#orgbba2cda)
    - [eth: block_number](#org4fc0936)
    - [eth: eth_estimateGas](#org51f9da8)
    - [eth: eth_sendRawTransaction](#org16c2aed)
    - [eth: eth_sendTransaction](#org94358e6)
    - [eth: eth_call](#org9cc613e)
    - [eth: eth_getBlockTransactionCountByHash](#org214018b)
    - [eth: eth_getBlockByNumber](#org1acb50f)
    - [eth: eth_getBlockByHash](#org1c5962f)
    - [eth: eth_getTransactionReceipt](#org3e1ff33)
    - [eth: eth_getTransactionByBlockNumberAndIndex](#orgf171791)
    - [eth: eth_getTransactionByBlockHashAndIndex](#org6f2a455)
    - [eth: eth_getTransactionByHash](#org2736185)
    - [eth: eth_getTransactionCount](#org942af2b)
    - [eth: eth_syncing](#orgb4e3458)
    - [eth: eth_getStorageAt](#org62ef70c)
    - [eth: eth_gasPrice](#org490b701)
    - [eth: eth_blockNumber](#org2506ee9)
    - [eth: eth_getCode](#org17f09e7)
    - [eth: eth_chainId](#org4fb78ff)
    - [eth: eth_getBalance](#org3d0042e)
    - [eth: coinbase](#org71cc919)
    - [eth: accounts](#org90711ec)
    - [eth: eth_getUncleByBlockNumberAndIndex](#orgf9ac5de)
    - [eth: eth_getUncleByBlockHashAndIndex](#orgd82d916)
    - [eth: eth_getUncleCountByBlockNumber](#orga6e3e06)
    - [eth: eth_getUncleCountByBlockHash](#orgadeba27)
    - [eth: getWork](#org63881f7)
    - [eth: submitWork](#org667f743)
    - [eth: submitHashrate](#org69542f5)
    - [eth: protocolVersion](#orgd1a7b9f)
    - [eth: hashrate](#org98b6e2f)
    - [eth: eth_maxPriorityFeePerGas](#orgbb4a48f)
    - [eth: fee_history](#orgb708a23)
    - [eth: eth_mining](#org7073a08)
    - [eth: block_by_number](#org5e745a6)


<a id="orgbba2cda"></a>

# Frontier: RPC requests templates in curl


<a id="org4fc0936"></a>

### eth: block_number

Get the current block number

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_blockNumber", "params": [], "id": 0}' \
        http://localhost:9944
```


<a id="org51f9da8"></a>

### eth: eth_estimateGas

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_estimateGas", "params": [{"from":null,"to":"0x6b175474e89094c44da98b954eedeac495271d0f","data":"0x70a082310000000000000000000000006E0d01A76C3Cf4288372a29124A26D4353EE51BE"}, "finalized"], "id": 0}' \
        http://localhost:9944
```


<a id="org16c2aed"></a>

### eth: eth_sendRawTransaction

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_sendRawTransaction", "params": ["0x0000000000000000"], "id": 0}' \
        http://localhost:9944
```


<a id="org94358e6"></a>

### eth: eth_sendTransaction

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_sendTransaction", "params": [{"from":null,"to":"0x6b175474e89094c44da98b954eedeac495271d0f","data":"0x70a082310000000000000000000000006E0d01A76C3Cf4288372a29124A26D4353EE51BE"}, "finalized"], "id": 0}' \
        http://localhost:9944
```


<a id="org9cc613e"></a>

### eth: eth_call

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_call", "params": [{"from":null,"to":"0x6b175474e89094c44da98b954eedeac495271d0f","data":"0x70a082310000000000000000000000006E0d01A76C3Cf4288372a29124A26D4353EE51BE"}, "finalized"], "id": 0}' \
        http://localhost:9944
```


<a id="org214018b"></a>

### eth: eth_getBlockTransactionCountByHash

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getBlockTransactionCountByHash", "params": ["0x0000000000000000000000000000000000000000000000000000000000000000"], "id": 0}' \
        http://localhost:9944
```


<a id="org1acb50f"></a>

### eth: eth_getBlockByNumber

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getBlockByNumber", "params": [12345, false], "id": 0}' \
        http://localhost:9944
```


<a id="org1c5962f"></a>

### eth: eth_getBlockByHash

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getBlockByHash", "params": ["0x0000000000000000000000000000000000000000000000000000000000000000", false], "id": 0}' \
        http://localhost:9944
```


<a id="org3e1ff33"></a>

### eth: eth_getTransactionReceipt

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getTransactionReceipt", "params": ["0x0000000000000000000000000000000000000000000000000000000000000000"], "id": 0}' \
        http://localhost:9944
```


<a id="orgf171791"></a>

### eth: eth_getTransactionByBlockNumberAndIndex

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getTransactionByBlockNumberAndIndex", "params": ["latest", 0], "id": 0}' \
        http://localhost:9944
```


<a id="org6f2a455"></a>

### eth: eth_getTransactionByBlockHashAndIndex

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getTransactionByBlockHashAndIndex", "params": ["0x0000000000000000000000000000000000000000000000000000000000000000", 0], "id": 0}' \
        http://localhost:9944
```


<a id="org2736185"></a>

### eth: eth_getTransactionByHash

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getTransactionByHash", "params": ["0x0000000000000000000000000000000000000000000000000000000000000000"], "id": 0}' \
        http://localhost:9944
```


<a id="org942af2b"></a>

### eth: eth_getTransactionCount

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getTransactionCount", "params": ["0x0000000000000000000000000000000000000000", "latest"], "id": 0}' \
        http://localhost:9944
```


<a id="orgb4e3458"></a>

### eth: eth_syncing

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_syncing", "params": [], "id": 0}' \
        http://localhost:9944
```


<a id="org62ef70c"></a>

### eth: eth_getStorageAt

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getStorageAt", "params": ["0x0000000000000000000000000000000000000000", "0x0000000000000000000000000000000000000000000000000000", "latest"], "id": 0}' \
        http://localhost:9944
```


<a id="org490b701"></a>

### eth: eth_gasPrice

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_gasPrice", "params": [], "id": 0}' \
        http://localhost:9944
```


<a id="org2506ee9"></a>

### eth: eth_blockNumber

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_blockNumber", "params": [], "id": 0}' \
        http://localhost:9944
```


<a id="org17f09e7"></a>

### eth: eth_getCode

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getCode", "params": ["0x0000000000000000000000000000000000000000", "latest"], "id": 0}' \
        http://localhost:9944
```


<a id="org4fb78ff"></a>

### eth: eth_chainId

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_chainId", "params": [], "id": 0}' \
        http://localhost:9944
```


<a id="org3d0042e"></a>

### eth: eth_getBalance

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getBalance", "params": ["0x0000000000000000000000000000000000000000", "latest"], "id": 0}' \
        http://localhost:9944
```


<a id="org71cc919"></a>

### eth: coinbase

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_coinbase", "params": [], "id": 0}' \
        http://localhost:9944
```


<a id="org90711ec"></a>

### eth: accounts

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_accounts", "params": [], "id": 0}' \
        http://localhost:9944
```


<a id="orgf9ac5de"></a>

### eth: eth_getUncleByBlockNumberAndIndex

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getUncleByBlockNumberAndIndex", "params": [0, 0], "id": 0}' \
        http://localhost:9944
```


<a id="orgd82d916"></a>

### eth: eth_getUncleByBlockHashAndIndex

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getUncleByBlockHashAndIndex", "params": ["0x0000000000000000000000000000000000000000000000000000000000000000", 0], "id": 0}' \
        http://localhost:9944
```


<a id="orga6e3e06"></a>

### eth: eth_getUncleCountByBlockNumber

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getUncleCountByBlockNumber", "params": [707], "id": 0}' \
        http://localhost:9944
```


<a id="orgadeba27"></a>

### eth: eth_getUncleCountByBlockHash

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getUncleCountByBlockHash", "params": [ "0x0000000000000000000000000000000000000000000000000000000000000000"], "id": 0}' \
        http://localhost:9944
```


<a id="org63881f7"></a>

### eth: getWork

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getWork", "params": [], "id": 0}' \
        http://localhost:9944
```


<a id="org667f743"></a>

### eth: submitWork

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_submitWork", "params": ["0x0000000000000000", "0x0000000000000000000000000000000000000000000000000000000000000000", "0x0000000000000000000000000000000000000000000000000000000000000000"], "id": 0}' \
        http://localhost:9944
```


<a id="org69542f5"></a>

### eth: submitHashrate

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_submitHashrate", "params": ["0x0000000000000000000000000000000000000000000000000000000000000000", "0x0000000000000000000000000000000000000000000000000000000000000000"], "id": 0}' \
        http://localhost:9944
```


<a id="orgd1a7b9f"></a>

### eth: protocolVersion

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_protocolVersion", "params": [], "id": 0}' \
        http://localhost:9944
```


<a id="org98b6e2f"></a>

### eth: hashrate

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_hashrate", "params": ["0x3", 10, []], "id": 0}' \
        http://localhost:9944
```


<a id="orgbb4a48f"></a>

### eth: eth_maxPriorityFeePerGas

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_maxPriorityFeePerGas", "params": [], "id": 0}' \
        http://localhost:9944
```


<a id="orgb708a23"></a>

### eth: fee_history

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_feeHistory", "params": ["0x3", 10, []], "id": 0}' \
        http://localhost:9944
```


<a id="org7073a08"></a>

### eth: eth_mining

Get the current block number

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_mining", "params": [], "id": 0}' \
        http://localhost:9944 | jq ".result"
```


<a id="org5e745a6"></a>

### eth: block_by_number

Get block by number

```bash
   curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"jsonrpc": "2.0", "method": "eth_getBlockByNumber", "params": [3, false], "id": 0}' \
        http://localhost:9944 | jq ".result.hash"
```
