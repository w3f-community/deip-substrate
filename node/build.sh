#!/bin/bash

set -e

PROFILE="release"
BINARY="node-template"

if [ -z "$CARGO_HOME" ]; then
  echo "Please specify \$CARGO_HOME"
  exit 1
fi
if [ -z "$CARGO_TARGET_DIR" ]; then
  echo "Please specify \$CARGO_HOME"
  exit 1
fi

echo "+--- Build options ------------------------------+"
echo "profile=$PROFILE"
echo "binary=$BINARY"
echo "CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
echo "CARGO_HOME=$CARGO_HOME"

echo "+--- Building node ------------------------------+"
mkdir -p $CARGO_HOME
mkdir -p $CARGO_TARGET_DIR
cargo build --"$PROFILE" --bin="$BINARY"
