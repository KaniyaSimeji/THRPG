mod model;

use crate::setting::setup::{config_parse_toml, BOTInfo};
use axum::response;

/// GET : {url}/information
pub async fn information() -> response::Json<BOTInfo> {
    response::Json(BOTInfo::info())
}

/// GET : {url}/owner
pub async fn owner() -> response::Json<model::OwnerInfo> {
    let user = serenity::model::id::UserId::from(config_parse_toml().await.manager_id());
    let token = config_parse_toml().await.token();
    let http = serenity::http::client::Http::new_with_token(&token);
    response::Json(model::OwnerInfo {
        name: user.to_user(http).await.unwrap().name,
    })
}
