use crate::chara::{CharaConfig, LevelupExpType, LuckyLevel, SkillType};
use crate::utils::{dir_files, dir_files_noasync};
use anyhow::Context;
use chrono::prelude::NaiveDateTime;
use once_cell::sync::Lazy;
use rand::prelude::IteratorRandom;
use serde::{Deserialize, Serialize};
use thrpg_database::userdata::Model;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub enum PlayMode {
    Simple,
    Raid,
    Story { id: String },
}

impl ToString for PlayMode {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl PlayMode {
    pub fn try_from_value(value: &str) -> anyhow::Result<Self> {
        match value {
            "Simple" => Ok(Self::Simple),
            "Raid" => Ok(Self::Raid),
            _ => Err(anyhow::anyhow!(format!("No match {}", value))),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Story { id: e } => e.as_str(),
            Self::Simple => "Simple",
            Self::Raid => "Raid",
        }
    }
    /// get story id
    pub fn story_id(&self) -> Option<&str> {
        match self {
            Self::Simple => None,
            Self::Raid => None,
            Self::Story { id: a } => Some(a),
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct BattleData {
    uuid: Uuid,
    player_data: CharaConfig,
    enemy_data: CharaConfig,
    play_mode: PlayMode,
    elapesd_turns: u32,
    start_time: NaiveDateTime,
    is_running: bool,
}

impl CharaConfig {
    pub async fn chara_new(name_arg: &str) -> anyhow::Result<Self> {
        static REIMU_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"(?i)(^(h|H)+akurei)?(r|R)eimu$").unwrap());
        static SAKUYA_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"(?i)(^(i|I)+zayoi)?(s|S)akuya$").unwrap());
        static MARISA_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"(?i)(^(k|K)+irisame)?(m|M)arisa$").unwrap());

        if let Some(_) = REIMU_REGEX.find(&name_arg) {
            let chara_datas = dir_files("chara").await.unwrap();
            let reimu_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "博麗霊夢")
                .context("Not found")?;
            Ok(reimu_data)
        } else if let Some(_) = SAKUYA_REGEX.find(&name_arg) {
            let chara_datas = dir_files("chara").await.unwrap();
            let sakuya_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "十六夜咲夜")
                .context("Not found")?;
            Ok(sakuya_data)
        } else if let Some(_) = MARISA_REGEX.find(&name_arg) {
            let chara_datas = dir_files("chara").await.unwrap();
            let marisa_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "霧雨魔理沙")
                .context("Not found")?;
            Ok(marisa_data)
        } else if &name_arg == &"博麗霊夢" {
            let chara_datas = dir_files_noasync("chara").unwrap();
            let reimu_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "博麗霊夢")
                .context("Not found")?;
            Ok(reimu_data)
        } else if &name_arg == &"十六夜咲夜" {
            let chara_datas = dir_files_noasync("chara").unwrap();
            let sakuya_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "十六夜咲夜")
                .context("Not found")?;
            Ok(sakuya_data)
        } else if &name_arg == &"霧雨魔理沙" {
            let chara_datas = dir_files_noasync("chara").unwrap();
            let marisa_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "霧雨魔理沙")
                .context("Not found")?;
            Ok(marisa_data)
        } else {
            Err(anyhow::anyhow!("No match regex {:?}", &name_arg))
        }
    }

    pub fn chara_new_noasync(name_arg: &str) -> anyhow::Result<Self> {
        static REIMU_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"(?i)(^(h|H)+akurei)?(r|R)eimu$").unwrap());
        static SAKUYA_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"(?i)(^(i|I)+zayoi)?(s|S)akuya$").unwrap());
        static MARISA_REGEX: Lazy<regex::Regex> =
            Lazy::new(|| regex::Regex::new(r"(?i)(^(k|K)+irisame)?(m|M)arisa$").unwrap());

        if let Some(_) = REIMU_REGEX.find(&name_arg) {
            let chara_datas = dir_files_noasync("chara").unwrap();
            let reimu_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "博麗霊夢")
                .context("Not found")?;
            Ok(reimu_data)
        } else if let Some(_) = SAKUYA_REGEX.find(&name_arg) {
            let chara_datas = dir_files_noasync("chara").unwrap();
            let sakuya_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "十六夜咲夜")
                .context("Not found")?;
            Ok(sakuya_data)
        } else if let Some(_) = MARISA_REGEX.find(&name_arg) {
            let chara_datas = dir_files_noasync("chara").unwrap();
            let marisa_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "霧雨魔理沙")
                .context("Not found")?;
            Ok(marisa_data)
        } else if &name_arg == &"博麗霊夢" {
            let chara_datas = dir_files_noasync("chara").unwrap();
            let reimu_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "博麗霊夢")
                .context("Not found")?;
            Ok(reimu_data)
        } else if &name_arg == &"十六夜咲夜" {
            let chara_datas = dir_files_noasync("chara").unwrap();
            let sakuya_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "十六夜咲夜")
                .context("Not found")?;
            Ok(sakuya_data)
        } else if &name_arg == &"霧雨魔理沙" {
            let chara_datas = dir_files_noasync("chara").unwrap();
            let marisa_data = chara_datas
                .into_iter()
                .find(|f| f.charabase.name == "霧雨魔理沙")
                .context("Not found")?;
            Ok(marisa_data)
        } else {
            Err(anyhow::anyhow!("No match regex {:?}", &name_arg))
        }
    }
}

