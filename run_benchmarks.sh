#!/bin/bash

set -e

echo "================================"
echo "Arrow Benchmark Suite"
echo "================================"
echo ""

# Run arrow-rs benchmark
echo "Running arrow-rs benchmark..."
echo "----------------------------"
cargo run --release parquet
echo ""

# Run arrow2 benchmark
echo "Running arrow2 benchmark..."
echo "----------------------------"
cargo run --release parquet2
echo ""

# Run arrow-cpp benchmark
echo "Running arrow-cpp benchmark..."
echo "----------------------------"
cd bench-cpp
if [ ! -d "build" ]; then
    echo "Configuring CMake build..."
    sh pre_build.sh
fi
sh run.sh
cd ..
echo ""

echo "================================"
echo "All benchmarks completed!"
echo "================================"