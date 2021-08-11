/usr/local/bin/artemis build-spec --disable-default-bootnode > spec.json

ethereum_initial_header=$(curl http://172.28.1.4:8545 \
                          -X POST \
                          -H "Content-Type: application/json" \
                          -d '{"jsonrpc": "2.0", "method": "eth_getBlockByNumber", "params": ["latest", false], "id": 1}' \
                        | node transformEthHeader.js)

node overrideParachainSpec.js spec.json \
    genesis.runtime.verifierLightclient.initialDifficulty 0x0 \
    genesis.runtime.verifierLightclient.initialHeader "$ethereum_initial_header" \
    genesis.runtime.parachainInfo.parachainId 200 \
    para_id 200

/usr/local/bin/artemis export-genesis-state --parachain-id=200 --chain=spec.json > /data/genesisState
/usr/local/bin/artemis export-genesis-wasm --chain=spec.json > /data/genesisWasm