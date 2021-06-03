docker run -v parachainvol:/data --network host -it parity/polkadot:latest bash -c "/usr/local/bin/polkadot --chain=/data/rococo-local-raw.json --tmp --ws-port=9944 --port=30444"
