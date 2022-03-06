pub mod charabase {
    use anyhow::Context;
    use rand::prelude::IteratorRandom;
    use serde::{Deserialize, Serialize};
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

    #[derive(Deserialize)]
    pub struct CharaConfig {
        pub base: CharaBase,
        pub attack: Vec<CharaAttack>,
        pub meta: CharaMeta,
    }
    #[derive(Debug, Clone, Deserialize, Serialize)]
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

    #[derive(Deserialize)]
    pub struct CharaMeta {
        pub levelup_exp: LevelupExpType,
        pub species_type: SpeciesType,
        pub get_exp: u32,
        pub skill_type: u32,
    }

    #[derive(Deserialize)]
    pub enum LevelupExpType {
        Early,
        Normal,
        Late,
    }

    #[derive(Deserialize)]
    pub enum SpeciesType {
        Human { detailed_overview: String },
        Yokai { detailed_overview: String },
        Fairy { detailed_overview: String },
        Magician,
        Witch,
        Vampire,
        Yukionna,
        Shikigami,
        Poltergeists,
        HanzinHanrei, // Half-human half-phantom
        Ghost,
        Oni,
        NightSparrow,
        WereHakutaku,
        Rabbit { detailed_overview: String },
        Tengu { detailed_overview: String },
        Doll,
        Shinigami,
        Yamaxanadu,
        Kami { detailed_overview: String },
        Kappa,
        Tennin,
        TsurubeOtoshi,
        Tsuchigumo,
        Hashihime,
        Satori,
        Kasha,
        HellRaven,
        KarakasaObake,
        ShipGhost,
        Nue,
        Yamabiko,
        Jiangshi,
        Hermit,
        Taoist,
        Saint,
        Tanuki,
        Menreiki,
        Mermaid,
        Rokurokubi,
        Werewolf,
        Tsukumogami,
        Amanojaku,
        Kobito,
        Baku,
        DivineSpirit,
        Yamauba,
        Komainu,
        Soul,
        Jidiao,
        Haniwa,
        Kurokoma,
        Taotie,
        Manekineko,
        Yamawaro,
        KudaKitsune,
        Oomukade,
    }

    pub enum SkillType {
        Lucky(LuckyLevel),
        Effort,
    }

    pub enum LuckyLevel {
        LuckyOne,
        LuckyTwo,
        LuckyThree,
    }

    pub fn lucky_number(lucky_level: LuckyLevel) -> f32 {
        match lucky_level {
            LuckyLevel::LuckyOne => 1.1,
            LuckyLevel::LuckyTwo => 1.3,
            LuckyLevel::LuckyThree => 1.5,
        }
    }

    /// Amount of exp earned in battle
    /// Exp = 18 + (Enemy level*2 - my level) * {enemy appear}th boss (* lucky_number)
    ///
    pub fn math_exp(
        enemy_level: u32,
        my_level: u32,
        enemy_appear: u8,
        lucky_level: Option<LuckyLevel>,
    ) -> f32 {
        let mut base_exp = (18 + (enemy_level * 2 - my_level) * enemy_appear as u32) as f32;
        if let Some(l) = lucky_level {
            base_exp *= lucky_number(l)
        }

        base_exp as f32
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    pub struct BattleData {
        pub player_data: CharaBase,
        pub enemy_data: CharaBase,
        pub elapesd_turns: u32,
        pub start_time: chrono::prelude::DateTime<chrono::prelude::Local>,
    }
}
