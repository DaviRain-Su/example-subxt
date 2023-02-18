# example subxt useage

## mmr_generateProof

```bash
echo '{"id":1,"jsonrpc":"2.0","method":"mmr_generateProof","params":[[4007966,4007967,4007968]]}' |  websocat -n1 -B 99999999 wss://rococo-rpc.polkadot.io | jq
```

## beefy_getFinalizedHead

```bash 
echo '{"id":1,"jsonrpc":"2.0","method":"beefy_getFinalizedHead","params":[]}' |  websocat -n1 -B 99999999 wss://rococo-rpc.polkadot.io | jq
```