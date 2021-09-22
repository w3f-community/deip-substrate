#!/bin/bash

set -e

BUILDER_IMAGE=${BUILDER_IMAGE:-deip-rust-builder}
build_source=/home/build_source
build_cache=/home/build_cache

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

echo "+------------------------------------------------+"
echo "|    Prepare builder-image                       |"
echo "+------------------------------------------------+"
docker build -t "$BUILDER_IMAGE" .

echo "+------------------------------------------------+"
echo "|    Building...                                 |"
echo "+------------------------------------------------+"
workspace_name=$(basename $(realpath "$BUILD_SOURCE"/"$BUILD_WORKDIR"))
docker run --rm -ti -v "$BUILD_SOURCE":"$build_source" -v "$BUILD_CACHE":"$build_cache" \
  --env CARGO_HOME="$build_cache"/.cargo/"$workspace_name"/home \
  --env CARGO_TARGET_DIR="$build_cache"/.cargo/"$workspace_name"/target \
  --env WORKSPACE="$build_source"/"$BUILD_WORKDIR" \
  "$BUILDER_IMAGE" ./"$BUILD_SCRIPT"
