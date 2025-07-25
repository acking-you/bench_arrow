## Arrow-rs VS Arrow-cpp

## Rarquet reader

Cpp source code:[parquet-reader](./bench-cpp/main.cpp)

Rust source code:[parquet-reader](./src/parquet/mod.rs)

How to run this benchmark?
1. `cargo run --release parquet` We can got arrow-rs result
2. `cargo run --release parquet2` We can got arrow2 result
2. `cd bench-cpp && sh pre_build.sh && sh run.sh` We can got arrow-cpp result

### Bench result

Test environment:
CPU: 32C 
MEM: 64GB 
DISK: Delay 500 microseconds, bandwidth 150MB/s

The following is a comparison table of the average time consumed by three Arrow implementation versions:

| Implement version | avg(ms) | comparison    | relative performance |
| ----------------- | ------- | ------------- | -------------------- |
| **arrow-rs**      | 850.08  | ★★★ (fastest) | 1.00x                |
| arrow-cpp         | 1358.71 | ★★☆           | 1.60x                |
| arrow2            | 1609.19 | ★☆☆           | 1.89x                |


arrow-rs:
```sh
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

arrow2:
```sh
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

arrow-cpp:
```sh
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