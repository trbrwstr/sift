#!/bin/bash

set -e

echo "Building release binary..."

cargo build --release

echo "Copying binary..."

cp target/release/sift /usr/local/bin/sift

echo "Installed: sift"