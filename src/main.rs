mod parquet;

use std::fmt;
use std::time::{Duration, Instant};

struct BenchmarkResult {
    execution_times: Vec<Duration>,
    avg_time: Duration,
}

impl fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format individual run times
        for (i, duration) in self.execution_times.iter().enumerate() {
            write!(f, "[{:02}] Duration: {:?}", i + 1, duration)?;

            // Highlight significant outliers (>150% of average)
            if duration > &(self.avg_time.mul_f32(1.5)) {
                write!(f, " ⚠ (outlier)")?;
            }
            writeln!(f)?;
        }

        // Format average time
        write!(f, "Average:   {:?}", self.avg_time)?;

        // Include total duration
        write!(
            f,
            " | Total: {:?}",
            self.execution_times.iter().sum::<Duration>()
        )
    }
}

/// Measures execution time of a function
///
/// # Arguments
/// - `func`: Function to benchmark (must implement `FnMut`)
/// - `runs`: Number of times to execute the function
/// - `warmup`: Optional warmup iterations before timing starts
///
/// # Example
/// ```
/// let test_fn = || { /* code to test */ };
/// let result = benchmark(test_fn, 100, Some(5));
/// println!("{}", result);
/// ```
fn benchmark<F>(mut func: F, runs: usize, warmup: Option<usize>) -> BenchmarkResult
where
    F: FnMut(),
{
    let warmup_iter = warmup.unwrap_or(3);

    // Warmup phase (untimed)
    for _ in 0..warmup_iter {
        func();
    }

    let mut execution_times = Vec::with_capacity(runs);
    let mut total_duration = Duration::ZERO;

    for _ in 0..runs {
        let start = Instant::now();
        func();
        let elapsed = start.elapsed();

        execution_times.push(elapsed);
        total_duration += elapsed;
    }

    // Calculate average with safety checks
    let avg_time = total_duration.checked_div(runs as u32).unwrap_or_default();

    BenchmarkResult {
        execution_times,
        avg_time,
    }
}

// Demo usage
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run --release <parquet|parquet2>");
        return;
    }

    let bench_type = args[1].to_lowercase();
    let bench_fn = if bench_type == "parquet2" {
        parquet::sync_read_with_parquet2
    } else if bench_type == "parquet" {
        parquet::sync_read_with_parquet
    } else {
        eprintln!(
            "error: Invalid benchmark type '{}'. Use 'parquet' or 'parquet2'.",
            bench_type
        );
        return;
    };

    // Run benchmark: 10 test runs with 2 warmup cycles
    let results = benchmark(bench_fn, 10, Some(2));
    println!("Benchmark Results:\n{}", results);

    // Outlier warning
    if results
        .execution_times
        .iter()
        .any(|t| t > &(results.avg_time.mul_f32(1.5)))
    {
        println!("\n⚠  Outliers detected - consider increasing warmup cycles");
    }
}
