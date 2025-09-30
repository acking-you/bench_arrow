# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a benchmark comparison project that evaluates the performance of different Apache Arrow implementations when reading Parquet files. The project compares:
- arrow-rs (Rust implementation)
- arrow2 (Alternative Rust implementation) 
- arrow-cpp (C++ implementation)

## Commands

### Run All Benchmarks
- `sh run_benchmarks.sh` - Run all benchmarks (arrow-rs, arrow2, and arrow-cpp) sequentially

### Individual Benchmarks

#### Rust Benchmarks
- `cargo run --release parquet` - Run arrow-rs benchmark
- `cargo run --release parquet2` - Run arrow2 benchmark
- `cargo check` - Check Rust code compilation
- `cargo build --release` - Build optimized Rust binary

#### C++ Benchmarks
- `cd bench-cpp && sh pre_build.sh` - Configure CMake build (requires VCPKG and C++ toolchain environment variables)
- `cd bench-cpp && sh run.sh` - Build and run arrow-cpp benchmark

## Code Architecture

### Rust Implementation
- Entry point: `src/main.rs` - Contains benchmarking framework that runs functions multiple times (10 runs with 2 warmup cycles by default)
- Parquet readers: `src/parquet/mod.rs` - Contains implementations for both arrow-rs (`sync_read_with_parquet`) and arrow2 (`sync_read_with_parquet2`)
- Test data: Reads from `./test_data/hits_20.parquet` (1,000,000 rows)

### C++ Implementation
- Located in `bench-cpp/` directory
- Uses CMake with vcpkg for dependency management
- Requires environment variables: `CPP_TOOLCHAIN_PATH` and `VCPKG_ROOT`

### Benchmark Methodology
- Each implementation reads the same Parquet file 10 times
- Performance is measured with warmup iterations to avoid cold-start effects
- Results show individual run times, averages, and outlier detection