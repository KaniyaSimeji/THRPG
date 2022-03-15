pub mod charabase {
    use anyhow::Context;
    use chrono::prelude::{DateTime, Local};
    use once_cell::sync::Lazy;
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

    pub async fn dir_files<T: AsRef<Path>>(
        toml_dir_path: T,
    ) -> Result<Vec<CharaBase>, anyhow::Error> {
        if toml_dir_path.as_ref().is_dir() {
            let mut vec: Vec<PathBuf> = Vec::new();
            let mut entries = tokio::fs::read_dir(toml_dir_path)
                .await
                .context("ディレクトリが読み込めません")?;
            while let Some(entry) = entries.next_entry().await? {
                let dir_entrey = entry.path();

                vec.push(dir_entrey);
            }

            vec.iter().map(read_enemy).collect()
        } else {
            Err(anyhow::anyhow!("not directory"))
        }
    }

    // For example:
    // toml_dir_path : chara
    pub async fn random_enemy<T: AsRef<Path>>(
        toml_dir_path: T,
    ) -> Result<CharaBase, anyhow::Error> {
        let files = dir_files(toml_dir_path).await.unwrap();
        let mut rng = rand::thread_rng();
        let random_enemy = files.iter().choose(&mut rng).unwrap().clone();
        Ok(random_enemy)
    }

    #[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
    pub struct CharaConfig {
        pub base: CharaBase,
        pub attack: Vec<CharaAttack>,
        pub meta: CharaMeta,
    }
    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CharaBase {
        pub name: String,
        pub power: u8,
        pub guard: u8,
        pub speed: u8,
        pub hp: u8,
        pub mp: u8,
    }

    impl CharaBase {
        pub async fn chara_new(name_arg: String) -> anyhow::Result<CharaBase> {
            static REIMU_REGEX: Lazy<regex::Regex> =
                Lazy::new(|| regex::Regex::new(r"(?i)(^(h|H)+akurei)?(r|R)eimu$").unwrap());
            static SAKUYA_REGEX: Lazy<regex::Regex> =
                Lazy::new(|| regex::Regex::new(r"(?i)(^(i|I)+zayoi)?(s|S)akuya$").unwrap());
            static MARISA_REGEX: Lazy<regex::Regex> =
                Lazy::new(|| regex::Regex::new(r"(?i)(^(k|K)+irisame)?(m|M)arisa$").unwrap());

            let reimu_parse = REIMU_REGEX.replace(&name_arg, "Reimu").into_owned();
            let _: anyhow::Result<CharaBase> = match reimu_parse.as_str() {
                "Reimu" => {
                    let chara_datas = dir_files("chara").await.unwrap();
                    let reimu_data = chara_datas
                        .into_iter()
                        .find(|f| f.name == "博麗霊夢")
                        .context("Not found")?;
                    Ok(reimu_data)
                }
                _ => Err(anyhow::anyhow!(
                    "No regex {:?} {:?}",
                    *REIMU_REGEX,
                    &name_arg
                )),
            };

            let marisa_parse = MARISA_REGEX.replace(&name_arg, "Marisa").into_owned();
            let _: anyhow::Result<CharaBase> = match marisa_parse.as_str() {
                "Marisa" => {
                    let chara_datas = dir_files("chara").await.unwrap();
                    let marisa_data = chara_datas
                        .into_iter()
                        .find(|f| f.name == "霧雨魔理沙")
                        .context("Not found")?;
                    Ok(marisa_data)
                }
                _ => Err(anyhow::anyhow!(
                    "No regex {:?} {:?}",
                    *MARISA_REGEX,
                    &name_arg
                )),
            };

            let sakuya_parse = SAKUYA_REGEX.replace(&name_arg, "Sakuya").into_owned();
            match sakuya_parse.as_str() {
                "Sakuya" => {
                    let chara_datas = dir_files("chara").await.unwrap();
                    let sakuya_data = chara_datas
                        .into_iter()
                        .find(|f| f.name == "十六夜咲夜")
                        .context("Not found")?;
                    Ok(sakuya_data)
                }
                _ => Err(anyhow::anyhow!(
                    "No regex {:?} {:?}",
                    *SAKUYA_REGEX,
                    &name_arg
                )),
            }
        }
    }

    #[derive(Deserialize, PartialEq, PartialOrd, Debug, Clone)]
    pub struct CharaAttack {
        pub name: String,
        pub damage: u32,
        pub hit_rate: f32, // 0.08 etc...
        pub abnormal_state: Option<AbnormalState>,
    }

    #[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
    pub enum AbnormalState {
        Slowed,
        Poisoned,
        Unlucky,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct CharaMeta {
        pub levelup_exp: LevelupExpType,
        pub species_type: SpeciesType,
        pub get_exp: u32,
        pub skill_type: u32,
    }

    #[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
    pub enum LevelupExpType {
        Early,
        Normal,
        Late,
    }

    #[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
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
        player_level: u32,
        enemy_appear: u8,
        lucky_level: Option<LuckyLevel>,
    ) -> f32 {
        let mut base_exp = (18 + (enemy_level * 2 - player_level) * enemy_appear as u32) as f32;
        if let Some(l) = lucky_level {
            base_exp *= lucky_number(l)
        }

        base_exp as f32
    }

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
    pub struct BattleData {
        pub player_data: CharaBase,
        pub enemy_data: CharaBase,
        pub enemy_damage: u32,
        pub elapesd_turns: u32,
        pub start_time: DateTime<Local>,
        pub start_turn: u32,
    }

    /// player -> enemy
    pub fn calculate_player_damage(mut enemy: CharaConfig, player: &CharaConfig) -> CharaConfig {
        let mut rng = rand::thread_rng();
        let player_attack = player.attack.iter().choose(&mut rng).unwrap();
        let to_enemy_damage = enemy.base.hp - player_attack.damage as u8;
        enemy.base.hp = to_enemy_damage;
        enemy
    }

    /// enemy -> player
    pub fn calculate_enemy_damage(enemy: &CharaConfig, mut player: CharaConfig) -> CharaConfig {
        let mut rng = rand::thread_rng();
        let enemy_attack = enemy.attack.iter().choose(&mut rng).unwrap();
        let to_player_damage = *&mut player.base.hp - enemy_attack.damage as u8;
        player.base.hp = to_player_damage;
        player
    }

    pub fn turn<'a>(player: &'a CharaConfig, enemy: &'a CharaConfig) -> Vec<&'a CharaConfig> {
        let mut vec: Vec<&CharaConfig> = Vec::new();
        if player.base.speed >= enemy.base.speed {
            vec.push(player);
            vec.push(enemy);
        } else {
            vec.push(enemy);
            vec.push(player);
        };
        vec
    }
}
