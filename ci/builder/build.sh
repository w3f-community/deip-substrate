#!/bin/bash

set -e

BUILDER_IMAGE=${BUILDER_IMAGE:-deip-rust-builder}
revision=$(git rev-parse --short HEAD)

if [ -z "$BUILD_SOURCE" ]; then
  echo "Please specify BUILD_SOURCE directory"
  exit 1
fi
if [ -z "$BUILD_CACHE" ]; then
  echo "Please specify BUILD_CACHE directory"
  exit 1
fi
if [ -z "$BUILD_SCRIPT" ]; then
  echo "Please specify BUILD_SCRIPT file (relative to BUILD_SOURCE)"
  exit 1
fi
if [ -z "$BUILD_WORKDIR" ]; then
  echo "Please specify BUILD_WORKDIR directory (relative to BUILD_SOURCE)"
  exit 1
fi
if [ -z "$BINARY_NAME" ]; then
  echo "Please specify BINARY_NAME (build artifact)"
  exit 1
fi

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
ctx="$BUILD_CACHE"/ctx/"$BINARY_NAME_$revision"
mkdir -p ctx
cp $artifact ctx
echo $revision
docker build -f Dockerfile -t "$BINARY_NAME:$revision" \
  --build-arg=ARTIFACT=$BINARY_NAME \
  ctx
rm -rf ctx
