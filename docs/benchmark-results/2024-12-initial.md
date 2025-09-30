# Benchmark Result - 2024-12-Initial

## Test Information

### Date
2024-12 (Initial benchmark)

### Test Environment
- **CPU**: 32 Cores
- **Memory**: 64GB
- **OS**: Linux (assumed from benchmark environment)
- **Disk**: Delay 500 microseconds, bandwidth 150MB/s

### Software Versions

#### Rust Dependencies
```toml
arrow = { version = "55.2.0", features = ["prettyprint", "ipc_compression"] }
parquet = "55.2.0"
arrow2 = { version = "0.18.0", features = ["io_parquet", "io_parquet_compression"] }
parquet2 = "0.17.2"
```

#### C++ Dependencies
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

### Compilation Flags

#### Rust
```bash
cargo build --release
# Default release optimizations (opt-level = 3)
```

#### C++
```bash
cmake -DCMAKE_BUILD_TYPE=Release
# Standard C++ release optimizations
```

## Test Configuration
- **Dataset**: `test_data/hits_20.parquet` (1,000,000 rows)
- **Iterations**: 10 runs
- **Warmup Cycles**: 2 iterations

## Results

### Summary Table

| Implementation | Average (ms) | Min (ms) | Max (ms) | Std Dev (ms) | Relative Performance |
|---------------|-------------|----------|----------|--------------|---------------------|
| arrow-rs      | 850.08      | 834.29   | 865.76   | ~10.82       | 1.00x               |
| arrow-cpp     | 1358.71     | 1324.43  | 1423.47  | ~30.79       | 1.60x               |
| arrow2        | 1609.19     | 1582.29  | 1628.69  | ~15.12       | 1.89x               |

### Detailed Results

#### arrow-rs
```
Benchmark Results:
[01] Duration: 834.286508ms
[02] Duration: 843.900381ms
[03] Duration: 856.391019ms
[04] Duration: 865.760784ms
[05] Duration: 851.303845ms
[06] Duration: 852.156418ms
[07] Duration: 856.119058ms
[08] Duration: 843.616954ms
[09] Duration: 854.13646ms
[10] Duration: 843.094212ms
Average:   850.076563ms | Total: 8.500765639s
```

#### arrow2
```
Benchmark Results:
[01] Duration: 1.62560117s
[02] Duration: 1.609013149s
[03] Duration: 1.610131629s
[04] Duration: 1.628685417s
[05] Duration: 1.602293649s
[06] Duration: 1.615221061s
[07] Duration: 1.614847208s
[08] Duration: 1.594796727s
[09] Duration: 1.582285578s
[10] Duration: 1.609006676s
Average:   1.609188226s | Total: 16.091882264s
```

#### arrow-cpp
```
Benchmark Results:
[01] Duration: 1365.320 ms
[02] Duration: 1423.472 ms
[03] Duration: 1324.434 ms
[04] Duration: 1357.547 ms
[05] Duration: 1325.981 ms
[06] Duration: 1345.143 ms
[07] Duration: 1345.340 ms
[08] Duration: 1354.894 ms
[09] Duration: 1392.111 ms
[10] Duration: 1352.823 ms
Average:   1358.706 ms | Total: 13587.065 ms
```

## Notes
- Performance ranking: **arrow-rs** (★★★ fastest) > arrow-cpp (★★☆) > arrow2 (★☆☆)
- arrow-rs demonstrates the best performance, being approximately 60% faster than arrow-cpp and 89% faster than arrow2
- All tests performed on the same hardware with identical dataset