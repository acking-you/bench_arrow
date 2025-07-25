#include <arrow/api.h>
#include <arrow/io/api.h>
#include <chrono>
#include <iomanip>
#include <iostream>
#include <numeric>
#include <parquet/arrow/reader.h>
#include <parquet/exception.h>
#include <vector>

using namespace std;
using namespace chrono;

struct BenchmarkResult {
    vector<duration<double, nano>> execution_times;
    duration<double, nano> avg_time;
};

ostream &operator<<(ostream &os, const BenchmarkResult &result) {
    // Format individual run times
    for (size_t i = 0; i < result.execution_times.size(); ++i) {
        auto duration = result.execution_times[i];

        os << "[" << setfill('0') << setw(2) << (i + 1) << "] Duration: ";
        if (duration < 1ms) {
            os << setw(6) << fixed << setprecision(3) << duration.count() / 1000.0 << " μs";
        } else {
            os << setw(6) << fixed << setprecision(3) << duration.count() / 1e6 << " ms";
        }

        // Mark outliers (>150% of average)
        if (duration > result.avg_time * 1.5) {
            os << " ⚠ (outlier)";
        }
        os << endl;
    }

    // Format average time
    os << "Average:   ";
    if (result.avg_time < 1ms) {
        os << setw(8) << fixed << setprecision(3) << result.avg_time.count() / 1000.0 << " μs";
    } else {
        os << setw(8) << fixed << setprecision(3) << result.avg_time.count() / 1e6 << " ms";
    }

    // Calculate and display total time
    double total_time = accumulate(result.execution_times.begin(), result.execution_times.end(), 0.0,
                                   [](double acc, const auto &d) { return acc + d.count(); }) /
                        1e6; // Convert to milliseconds

    os << " | Total: " << fixed << setprecision(3) << total_time << " ms";
    return os;
}

/**
 * Benchmark a function's execution time
 *
 * @tparam F Callable type
 * @param func Function to benchmark
 * @param runs Number of test runs
 * @param warmup Warmup iterations (optional)
 * @return BenchmarkResult with detailed timing data
 */
template<typename F>
BenchmarkResult benchmark(F func, size_t runs, optional<size_t> warmup = nullopt) {
    // Default warmup is 3 iterations
    size_t warmup_count = warmup.value_or(3);

    // Warmup phase (untimed)
    for (size_t i = 0; i < warmup_count; ++i) {
        func();
    }

    vector<duration<double, nano>> execution_times;
    execution_times.reserve(runs);

    duration<double, nano> total_time = 0ns;

    for (size_t i = 0; i < runs; ++i) {
        auto start = high_resolution_clock::now();
        func();
        auto end = high_resolution_clock::now();

        auto elapsed = duration_cast<duration<double, nano>>(end - start);
        execution_times.push_back(elapsed);
        total_time += elapsed;
    }

    duration<double, nano> avg_time = total_time / runs;

    return BenchmarkResult{.execution_times = execution_times, .avg_time = avg_time};
}

// Prevent compiler optimization of unused results
template<typename T>
void do_not_optimize(const T &value) {
    asm volatile("" : : "r,m"(value) : "memory");
}

void read_whole_file() {
    std::cout << "Reading parquet-arrow-example.parquet at once" << std::endl;
    std::shared_ptr<arrow::io::ReadableFile> infile;
    PARQUET_ASSIGN_OR_THROW(infile, arrow::io::ReadableFile::Open(BENCH_DATA_PATH, arrow::default_memory_pool()));

    std::unique_ptr<parquet::arrow::FileReader> reader;
    PARQUET_ASSIGN_OR_THROW(reader, parquet::arrow::OpenFile(infile, arrow::default_memory_pool()));
    std::shared_ptr<arrow::Table> table;
    PARQUET_THROW_NOT_OK(reader->ReadTable(&table));
    std::cout << "Loaded " << table->num_rows() << " rows in " << table->num_columns() << " columns." << std::endl;
}

int main() {

    // Run benchmark: 10 test runs with 2 warmup cycles
    auto results = benchmark(read_whole_file, 10, 2);

    cout << "Benchmark Results:\n" << results << endl;

    // Check for outliers
    bool has_outliers =
            ranges::any_of(results.execution_times, [&](const auto &t) { return t > results.avg_time * 1.5; });

    if (has_outliers) {
        cout << "\n⚠  Outliers detected - consider increasing warmup cycles" << endl;
    }

    return 0;
}
