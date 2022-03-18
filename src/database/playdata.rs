use chrono::NaiveDateTime;
use sea_orm::{entity::prelude::*, DeriveEntityModel};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "playdata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub battle_id: Uuid,
    pub player_name: String,
    pub enemy_name: String,
    pub elapesd_turns: u32,
    pub start_time: NaiveDateTime,
    pub start_turn: u32,
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Relation {
    Fruit,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            &Self::Fruit => Entity::has_many(Entity).into(),
        }
    }
}

impl Related<Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Fruit.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
