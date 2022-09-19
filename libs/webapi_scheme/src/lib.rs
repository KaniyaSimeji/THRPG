mod model;

use axum::response;
use extension::extension_config::{ExtensionConfig, Extensiontype};
use setting_config::{config_parse_toml};
use crate::model::BOTInfo;

/// GET : {url}/info
pub async fn bot_info() -> response::Json<BOTInfo> {
    response::Json(BOTInfo::info())
}

/// GET : {url}/owner
pub async fn owner() -> response::Json<model::OwnerInfo> {
    let user = serenity::model::id::UserId::from(config_parse_toml().await.manager_id());
    let token = config_parse_toml().await.token().to_owned();
    let http = serenity::http::client::Http::new(&token);
    response::Json(model::OwnerInfo {
        name: user.to_user(http).await.unwrap().name,
    })
}

/// GET {url}/user/:id
pub async fn user(id: u64) -> response::Json<model::UserInfo> {
    todo!()
}

/// GET {url}/extensions
pub async fn extensions(
    extension_type: Option<Extensiontype>,
    q: Option<String>,
) -> response::Json<model::BOTExtensions> {
    todo!()
}

/// GET {url}/extensions/:id
pub async fn extension(id: String) -> response::Json<ExtensionConfig> {
    todo!()
}

/// GET {url}/ranking
pub async fn rankings() -> response::Json<model::Rankings> {
    todo!()
}

/// GET {url}/ranking/:ranking_type
pub async fn ranking(ranking_type: model::RankingType) -> response::Json<model::Ranking> {
    todo!()
}

/// GET {url}/ranking/:user_id
pub async fn user_rankings(id: u64) -> response::Json<model::UserRankings> {
    todo!()
}

/// GET {url}/ranking/:ranking_type/:user_id
pub async fn user_ranking(
    ranking_type: model::RankingType,
    id: u64,
) -> response::Json<model::UserRanking> {
    todo!()
}
