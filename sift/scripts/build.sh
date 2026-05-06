#!/bin/bash

set -e

echo "Building sift..."

cargo build --release

echo "Done."