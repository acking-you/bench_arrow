use clickhouse::{Client, Row};

use serde::{Deserialize, Serialize};
use time::Date as TimeDate;

use crate::clickhouse_sdk::{COUNT, SQL};

pub async fn execute_with_http(client: &Client) {
    #[derive(Debug, Row, Serialize, Deserialize)]
    struct MyRow {
        #[serde(with = "clickhouse::serde::time::date")]
        pickup_date: TimeDate,
    }
    let mut cursor = client.query(SQL).fetch::<MyRow>().expect("nerver fails");

    let mut dates = Vec::with_capacity(COUNT);
    while let Ok(Some(row)) = cursor.next().await {
        dates.push(row);
    }

    assert_eq!(dates.len(), COUNT);
}
