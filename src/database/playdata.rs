use crate::battle::{builder::BattleBuilder, model::CharaConfig, rpg_core::PlayMode};
use chrono::NaiveDateTime;
use sea_orm::{entity::prelude::*, DeriveEntityModel};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "playdata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub battle_uuid: Uuid,
    pub player_name: String,
    pub enemy_name: String,
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

impl From<Model> for BattleBuilder {
    fn from(from: Model) -> BattleBuilder {
        BattleBuilder::new(
            PlayMode::try_from_value(&from.play_mode).unwrap(),
            Some(CharaConfig::chara_new_noasync(&from.player_name).unwrap()),
            Some(CharaConfig::chara_new_noasync(&from.enemy_name).unwrap()),
            Some(from.elapesd_turns),
        )
    }
}
