use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Default server address 
/// check: [address_random](address_random())
pub const NULL_ADDRESS: &str = "null.address";

/// THRPG.toml params
/// example:
/// ```toml
/// token="your bot token"
/// prefix="th!" # your bot prefix
/// manager_id= #your userid
/// language="Japanese" # used language in your bot 
///
/// [postgresql_config]
/// db_address="postgres://postgres@localhost/thrpg" # postgresql server address
///
/// [redis_config]
/// db_address="" # redis server address
/// ```
#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    token: String,
    prefix: Option<String>,
    server_address: Option<String>,
    redis_config: Option<RedisConfig>,
    postgresql_config: PostgresqlConfig,
    manager_id: u64,
    language: Option<String>,
    timeout_duration: Option<u64>,
    authority_flags: Option<u32>,
    authority_strict: Option<bool>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RedisConfig {
    pub db_address: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PostgresqlConfig {
    pub db_address: String,
}

pub async fn config_parse_toml() -> Config {
    // Setting file is "THRPG.toml"
    let path = PathBuf::from("THRPG.toml");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    let toml: Config = toml::from_str(content.as_str()).unwrap();
    toml
}

/// format is {uuid}.null.address
pub fn address_random() -> String {
    let url_string = format!("{}.{}", uuid::Uuid::new_v4(), NULL_ADDRESS.to_string());
    url::Url::parse(&url_string).unwrap().into()
}

impl Config {
    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn server_address(&self) -> Option<&str> {
        self.server_address.as_ref().map(|string| string as &str)
    }
    pub fn redis_config(&self) -> Option<&RedisConfig> {
        self.redis_config.as_ref()
    }

    pub fn postgresql_config(&self) -> &PostgresqlConfig {
        &self.postgresql_config
    }

    pub fn manager_id(&self) -> u64 {
        self.manager_id
    }

    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_ref().map(|string| string as &str)
    }

    pub fn check_server_address(&self) -> anyhow::Result<String> {
        if let Some(url) = &self.server_address {
            let url = url::Url::parse(&url).map_err(|e| anyhow::anyhow!(e))?;
            let string: String = url.into();
            Ok(string)
        } else {
            Err(anyhow::anyhow!("No server address"))
        }
    }

    pub fn timeout_duration(&self) -> Option<u64> {
        self.timeout_duration
    }

    pub fn authority_flags(&self) -> Option<u32> {
        self.authority_flags
    }

    pub fn authority_strict(&self) -> bool {
        if let Some(ans) = self.authority_strict {
            ans
        } else {
            false
        }
    }
}
