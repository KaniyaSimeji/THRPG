use crate::battle::model::CharaConfig;
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

impl From<Model> for CharaConfig {
    fn from(from: Model) -> CharaConfig {
        let frame = CharaConfig::chara_new_noasync(&from.player).unwrap();
        frame
    }
}

impl From<&Model> for CharaConfig {
    fn from(from: &Model) -> CharaConfig {
        let frame = CharaConfig::chara_new_noasync(&from.player).unwrap();
        frame
    }
}

//
// Save program
//

pub async fn save(db: &DbConn, savedata: Model) {
    let user_id_string = &savedata.user_id;
    if let Some(userdata) = Entity::find_by_id(user_id_string.to_string())
        .one(db)
        .await
        .unwrap()
    {
        let mut userdata_mut: ActiveModel = userdata.into();
        userdata_mut.player = sea_orm::entity::Set(savedata.player);
        userdata_mut.level = sea_orm::entity::Set(savedata.level);
        userdata_mut.exp = sea_orm::entity::Set(savedata.exp);
        userdata_mut.battle_uuid = sea_orm::entity::Set(savedata.battle_uuid);

        userdata_mut.update(db).await.unwrap();
    } else {
        let new_data = ActiveModel {
            user_id: sea_orm::ActiveValue::Set(savedata.user_id),
            player: sea_orm::ActiveValue::Set(savedata.player),
            level: sea_orm::ActiveValue::Set(savedata.level),
            exp: sea_orm::ActiveValue::Set(savedata.exp),
            battle_uuid: sea_orm::ActiveValue::Set(savedata.battle_uuid),
        };

        new_data.insert(db).await.unwrap();
    }
}

pub async fn delete(db: &DbConn, user_id: u64) {
    if let Some(userdata) = Entity::find_by_id(user_id.to_string())
        .one(db)
        .await
        .unwrap()
    {
        userdata.delete(db).await.unwrap();
    }
}

pub async fn update_player(db: &DbConn, user_id: u64, player: String) {
    let userdata: Option<Model> = Entity::find_by_id(user_id.to_string())
        .one(db)
        .await
        .unwrap();

    if let Some(data) = userdata {
        let mut active_userdata: ActiveModel = data.into();

        active_userdata.player = sea_orm::entity::Set(player);

        active_userdata.update(db).await.unwrap();
    } else {
        let active_model: ActiveModel = Model {
            user_id: user_id.to_string(),
            player,
            level: 0,
            exp: 0,
            battle_uuid: None,
        }
        .into();
        active_model.insert(db).await.unwrap();
    }
}
