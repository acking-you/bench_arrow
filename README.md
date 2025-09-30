# Arrow-rs VS Arrow-cpp

[![Continuous Benchmarking](https://img.shields.io/badge/status-continuously%20updated-green)](./docs/benchmark-results/)

> 🔄 **This benchmark suite is continuously updated.** Check the [benchmark results directory](./docs/benchmark-results/) for the latest performance comparisons.

## Overview

Performance comparison of Apache Arrow implementations for Parquet file reading:
- **arrow-rs**: Rust implementation of Apache Arrow
- **arrow2**: Alternative Rust implementation 
- **arrow-cpp**: C++ implementation of Apache Arrow

## Source Code

- Rust implementations: [parquet-reader](./src/parquet/mod.rs)
- C++ implementation: [parquet-reader](./bench-cpp/main.cpp)

## Current Versions

### Rust Dependencies
```toml
arrow = { version = "55.2.0", features = ["prettyprint", "ipc_compression"] }
parquet = "55.2.0"
arrow2 = { version = "0.18.0", features = ["io_parquet", "io_parquet_compression"] }
parquet2 = "0.17.2"
```

### C++ Dependencies (vcpkg)
```json
{
  "dependencies": [
    {
      "name": "arrow",
      "default-features": false,
      "features": ["parquet"]
    }
  ],
  "overrides": [
    {
      "name": "arrow",
      "version-string": "20.0.0#1"
    }
  ]
}
```

## Build Configuration

### Rust Compilation
```bash
# Standard release build with optimization level 3
cargo build --release

# Run benchmarks
cargo run --release parquet   # arrow-rs
cargo run --release parquet2  # arrow2
```

### C++ Compilation
```bash
# CMake with Release configuration
cd bench-cpp
sh pre_build.sh  # Configures CMake with vcpkg toolchain
sh run.sh        # Builds with -DCMAKE_BUILD_TYPE=Release and runs

# Requires environment variables:
# - CPP_TOOLCHAIN_PATH
# - VCPKG_ROOT
```

## How to Run Benchmarks

### Run All Benchmarks
```bash
sh run_benchmarks.sh
```

This will automatically run all three implementations sequentially.

### Run Individual Benchmarks

1. **arrow-rs**: `cargo run --release parquet`
2. **arrow2**: `cargo run --release parquet2`
3. **arrow-cpp**: `cd bench-cpp && sh pre_build.sh && sh run.sh`

## Latest Benchmark Results

### Test Environment
- **CPU**: 32 Cores
- **Memory**: 64GB
- **Disk**: Delay 500μs, bandwidth 150MB/s
- **Dataset**: `test_data/hits_20.parquet` (1,000,000 rows)
- **Methodology**: 10 iterations with 2 warmup cycles

### Performance Summary

| Implementation | avg(ms) | comparison    | relative performance |
|---------------|---------|---------------|---------------------|
| **arrow-rs**  | 850.08  | ★★★ (fastest) | 1.00x              |
| arrow-cpp     | 1358.71 | ★★☆           | 1.60x              |
| arrow2        | 1609.19 | ★☆☆           | 1.89x              |

📊 **[View detailed benchmark results →](./docs/benchmark-results/2024-12-initial.md)**

## Benchmark History

All benchmark results are archived with detailed environment information, version specifications, and compilation flags:

- [Latest Results](./docs/benchmark-results/)
- [Benchmark Template](./docs/benchmark-results/BENCHMARK_TEMPLATE.md) - Template for recording new benchmark results

## Contributing

When adding new benchmark results:
1. Use the [benchmark template](./docs/benchmark-results/BENCHMARK_TEMPLATE.md)
2. Include all version information and compilation flags
3. Document any environment-specific optimizations
4. Save results with format: `YYYY-MM-description.md`
