#!/bin/bash

set -e

echo "Building LogForge..."

cargo build --release

echo "Done."