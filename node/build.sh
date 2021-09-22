#!/bin/bash

set -e

cd $WORKSPACE
CARGO_ARGS="build --release --bin=node-template" cargo.sh
