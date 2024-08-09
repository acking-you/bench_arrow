use bench_crates::{
    clickhouse_sdk::{
        ck_http::execute_with_http,
        ck_tcp::execute_with_tcp_native,
        grpc::{clickhouse_grpc::click_house_client::ClickHouseClient, execute_grpc},
        kk_tcp::execute_with_klick,
    },
    CRATE_NAME,
};
use clickhouse::Client;
use clickhouse_rs::Pool;
use criterion::{criterion_group, criterion_main, Criterion};
use klickhouse::{Client as KlickClient, ClientOptions};

use tokio::runtime::Runtime;

fn bench_sdk(c: &mut Criterion) {
    let name = CRATE_NAME.clone();
    let bench_id = "ck-client";

    match name.as_str() {
        "kk-rs" => {
            c.bench_function(bench_id, |b| {
                let runtime = Runtime::new().unwrap();
                let client = {
                    runtime.block_on(async move {
                        KlickClient::connect("localhost:9000", ClientOptions::default())
                            .await
                            .expect("kk-rs connect nerver fails")
                    })
                };
                b.to_async(runtime).iter(|| async {
                    execute_with_klick(&client).await;
                })
            });
        }
        "ck.rs" => {
            c.bench_function(bench_id, |b| {
                let client = Client::default().with_url("http://localhost:8123");
                b.to_async(Runtime::new().unwrap())
                    .iter(|| execute_with_http(&client))
            });
        }
        "ck-rs" => {
            c.bench_function(bench_id, |b| {
                let client = Pool::new("tcp://localhost:9000");
                b.to_async(Runtime::new().unwrap())
                    .iter(|| execute_with_tcp_native(&client))
            });
        }
        "ck-grpc" => {
            c.bench_function(bench_id, |b| {
                b.to_async(Runtime::new().unwrap()).iter(|| async move {
                    let mut client = ClickHouseClient::connect("http://127.0.0.1:9100")
                        .await
                        .expect("kk-rs connect nerver fails");
                    execute_grpc(&mut client).await;
                })
            });
        }
        _ => {
            unimplemented!("must be ck.rs/ck-rs/kk-rs")
        }
    }
}

criterion_group!(benches, bench_sdk);

criterion_main!(benches);
