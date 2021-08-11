#!/usr/bin/env bash
set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

# Build the image
time docker build -f ./parachain/docker/Dockerfile --build-arg RUSTC_WRAPPER= --build-arg PROFILE=release -t efinity/artemis:latest .

# Show the list of available images for this repo
echo "Image is ready"
popd
