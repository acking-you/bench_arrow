use once_cell::sync::Lazy;

pub mod arrow;
pub mod channel;
pub mod clickhouse_sdk;
pub mod parquet;
pub mod utils;

pub static CRATE_NAME: Lazy<String> = Lazy::new(|| {
    let name =
        std::env::var("CRATE_NAME").expect("must set `CRATE_NAME` env to run this benchmark");
    println!("current crate:`{name}`");
    name
});
