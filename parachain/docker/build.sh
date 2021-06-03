#!/usr/bin/env bash
set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT/parachain

# Build the image
time docker build -f ./docker/Dockerfile --build-arg RUSTC_WRAPPER= --build-arg PROFILE=release -t test/artemis:latest .

# Show the list of available images for this repo
echo "Image is ready"
docker images | grep ${GITREPO}

popd