impl From<Model> for CharaConfig {
    fn from(model: Model) -> Self {
        CharaConfig::chara_new_noasync(&model.player).unwrap()
    }
}

impl LuckyLevel {
    pub fn lucky_number(&self) -> f32 {
        match self {
            LuckyLevel::LuckyOne => 1.1,
            LuckyLevel::LuckyTwo => 1.3,
            LuckyLevel::LuckyThree => 1.5,
        }
    }
}
impl SkillType {
    pub fn lucky_level(&self) -> Option<&LuckyLevel> {
        match self {
            Self::Lucky { level: l } => Some(l),
            _ => None,
        }
    }
}

impl BattleData {
    pub fn new(
        uuid: Uuid,
        player_data: CharaConfig,
        enemy_data: CharaConfig,
        play_mode: PlayMode,
        start_time: NaiveDateTime,
        elapesd_turns: u32,
    ) -> Self {
        Self {
            uuid,
            player_data,
            enemy_data,
            play_mode,
            start_time,
            elapesd_turns,
            is_running: false,
        }
    }

    /// Advance the elapsed turn
    pub fn add_turn(&mut self) -> &mut Self {
        self.elapesd_turns += 1;
        self
    }

    pub fn reset_turn(&mut self) -> &mut Self {
        self.elapesd_turns = 0;
        self
    }

    fn _turn<'a>(player: &'a CharaConfig, enemy: &'a CharaConfig) -> Vec<&'a CharaConfig> {
        let mut vec: Vec<&CharaConfig> = Vec::new();
        if player.charabase.speed >= enemy.charabase.speed {
            vec.push(player);
            vec.push(enemy);
        } else {
            vec.push(enemy);
            vec.push(player);
        };
        vec
    }
    /// Functions tat manipulate turnsh
    pub fn turn(&self) -> &CharaConfig {
        let turn_info = Self::_turn(&self.player_data, &self.enemy_data);
        // If it exceeds the length of `vec`, it will return to the first element of the array due
        // to the` Cycle` type.
        // Therefore, it is unlikely that it will be `None` unless the contents of` Vec` are empty.
        let info = turn_info
            .into_iter()
            .cycle()
            .nth(self.elapesd_turns as usize)
            .unwrap();

        info
    }

    /// get player data
    pub fn player(&self) -> &CharaConfig {
        &self.player_data
    }

    /// get enemy data
    pub fn enemy(&self) -> &CharaConfig {
        &self.enemy_data
    }

    /// get uuid
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// player -> enemy
    /// The turn advances by 1 when this function is called
    pub fn calculate_player_damage(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();
        let player_attack = self.player_data.attack.iter().choose(&mut rng).unwrap();
        let to_enemy_damage = self.enemy_data.charabase.hp - player_attack.damage as i16;
        self.enemy_data.charabase.hp = to_enemy_damage;
        self.add_turn();
        self
    }

    /// enemy -> player
    /// The turn advances by 1 when this function is called
    pub fn calculate_enemy_damage(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();
        let enemy_attack = self.enemy_data.attack.iter().choose(&mut rng).unwrap();
        let to_player_damage = self.player_data.charabase.hp - enemy_attack.damage as i16;
        self.player_data.charabase.hp = to_player_damage;
        self.add_turn();
        self
    }

    /// Increase enemy defense
    /// The turn advances by 1 when this function is called
    pub fn guard_enemy_damage(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();
        let enemy_attack = self.enemy_data.attack.iter().choose(&mut rng).unwrap();
        self.player_data.charabase.hp +=
            enemy_attack.damage as i16 - self.player_data.charabase.guard;
        self.add_turn();
        self
    }

