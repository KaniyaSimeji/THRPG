pub mod charabase {
    use anyhow::Context;
    use serde::Deserialize;
    use std::path::Path;
    use tokio::fs;

    pub(crate) async fn new_enemy<T: AsRef<Path>>(toml_path: T) -> anyhow::Result<CharaBase> {
        let chara_data = {
            let file_content = fs::read_to_string(toml_path)
                .await
                .context("ファイルを読み込めませんでした")?;

            toml::from_str(file_content.as_str())
        }
        .context("読み込めませんでした")?;

        Ok(chara_data)
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CharaBase {
        pub name: String,
        pub power: u8,
        pub guard: u8,
        pub speed: u8,
        pub hp: u8,
        pub mp: u8,
    }

    #[derive(Deserialize)]
    pub struct CharaAttack {
        pub name: String,
        pub damage: u32,
        pub hit_rate: f32, // 0.08 etc...
        pub abnormal_state: Option<AbnormalState>,
    }

    #[derive(Deserialize)]
    pub enum AbnormalState {
        Slowed,   // チルノとか
        Poisoned, //メディスンとか
        Unlucky,  // 鍵山雛とか
    }
}

pub mod chara_enemy {
    pub struct CharaEnemy {
        pub base: super::charabase::CharaBase,
        pub attack: Vec<super::charabase::CharaAttack>,
        pub exp: u32,
    }
}

pub mod chara_player {
    pub struct CharaPlayer {
        pub base: super::charabase::CharaBase,
        pub attack: Vec<super::charabase::CharaAttack>,
        pub required_exp: u32,
    }

    enum CharaSkill {}
}
