use clickhouse_grpc::{click_house_client::ClickHouseClient, QueryInfo};
use tokio_stream::StreamExt;
use tonic::{transport::Channel, Request};

use crate::{
    clickhouse_sdk::COUNT,
    utils::arrow_stream_reader::{ArrowStreamReader, CursorReader},
};

use super::SQL;

pub mod clickhouse_grpc {
    tonic::include_proto!("clickhouse.grpc");
}

fn get_default_req() -> Request<QueryInfo> {
    Request::new(QueryInfo {
        query: SQL.to_string(),
        query_id: Default::default(),
        settings: Default::default(),
        database: Default::default(),
        input_data: Default::default(),
        input_data_delimiter: Default::default(),
        output_format: "ArrowStream".to_string(),
        send_output_columns: Default::default(),
        external_tables: Default::default(),
        user_name: Default::default(),
        password: Default::default(),
        quota: Default::default(),
        jwt: Default::default(),
        session_id: Default::default(),
        session_check: Default::default(),
        session_timeout: Default::default(),
        cancel: Default::default(),
        next_query_info: Default::default(),
        input_compression_type: Default::default(),
        output_compression_type: Default::default(),
        output_compression_level: Default::default(),
        transport_compression_type: Default::default(),
        transport_compression_level: Default::default(),
        obsolete_result_compression: Default::default(),
        obsolete_compression_type: Default::default(),
    })
}

pub async fn execute_grpc(client: &mut ClickHouseClient<Channel>) {
    let request = get_default_req();
    let mut stream = client
        .execute_query_with_stream_output(request)
        .await
        .unwrap()
        .into_inner();
    let ret = if let Some(Ok(ret)) = stream.next().await {
        ret
    } else {
        panic!("first retuslt must not be empty");
    };
    let mut stream_reader = ArrowStreamReader::try_new(CursorReader::new(ret.output))
        .expect("construct arrow stream reader nerver fails");
    let mut count = 0;
    for rb in stream_reader.by_ref() {
        let rb = rb.expect("got rb never fails");
        count += rb.num_rows();
    }
    while let Some(ret) = stream.next().await {
        let ret = ret.expect("got value nerver fails");
        assert!(stream_reader.need_update_reader());
        stream_reader.update_reader(CursorReader::new(ret.output));
        for rb in stream_reader.by_ref() {
            let rb = rb.expect("got rb never fails");
            count += rb.num_rows();
        }
    }
    assert_eq!(count, COUNT);
}
