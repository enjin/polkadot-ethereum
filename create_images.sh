#!/bin/bash

BASEDIR=$(dirname $0)
pushd $BASEDIR
pushd ethereum
docker build -t efinity/artemis-etherum:latest .
popd
pushd parachain/docker
bash build.sh
popd
pushd relayer
docker build -t efinity/artemis-relayer:latest .
popd
pushd docker
docker build -t efinity/polkadot-nc:latest .
popd
popd
