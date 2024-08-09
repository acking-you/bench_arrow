pub mod ck_http;
pub mod ck_tcp;
pub mod grpc;
pub mod kk_tcp;

static SQL: &str = "SELECT
   pickup_date 
FROM trips";

const COUNT: usize = 1999657;

#[cfg(test)]
mod tests {

    use ::clickhouse::Client;
    use ::clickhouse_rs::Pool;
    use ck_http::execute_with_http;
    use ck_tcp::execute_with_tcp_native;
    use grpc::{clickhouse_grpc::click_house_client::ClickHouseClient, execute_grpc};
    use kk_tcp::execute_with_klick;
    use klickhouse::{Client as KlickClient, ClientOptions};

    use crate::utils::Timer;

    use super::*;

    #[tokio::test]
    async fn test_native() {
        let client = Pool::new("tcp://localhost:9000");
        for _ in 0..100 {
            let _timer = Timer::new();
            execute_with_tcp_native(&client).await;
        }
    }

    #[tokio::test]
    async fn test_http() {
        let client = Client::default().with_url("http://localhost:8123");
        for _ in 0..100 {
            let _timer = Timer::new();
            execute_with_http(&client).await;
        }
    }

    #[tokio::test]
    async fn test_klick() {
        let client = KlickClient::connect("localhost:9000", ClientOptions::default())
            .await
            .expect("kk-rs connect nerver fails");
        for _ in 0..100 {
            let _timer = Timer::new();
            execute_with_klick(&client).await;
        }
    }

    #[tokio::test]
    async fn test_grpc() {
        let mut client = ClickHouseClient::connect("http://11.151.241.21:9100")
            .await
            .expect("ck-grpc connect nerver fails");
        for _ in 0..1 {
            let _timer = Timer::new();
            execute_grpc(&mut client).await;
        }
    }
}
