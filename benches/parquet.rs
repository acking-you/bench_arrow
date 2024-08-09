use bench_crates::{
    parquet::{sync_read_with_parquet, sync_read_with_parquet2},
    CRATE_NAME,
};
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_parquet_read(c: &mut Criterion) {
    let name = CRATE_NAME.clone();
    let bench_id = "parquet_read";

    match name.as_str() {
        "p1" => {
            c.bench_function(bench_id, |b| b.iter(sync_read_with_parquet));
        }
        "p2" => {
            c.bench_function(bench_id, |b| b.iter(sync_read_with_parquet2));
        }
        _ => {
            unimplemented!("must be p1/p2")
        }
    }
}

criterion_group!(benches, bench_parquet_read);

criterion_main!(benches);
