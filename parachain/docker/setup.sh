docker run -v parachainvol:/data --network host -it test/artemis:latest bash -c "/usr/local/bin/artemis build-spec --disable-default-bootnode > /data/spec.json"
docker run -v parachainvol:/data --network host -it parity/polkadot:latest bash -c "/usr/local/bin/polkadot build-spec --disable-default-bootnode  --chain=rococo-local > /data/rococo-local.json"
docker run -v parachainvol:/data --network host -it parity/polkadot:latest bash -c "/usr/local/bin/polkadot build-spec --disable-default-bootnode  --chain=rococo-local > /data/rococo-local.json"
docker run -v parachainvol:/data --network host -it parity/polkadot:latest bash -c "/usr/local/bin/polkadot build-spec --disable-default-bootnode  --chain=/data/rococo-local.json > /data/rococo-local-raw.json"
