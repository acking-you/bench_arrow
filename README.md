## Benche Crates

This repository is used for performance testing of different crates.

### Quick start
For example, to compare performance between `tokio` and `flume` channels, you need to perform the following two steps.
#### step 1
```shell
CRATE_NAME=tokio cargo bench --bench channel
```
got output:
```md
current crate:`tokio`
mpsc-string             time:   [32.463 ns 32.615 ns 32.763 ns]                         
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

spsc-string             time:   [25.269 ns 25.473 ns 25.704 ns]                         
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

mpsc-u8                 time:   [27.772 ns 28.140 ns 28.508 ns]                     
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

spsc-u8                 time:   [25.052 ns 25.212 ns 25.371 ns]                     
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
```
#### step 2

Switching to flume channel:
```shell
CRATE_NAME=flume cargo bench --bench channel
```
Comparing results between `tokio` and `flume`：
```md
current crate:`flume`
mpsc-string             time:   [33.039 ns 33.784 ns 34.707 ns]                         
                        change: [+0.8381% +2.1137% +3.6732%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

spsc-string             time:   [25.611 ns 25.851 ns 26.116 ns]                         
                        change: [-1.3032% +0.3346% +1.8407%] (p = 0.71 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

mpsc-u8                 time:   [27.856 ns 27.918 ns 27.990 ns]                     
                        change: [-0.2561% +0.6078% +1.4459%] (p = 0.17 > 0.05)
                        No change in performance detected.
Found 16 outliers among 100 measurements (16.00%)
  8 (8.00%) high mild
  8 (8.00%) high severe

spsc-u8                 time:   [25.177 ns 25.551 ns 26.035 ns]                     
                        change: [-0.2135% +1.2134% +2.7571%] (p = 0.11 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
```

### clickhouse-rs vs clickhouse.rs vs klickhouse vs grpc

[clickhouse-rs](https://crates.io/crates/clickhouse-rs)

[clickhouse.rs](https://crates.io/crates/clickhouse)

[klickhouse](https://crates.io/crates/klickhouse)

[grpc](https://clickhouse.com/docs/en/interfaces/grpc)
#### Pre-Requirements
You'll need a machine running clickhouse-server, and having completed ClickHouse's [Advanced Tutorial](https://clickhouse.com/docs/en/tutorial).

#### Result
run this command:
```shell
CRATE_NAME=kk-rs cargo bench --bench ch-sdk
CRATE_NAME=ck-rs cargo bench --bench ch-sdk
CRATE_NAME=ck.rs cargo bench --bench ch-sdk
CRATE_NAME=ck-grpc cargo bench --bench ch-sdk
```
And you got this result:

`grpc` > `clickhouse.rs` > `clickhouse-rs` >> `klickhouse`

```md
current crate:`kk-rs`
Benchmarking ck-client: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 15.1s, or reduce sample count to 30.
ck-client               time:   [146.08 ms 147.62 ms 149.20 ms]                      
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

current crate:`ck-rs`
ck-client               time:   [46.477 ms 46.618 ms 46.755 ms]                      
                        change: [-68.775% -68.420% -68.078%] (p = 0.00 < 0.05)
                        Performance has improved.

current crate:`ck.rs`
ck-client               time:   [29.507 ms 30.368 ms 31.486 ms]                      
                        change: [-36.737% -34.857% -32.346%] (p = 0.00 < 0.05)
                        Performance has improved.

current crate:`ck-grpc`
ck-client               time:   [20.229 ms 20.379 ms 20.548 ms]                      
                        change: [-35.334% -32.894% -30.911%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 23 outliers among 100 measurements (23.00%)
  1 (1.00%) low severe
  5 (5.00%) low mild
  2 (2.00%) high mild
  15 (15.00%) high severe

```


### arrow-parquet vs parquet2
The results show that parquet2 is not as good as it claims to be (synchronized reads), but this is of course related to the fact that parquet2 has almost stopped being maintained. From a usage point of view, parquet2 can be controlled at a much finer granularity than arrow-parquet (from the official repository samples)

```md
current crate:`p1`
Benchmarking parquet_read: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 40.0s, or reduce sample count to 10.
parquet_read            time:   [394.21 ms 397.03 ms 400.14 ms]   

current crate:`p2`
Benchmarking parquet_read: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 64.3s, or reduce sample count to 10.
parquet_read            time:   [656.69 ms 660.92 ms 665.27 ms]                         
                        change: [+64.835% +66.467% +68.098%] (p = 0.00 < 0.05)
                        Performance has regressed.
```