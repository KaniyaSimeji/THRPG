pub mod charabase {
    use anyhow::Context;
    use rand::prelude::IteratorRandom;
    use serde::Deserialize;
    use std::path::{Path, PathBuf};

    pub(crate) fn read_enemy<T: AsRef<Path>>(toml_path: T) -> anyhow::Result<CharaBase> {
        let chara_data = {
            let file_content =
                std::fs::read_to_string(toml_path).context("ファイルを読み込めませんでした")?;

            toml::from_str(file_content.as_str())
        }
        .context("読み込めませんでした")?;

        Ok(chara_data)
    }

    // For example:
    // toml_dir_path : chara
    pub async fn random_enemy<T: AsRef<Path>>(
        toml_dir_path: T,
    ) -> Result<CharaBase, anyhow::Error> {
        if toml_dir_path.as_ref().is_dir() {
            let mut vec: Vec<PathBuf> = Vec::new();
            let mut entries = tokio::fs::read_dir(toml_dir_path)
                .await
                .context("ディレクトリが読み込めません")?;
            while let Some(entry) = entries.next_entry().await? {
                let dir_entrey = entry.path();

                vec.push(dir_entrey);
            }
            let mut rng = rand::thread_rng();
            let random_path = vec.iter().choose(&mut rng).unwrap().clone();
            let enemy = read_enemy(random_path);
            enemy
        } else {
            Err(anyhow::anyhow!("ディレクトリではありません"))
        }
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
        Slowed,
        Poisoned,
        Unlucky,
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
