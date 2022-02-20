use sea_orm::{entity::prelude::*, DeriveEntityModel};

//
// Make SeaORM entity
//
#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "userdata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id: u64,
    pub player: String,
    pub level: u32,
    pub exp: u32,
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

//
// Save program
//

pub async fn save(db: &DbConn, savedata: Model) {
    if let Some(userdata) = Entity::find_by_id(savedata.user_id).one(db).await.unwrap() {
        let mut userdata_mut: ActiveModel = userdata.into();
        userdata_mut.player = sea_orm::entity::Set(savedata.player);
        userdata_mut.level = sea_orm::entity::Set(savedata.level);
        userdata_mut.exp = sea_orm::entity::Set(savedata.exp);

        userdata_mut.update(db).await.unwrap();
    } else {
        let new_data = ActiveModel {
            user_id: sea_orm::ActiveValue::Set(savedata.user_id),
            player: sea_orm::ActiveValue::Set(savedata.player),
            level: sea_orm::ActiveValue::Set(savedata.level),
            exp: sea_orm::ActiveValue::Set(savedata.exp),
        };

        new_data.save(db).await.unwrap();
    }
}
