use chrono::NaiveDate;
use clickhouse_rs::Pool;
use futures_util::StreamExt;

use crate::clickhouse_sdk::COUNT;
use crate::clickhouse_sdk::SQL;

pub async fn execute_with_tcp_native(pool: &Pool) {
    let mut client = pool.get_handle().await.expect("got client nerver fails");
    let mut stream = client.query(SQL).stream();

    let mut dates = Vec::with_capacity(COUNT);
    while let Some(row) = stream.next().await {
        let row = row.expect("got row never fails");
        let value: NaiveDate = row.get("pickup_date").unwrap();
        dates.push(value);
    }
    assert_eq!(dates.len(), COUNT);
}
