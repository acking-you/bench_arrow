use futures_util::StreamExt;
use klickhouse::Client as KlickClient;
use klickhouse::Date as KlickDate;
use klickhouse::Row as KlickRow;
use serde::{Deserialize, Serialize};

use crate::clickhouse_sdk::COUNT;
use crate::clickhouse_sdk::SQL;

pub async fn execute_with_klick(client: &KlickClient) {
    #[derive(Debug, KlickRow, Serialize, Deserialize)]
    struct MyRow {
        pickup_date: KlickDate,
    }
    let mut cursor = client.query::<MyRow>(SQL).await.expect("nerver fails");

    let mut dates = Vec::with_capacity(COUNT);
    while let Some(row) = cursor.next().await {
        let row = row.expect("never fails");
        dates.push(row);
    }

    assert_eq!(dates.len(), COUNT);
}
