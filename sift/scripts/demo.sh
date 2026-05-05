#!/bin/bash

set -e

echo ""
echo "=============================="
echo "⚡ LogForge Demo"
echo "=============================="
echo ""

sleep 1
echo "Step 1: Inspecting sample log file..."
sleep 1

echo ""
echo "$ head -n 3 examples/sample.log"
head -n 3 examples/sample.log

sleep 2

echo ""
echo "Step 2: Basic analysis (full file scan)"
sleep 1

echo ""
echo "$ logforge analyze examples/sample.log"
sleep 1
cargo run -p logforge-cli -- analyze examples/sample.log

sleep 2

echo ""
echo "Step 3: Filtering only errors"
sleep 1

echo ""
echo "$ logforge analyze examples/sample.log --filter error"
sleep 1
cargo run -p logforge-cli -- analyze examples/sample.log --filter "error"

sleep 2

echo ""
echo "Step 4: Structured table output"
sleep 1

echo ""
echo "$ logforge analyze examples/sample.log --output table"
sleep 1
cargo run -p logforge-cli -- analyze examples/sample.log --output table

sleep 2

echo ""
echo "Step 5: Quick stats view"
sleep 1

echo ""
echo "$ logforge stats examples/sample.log"
sleep 1
cargo run -p logforge-cli -- stats examples/sample.log

sleep 2

echo ""
echo "Step 6: Performance benchmark"
sleep 1

echo ""
echo "$ logforge bench examples/sample.log"
sleep 1
cargo run -p logforge-cli -- bench examples/sample.log

sleep 2

echo ""
echo "=============================="
echo "Done."
echo "=============================="
echo ""
echo "Key idea: large logs → instant insight"
echo ""