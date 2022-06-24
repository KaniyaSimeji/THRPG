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
