pub mod playdata;
pub mod score;
pub mod userdata;

pub mod redis_connect {
    use redis::aio::AsyncStream;

    pub type AsyncConnection =
        redis::aio::Connection<std::pin::Pin<Box<dyn AsyncStream + Sync + Send>>>;
    /// url format :
    /// "redis://<username>:<password>@<hostname>:<port>/<db>"
    pub async fn connect(url: impl Into<String>) -> redis::RedisResult<AsyncConnection> {
        let client = redis::Client::open(url.into()).expect("不正なURLです");
        let connect = client.get_tokio_connection().await?;
        Ok(connect)
    }
}

pub mod postgres_connect {
    /// url format:
    /// "postgres://<username>:<password>@<hostname>:<port>/<database>"
    pub async fn connect(
        url: impl Into<String>,
    ) -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr> {
        sea_orm::Database::connect(url.into().to_owned()).await
    }
}
