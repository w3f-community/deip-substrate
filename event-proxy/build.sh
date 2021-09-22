#!/bin/bash

set -e

cd $WORKSPACE
CARGO_ARGS="build --release" cargo.sh
