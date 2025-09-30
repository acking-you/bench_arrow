# Benchmark Result Template

## Test Information

### Date
YYYY-MM-DD HH:MM:SS (Timezone)

### Test Environment
- **CPU**: [Model and Cores]
- **Memory**: [Amount in GB]
- **OS**: [OS Name and Version]
- **Disk**: [Type, Delay, Bandwidth]

### Software Versions

#### Rust Dependencies
```toml
arrow = { version = "X.X.X", features = [...] }
parquet = "X.X.X"
arrow2 = { version = "X.X.X", features = [...] }
parquet2 = "X.X.X"
```

#### C++ Dependencies
```json
{
  "dependencies": [...],
  "overrides": [...]
}
```

### Compilation Flags

#### Rust
```bash
# Build command and flags
cargo build --release
# Specific RUSTFLAGS if any
RUSTFLAGS="..." 
```

#### C++
```bash
# CMake configuration
cmake -DCMAKE_BUILD_TYPE=Release ...
# Compiler flags
CXX_FLAGS="..."
```

## Test Configuration
- **Dataset**: [filename, rows, size]
- **Iterations**: [number of runs]
- **Warmup Cycles**: [number of warmup iterations]

## Results

### Summary Table

| Implementation | Average (ms) | Min (ms) | Max (ms) | Std Dev (ms) | Relative Performance |
|---------------|-------------|----------|----------|--------------|---------------------|
| arrow-rs      |             |          |          |              | 1.00x               |
| arrow-cpp     |             |          |          |              | X.XXx               |
| arrow2        |             |          |          |              | X.XXx               |

### Detailed Results

#### arrow-rs
```
[Run details with individual timings]
```

#### arrow2
```
[Run details with individual timings]
```

#### arrow-cpp
```
[Run details with individual timings]
```

## Notes
[Any additional observations, anomalies, or important context]