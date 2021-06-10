pkill node || true

node /app/ganache-core.docker.cli.js \
    --port=8545 \
    --blockTime=6 \
    --networkId=344 \
    --deterministic \
    --db /data/ganachedb \
    --mnemonic='stone speak what ritual switch pigeon weird dutch burst shaft nature shove'
