#!/bin/bash

set -e

BUILDER_IMAGE=${BUILDER_IMAGE:-deip-rust-builder}

REVISION=${REVISION?Please specify vcs REVISION}
BUILD_SOURCE=${BUILD_SOURCE?Please specify BUILD_SOURCE directory}
BUILD_CACHE=${BUILD_CACHE?Please specify BUILD_CACHE directory}
BUILD_SCRIPT=${BUILD_SCRIPT?Please specify BUILD_SCRIPT file (relative to BUILD_SOURCE)}
BUILD_WORKDIR=${BUILD_WORKDIR?Please specify BUILD_WORKDIR directory (relative to BUILD_SOURCE)}
BINARY_NAME=${BINARY_NAME?Please specify BINARY_NAME (build artifact)}
APP_IMAGE=${APP_IMAGE?Please specify APP_IMAGE tag}

echo "+------------------------------------------------+"
echo "|    Prepare builder-image                       |"
echo "+------------------------------------------------+"
docker build -f Dockerfile-builder -t "$BUILDER_IMAGE" .

echo "+------------------------------------------------+"
echo "|    Building...                                 |"
echo "+------------------------------------------------+"

build_source=/home/build_source
build_cache=/home/build_cache

workspace_name=$(basename $(realpath "$BUILD_SOURCE"/"$BUILD_WORKDIR"))
cargo_target_dir=.cargo/"$workspace_name"/target
build_mode=release

docker run --rm -ti -v "$BUILD_SOURCE":"$build_source" -v "$BUILD_CACHE":"$build_cache" \
  --env CARGO_HOME="$build_cache"/.cargo/"$workspace_name"/home \
  --env CARGO_TARGET_DIR="$build_cache/$cargo_target_dir" \
  --env WORKSPACE="$build_source"/"$BUILD_WORKDIR" \
  --env CARGO_BIN="$BINARY_NAME" \
  --env BUILD_MODE="$build_mode" \
  "$BUILDER_IMAGE" ./"$BUILD_SCRIPT"

artifact="$BUILD_CACHE/$cargo_target_dir/$build_mode/$BINARY_NAME"
ctx="$BUILD_CACHE"/ctx/"$BINARY_NAME_$REVISION"
mkdir -p ctx
cp $artifact ctx
docker build -f Dockerfile -t "$APP_IMAGE" \
  --build-arg=ARTIFACT=$BINARY_NAME \
  ctx
rm -rf ctx
