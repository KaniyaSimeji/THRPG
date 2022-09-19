use extension::extension_config::ExtensionConfig;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct OwnerInfo {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserInfo {
    pub name: String,
}

pub type BOTExtensions = Vec<ExtensionConfig>;

#[derive(Deserialize, Serialize)]
pub struct Ranking {}

#[derive(Deserialize, Serialize)]
pub struct Rankings {}

#[derive(Deserialize, Serialize)]
pub enum RankingType {
    Social,
    Local,
}

#[derive(Deserialize, Serialize)]
pub struct UserRankings {}

#[derive(Deserialize, Serialize)]
pub struct UserRanking {}

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
