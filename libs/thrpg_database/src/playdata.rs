use chrono::NaiveDateTime;
use sea_orm::{entity::prelude::*, DeriveEntityModel};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "playdata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub battle_uuid: Uuid,
    pub player: serde_json::Value,
    pub enemy: serde_json::Value,
    pub elapesd_turns: u32,
    pub start_time: NaiveDateTime,
    pub play_mode: String,
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Relation {
    PlayData,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            &Self::PlayData => Entity::has_many(Entity).into(),
        }
    }
}

impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PlayData.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
