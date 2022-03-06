use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct OwnerInfo {
    pub name: String,
}
