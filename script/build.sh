#!/bin/bash

# This script is used to build the project.
cd $(dirname "$0")/..
mkdir -p export && rm -rf export/*
mkdir -p export/build
mkdir -p export/build/bin

# Build the project
cd ipm-core/
cargo build --release
cp target/release/ipm ../export/build/bin/ipm
cd ../export/build/
# Create System Structure
mkdir -p package
mkdir -p package/list
mkdir -p package/cache
mkdir -p package/installed