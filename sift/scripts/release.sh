#!/bin/bash

set -e

echo "Building release binary..."

cargo build --release

echo "Copying binary..."

cp target/release/logforge /usr/local/bin/logforge

echo "Installed: logforge"