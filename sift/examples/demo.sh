#!/usr/bin/env bash

set -e

CMD="sift"
if ! command -v sift &> /dev/null; then
  CMD="cargo run -p sift --"
fi

echo ""
echo "=============================="
echo "⚡ Sift Demo"
echo "=============================="
echo ""

sleep 1
echo "Inspecting JSON logs..."
sleep 1

echo ""
echo "$ head -n 3 examples/app.json"
head -n 3 examples/app.json

sleep 2

echo ""
echo "Analyze JSON logs"
echo "$ $CMD analyze examples/app.json --format json"
sleep 1
$CMD analyze examples/app.json --format json

sleep 2

echo ""
echo "Filter errors from JSON logs"
echo "$ $CMD analyze examples/app.json --format json --filter error"
sleep 1
$CMD analyze examples/app.json --format json --filter "error"

sleep 2

echo ""
echo "Inspect nginx logs"
echo "$ head -n 3 examples/nginx.log"
head -n 3 examples/nginx.log

sleep 2

echo ""
echo "Analyze nginx logs"
echo "$ $CMD analyze examples/nginx.log --format nginx --output table"
sleep 1
$CMD analyze examples/nginx.log --format nginx --output table

sleep 2

echo ""
echo "Quick stats"
echo "$ $CMD stats examples/nginx.log --format nginx"
sleep 1
$CMD stats examples/nginx.log --format nginx

sleep 2

echo ""
echo "Benchmark"
echo "$ $CMD bench examples/app.json"
sleep 1
$CMD bench examples/app.json

sleep 2

echo ""
echo "=============================="
echo "Done."
echo "=============================="
echo ""
echo "Sift: find signal in logs instantly."
echo ""