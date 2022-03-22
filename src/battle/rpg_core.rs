use crate::battle::model::{CharaConfig, LuckyLevel};
use crate::battle::utils::dir_files;
use anyhow::Context;
use chrono::prelude::NaiveDateTime;
use once_cell::sync::Lazy;
use rand::prelude::IteratorRandom;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize)]
pub enum PlayMode {
    Simple,
    Raid,
    Story { id: String },
}

#[derive(Debug, Clone, Deserialize, PartialEq, PartialOrd)]
pub struct BattleData {
    player_data: CharaConfig,
    enemy_data: CharaConfig,
    play_mode: PlayMode,
    elapesd_turns: u32,
    start_time: NaiveDateTime,
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
        } else {
            Err(anyhow::anyhow!("No match regex {:?}", &name_arg))
        }
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
        base_exp *= l.lucky_number()
    }

    base_exp as f32
}

impl PlayMode {
    /// get story id
    pub fn story_id(&self) -> Option<&str> {
        match self {
            Self::Simple => None,
            Self::Raid => None,
            Self::Story { id: a } => Some(a),
        }
    }
}

impl BattleData {
    pub fn new(
        player_data: CharaConfig,
        enemy_data: CharaConfig,
        play_mode: PlayMode,
        start_time: NaiveDateTime,
        elapesd_turns: u32,
    ) -> Self {
        Self {
            player_data,
            enemy_data,
            play_mode,
            start_time,
            elapesd_turns,
        }
    }

    /// Advance the elapsed turn
    pub fn add_turn(&mut self) -> &mut Self {
        self.elapesd_turns += 1;
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
    /// When this function is called, the turn advances by 1.
    /// ([add_turn](add_turn))
    pub fn turn(&mut self) -> &CharaConfig {
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

    /// player -> enemy
    fn calculate_player_damage<'a>(
        enemy: &'a mut CharaConfig,
        player: &CharaConfig,
    ) -> &'a CharaConfig {
        let mut rng = rand::thread_rng();
        let player_attack = player.attack.iter().choose(&mut rng).unwrap();
        let to_enemy_damage = enemy.charabase.hp - player_attack.damage as u8;
        enemy.charabase.hp = to_enemy_damage;
        enemy
    }

    /// enemy -> player
    fn calculate_enemy_damage<'a>(
        enemy: &CharaConfig,
        player: &'a mut CharaConfig,
    ) -> &'a CharaConfig {
        let mut rng = rand::thread_rng();
        let enemy_attack = enemy.attack.iter().choose(&mut rng).unwrap();
        let to_player_damage = *&mut player.charabase.hp - enemy_attack.damage as u8;
        player.charabase.hp = to_player_damage;
        player
    }
    /// Battle
    pub fn battle(mut self) -> Self {
        todo!()
    }
}
