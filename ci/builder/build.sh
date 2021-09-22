#!/bin/bash

set -e

BUILDER_IMAGE=${BUILDER_IMAGE:-deip-node-builder}
BUILDER_WORKDIR=/home/build

if [ -z "$WORKSPACE" ]; then
  echo "Please specify workspace directory"
  exit 1
fi

if [ -z "$BUILD_SCRIPT" ]; then
  echo "Please specify build script file name (relative to WORKSPACE)"
  exit 1
fi

echo "+------------------------------------------------+"
echo "|    Prepare builder-image                       |"
echo "+------------------------------------------------+"
docker build -t "$BUILDER_IMAGE" .

echo "+------------------------------------------------+"
echo "|    Building substrate-node                     |"
echo "+------------------------------------------------+"
docker run --rm -ti -v "$WORKSPACE":"$BUILDER_WORKDIR" \
  --env CARGO_HOME="$BUILDER_WORKDIR"/ci/builder/.cargo/home \
  --env CARGO_TARGET_DIR="$BUILDER_WORKDIR"/ci/builder/.cargo/target \
  "$BUILDER_IMAGE" ./"$BUILD_SCRIPT"