    /// Increase player defense
    /// The turn advances by 1 when this function is called
    pub fn guard_player_damage(&mut self) -> &mut Self {
        let mut rng = rand::thread_rng();
        let player_attack = self.player_data.attack.iter().choose(&mut rng).unwrap();
        self.enemy_data.charabase.hp +=
            player_attack.damage as i16 - self.enemy_data.charabase.guard;
        self.add_turn();
        self
    }

    pub fn elapesd_turns(&self) -> u32 {
        self.elapesd_turns
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn start_running(&mut self) -> &mut Self {
        self.is_running = true;
        self
    }

    pub fn finish_running(&mut self) -> &mut Self {
        self.is_running = false;
        self
    }

    pub async fn result_battle(&mut self) -> &Self {
        let turn = self.turn();
        let damage = if turn == &self.player_data {
            self.calculate_player_damage()
        } else {
            self.calculate_enemy_damage()
        };
        damage
    }

    pub async fn result_guard(&mut self) -> &Self {
        let turn = self.turn();
        let guard = if turn == &self.player_data {
            self.guard_player_damage()
        } else {
            self.guard_enemy_damage()
        };
        guard
    }

    /// Amount of exp earned in battle
    ///
    pub fn calculate_exp(&self, enemy_level: u32, player_level: u32) -> u32 {
        let enemy_level_exponentiation = |mut x: u32| {
            let mut num = 0;
            while x == 0 {
                num += x * x;
                x -= 1;
            }
            num
        };

        let mut base_exp = (self.enemy_data.meta.get_exp
            + rand::random::<u8>() as u32
            + (enemy_level_exponentiation(enemy_level) - player_level * enemy_level))
            as f32;

        if matches!(
            self.player_data.meta.skill_type,
            SkillType::Lucky { level: _ }
        ) {
            base_exp *= if let Some(l) = self.player_data.meta.skill_type.lucky_level() {
                l.lucky_number()
            } else {
                base_exp
            }
        }

        base_exp as u32
    }

    /// Find the player level from exp
    pub fn calculate_player_level(&self, exp: f64) -> f64 {
        match &self.player_data.meta.levelup_exp {
            LevelupExpType::Early => exp.cbrt().abs(),
            LevelupExpType::Normal => exp.cbrt().abs(),
            LevelupExpType::Late => exp.cbrt().abs(),
        }
    }

    /// Find the enemy level from exp
    pub fn calculate_enemy_level(&self, exp: f64) -> f64 {
        match &self.enemy_data.meta.levelup_exp {
            LevelupExpType::Early => exp.cbrt().abs(),
            LevelupExpType::Normal => exp.cbrt().abs(),
            LevelupExpType::Late => exp.cbrt().abs(),
        }
    }

    /// Need to up the level exp
    pub fn calculate_need_level(&self, level: u32) -> u32 {
        let power_of_three = |level: u32| level * level * level;

        match &self.player_data.meta.levelup_exp {
            LevelupExpType::Early => {
                if level <= 35 {
                    power_of_three(level) - level * (level * 3)
                } else if level <= 70 {
                    power_of_three(level)
                } else {
                    power_of_three(level) + level * (level * 3)
                }
            }
            LevelupExpType::Normal => power_of_three(level),
            LevelupExpType::Late => {
                if level <= 30 {
                    power_of_three(level) + level * (level * 3)
                } else if level <= 65 {
                    power_of_three(level)
                } else {
                    power_of_three(level) - level * (level * 3)
                }
            }
        }
    }

    pub fn status_up(&mut self, charatype: StatusCharaType) -> &mut Self {
        match charatype {
            StatusCharaType::Enemy => {
                self.enemy_data.charabase.hp += 2;
                self.enemy_data.charabase.power += 2;
                self.enemy_data.charabase.guard += 2;
                self.enemy_data.charabase.speed += 2;
                self.enemy_data.charabase.mp += 2;
                self
            }
            StatusCharaType::Player => {
                self.player_data.charabase.hp += 2;
                self.player_data.charabase.power += 2;
                self.player_data.charabase.guard += 2;
                self.player_data.charabase.speed += 2;
                self.player_data.charabase.mp += 2;
                self
            }
        }
    }
}

pub enum StatusCharaType {
    Enemy,
    Player,
}
