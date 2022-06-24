use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const NULL_ADDRESS: &str = "null.address";
/// THRPG.toml params
#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    token: String,
    prefix: Option<String>,
    server_address: Option<String>,
    redis_config: Option<RedisConfig>,
    postgresql_config: PostgresqlConfig,
    manager_id: u64,
    language: Option<Languages>,
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

#[derive(Deserialize, Serialize, Clone)]
pub enum Languages {
    Japanese,
    English,
}

#[derive(Deserialize, Serialize)]
pub struct BOTInfo {
    pub name: String,
    pub author: String,
    pub version: String,
    pub website: String,
    pub repository: String,
    pub license: String,
}

impl BOTInfo {
    pub fn info() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME").to_string(),
            author: env!("CARGO_PKG_AUTHORS").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            website: env!("CARGO_PKG_HOMEPAGE").to_string(),
            repository: env!("CARGO_PKG_REPOSITORY").to_string(),
            license: env!("CARGO_PKG_LICENSE").to_string(),
        }
    }
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
    pub fn token(self) -> String {
        self.token
    }

    pub fn server_address(&self) -> Option<&String> {
        self.server_address.as_ref()
    }
    pub fn redis_config(&self) -> Option<&RedisConfig> {
        self.redis_config.as_ref()
    }

    pub fn postgresql_config(&self) -> PostgresqlConfig {
        self.postgresql_config.clone()
    }

    pub fn manager_id(&self) -> u64 {
        self.manager_id
    }

    pub fn prefix(&self) -> Option<&String> {
        self.prefix.as_ref()
    }

    pub fn check_server_address(self) -> anyhow::Result<String> {
        if let Some(url) = self.server_address {
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
