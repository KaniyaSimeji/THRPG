use sea_orm::{
    sea_query::{tests_cfg::Char, ColumnDef, PostgresQueryBuilder, Table},
    ConnectionTrait, Schema,
};
use serenity::model::id::UserId;

pub struct SaveData {
    user_id: UserId,
    player: String,
    level: u32,
    exp: u32,
}

impl SaveData {
    pub fn new(user_id: UserId, player: String, level: u32, exp: u32) -> Self {
        SaveData {
            user_id,
            player,
            level,
            exp,
        }
    }

    pub fn save_db(content: SaveData, connect_db: sea_orm::DatabaseConnection) {
        Table::create()
            .table(Char::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Char::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .to_owned()
            .to_string(PostgresQueryBuilder);
    }
}
