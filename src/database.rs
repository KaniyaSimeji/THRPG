pub mod save;
pub mod score;

pub mod redis_connect {
    // url format :
    // "redis://<username>:<password>@<hostname>:<port>/<db>"
    pub fn connect(url: impl Into<String>) -> redis::RedisResult<redis::Connection> {
        let client = redis::Client::open(url.into())?;

        let con = client.get_connection()?;

        Ok(con)
    }
}

pub mod postgres_connect {
    // url format:
    // "postgres://<username>:<password>@<hostname>:<port>/<database>"
    pub async fn connect(
        url: impl Into<String>,
    ) -> Result<sea_orm::DatabaseConnection, sea_orm::DbErr> {
        sea_orm::Database::connect(url.into()).await
    }
}
