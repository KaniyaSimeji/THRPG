use once_cell::sync::Lazy;
use serde::Deserialize;
use setting_config::Languages;
use std::path::PathBuf;

static JAPANESE_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("i18n/ja.toml"));
static ENGLISH_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("i18n/en.toml"));

#[derive(Deserialize)]
pub struct Bottexts {
    pub log_message: LogMessage,
    pub author_info_embed: InfoEmbed,
    pub game_message: GameMessage,
    pub enemy_description: EnemyDescription,
}

#[derive(Deserialize)]
pub struct LogMessage {
    pub bot_start_connect: String,
    pub not_found_token: String,
    pub invalid_token: String,
    pub invalid_redis_url: String,
    pub invalid_postgres_url: String,
    pub can_not_connect_redis: Option<String>,
    pub can_not_connect_postgres: Option<String>,
    pub can_not_read_file: String,
    pub toml_parse_error: String,
    pub invalid_chara: String,
    pub postgresql_execution_error: String,
    pub postgresql_record_not_found: String,
    pub make_embed_error: String,
}

#[derive(Deserialize)]
pub struct InfoEmbed {
    pub project_info: String,
    pub author: Vec<String>,
}

#[derive(Deserialize)]
pub struct GameMessage {
    pub appear_enemy: String,
    pub enemy_description: String,
    pub reaction_operation: String,
    pub battle_result: String,
    pub took_damage: String,
}

#[derive(Deserialize)]
pub struct EnemyDescription {
    pub sakuya_name: String,
    pub sakuya_description: String,
    pub reimu_name: String,
    pub reimu_description: String,
    pub marisa_name: String,
    pub marisa_description: String,
}

pub fn i18n_text(language: Languages) -> Bottexts {
    let ja_contents = std::fs::read_to_string(&*JAPANESE_PATH).unwrap();
    let en_contents = std::fs::read_to_string(&*ENGLISH_PATH).unwrap();

    match language {
        Languages::Japanese => {
            let japanese_toml: Bottexts = toml::from_str(&ja_contents).expect("Not read toml");
            japanese_toml
        }
        Languages::English => {
            let english_toml: Bottexts = toml::from_str(&en_contents).expect("Not read toml");
            english_toml
        }
    }
}
