use bench_crates::channel::use_flume_mpmc;
use bench_crates::channel::use_flume_spsc;
use bench_crates::channel::use_tokio_mpsc;
use bench_crates::channel::use_tokio_spsc;
use bench_crates::channel::DataProvider;
use bench_crates::channel::StringData;
use bench_crates::channel::U8Data;
use bench_crates::CRATE_NAME;
use criterion::{criterion_group, criterion_main, Criterion};

fn get_name<T: DataProvider>(name: &str) -> String {
    format!("{name}-{}", T::TYPE_LITERAL)
}

fn bench_start<R1, R2>(
    c: &mut Criterion,
    mpsc_name: &str,
    mpsc_fn: fn() -> R1,
    spsc_name: &str,
    spsc_fn: fn() -> R2,
) {
    c.bench_function(mpsc_name, |b| b.iter(mpsc_fn));
    c.bench_function(spsc_name, |b| b.iter(spsc_fn));
}

pub fn bench_channel<T: DataProvider>(c: &mut Criterion) {
    let name = CRATE_NAME.clone();

    let mpsc_name = get_name::<T>("mpsc");
    let spsc_name = get_name::<T>("spsc");

    // pre alloc
    let _a = T::get_random_data();

    match name.as_str() {
        "flume" => {
            bench_start(
                c,
                &mpsc_name,
                use_flume_mpmc::<T>,
                &spsc_name,
                use_flume_spsc::<T>,
            );
        }
        "tokio" => bench_start(
            c,
            &mpsc_name,
            use_tokio_mpsc::<T>,
            &spsc_name,
            use_tokio_spsc::<T>,
        ),
        _ => {
            unimplemented!("must be flume or tokio")
        }
    }
}

criterion_group!(
    benches,
    bench_channel::<StringData>,
    bench_channel::<U8Data>,
);

criterion_main!(benches);
