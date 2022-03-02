use crate::setting::setup::BOTInfo;
use axum::response;

pub async fn infomation() -> response::Json<BOTInfo> {
    response::Json(BOTInfo::info())
}
