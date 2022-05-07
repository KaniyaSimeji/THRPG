use sea_orm::{entity::prelude::*, DeriveEntityModel};
use uuid::Uuid;
//
// Make SeaORM entity
//
#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "userdata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: String,
    pub player: String,
    pub level: i64,
    pub exp: i64,
    pub battle_uuid: Option<Uuid>,
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
