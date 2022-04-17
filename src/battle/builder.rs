use crate::battle::{
    model::CharaConfig,
    rpg_core::{BattleData, PlayMode},
    utils::dir_files,
};
use chrono::prelude::{Local, NaiveDateTime};
use rand::prelude::IteratorRandom;
use uuid::Uuid;

/// Structure for making battles from fragmentary information
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BattleBuilder {
    uuid: Uuid,
    datatime: NaiveDateTime,
    mode: PlayMode,
    player: Option<CharaConfig>,
    enemy: Option<CharaConfig>,
    elapesd_turns: u32,
}

/// Settings for randomly selecting a character
pub struct RandomOption {
    /// Whether the values of `player` and` enemy` may be the same
    pub allow_same_chara: bool,
    pub toml_dir_path: std::path::PathBuf,
    pub exclude_charas: Option<Vec<String>>,
}

impl Default for RandomOption {
    /// Default value
    /// `toml_dir_path`:`chara/`
    /// `exclude_charas`: `None`
    /// `allow_same_chara`: `true`
    fn default() -> Self {
        Self {
            toml_dir_path: std::path::Path::new("chara").to_path_buf(),
            exclude_charas: None,
            allow_same_chara: true,
        }
    }
}

impl Default for BattleBuilder {
    fn default() -> Self {
        Self {
            datatime: Local::now().naive_local(),
            mode: PlayMode::Simple,
            ..Default::default()
        }
    }
}

impl BattleBuilder {
    pub fn new(
        mode: PlayMode,
        player: Option<CharaConfig>,
        enemy: Option<CharaConfig>,
        elapesd_turns: Option<u32>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            datatime: Local::now().naive_local(),
            mode,
            player,
            enemy,
            elapesd_turns: elapesd_turns.unwrap_or_default(),
        }
    }

    /// get uuid
    pub const fn uuid(&self) -> Uuid {
        self.uuid
    }

    /// get datatime
    pub const fn datatime(&self) -> NaiveDateTime {
        self.datatime
    }

    /// get playmode
    pub const fn playmode(&self) -> &PlayMode {
        &self.mode
    }

    /// exist player
    pub const fn exist_player(&self) -> bool {
        match self.player {
            Some(_) => true,
            None => false,
        }
    }

    /// exist enemy
    pub fn exist_enemy(&self) -> bool {
        match self.enemy {
            Some(_) => true,
            None => false,
        }
    }

    /// make enemy
    pub fn enemy<T>(&mut self, new_enemy: T) -> &mut Self
    where
        T: Into<CharaConfig>,
    {
        self.enemy = Some(new_enemy.into());
        self
    }

    /// make player
    pub fn player<T>(&mut self, new_player: T) -> &mut Self
    where
        T: Into<CharaConfig>,
    {
        self.player = Some(new_player.into());
        self
    }

    /// Randomly choose the enemy
    pub async fn enemy_random(&mut self, random_options: RandomOption) -> &mut Self {
        let charas = dir_files(random_options.toml_dir_path).await.unwrap();
        let mut rng = rand::thread_rng();
        let mut chara = None;
        if random_options.allow_same_chara == false {
            while self.player != chara {
                chara = Some(charas.iter().choose(&mut rng).unwrap().clone());
            }
        } else if let Some(f) = random_options.exclude_charas {
            if f.iter()
                .any(|f| f == &chara.as_ref().unwrap().charabase.name)
                == true
            {
                loop {
                    chara = Some(charas.iter().choose(&mut rng).unwrap().clone());
                    if f.iter()
                        .any(|f| f == &chara.as_ref().unwrap().charabase.name)
                        != true
                    {
                        break;
                    }
                }
            }
        } else {
            chara = Some(charas.iter().choose(&mut rng).unwrap().clone());
        }
        self.enemy = chara;
        self
    }

    pub fn player_status_setting(&mut self, level: i16) -> &mut Self {
        match self.player.clone() {
            Some(mut p) => {
                p.charabase.power += 2 * level;
                p.charabase.guard += 2 * level;
                p.charabase.speed += 2 * level;
                p.charabase.mp += 2 * level;
                p.charabase.hp += 2 * level;
                self.player = Some(p);
                self
            }
            None => self,
        }
    }

    pub fn enemy_status_setting(&mut self, level: i16) -> &mut Self {
        match self.enemy.clone() {
            Some(mut p) => {
                p.charabase.power += 2 * level;
                p.charabase.guard += 2 * level;
                p.charabase.speed += 2 * level;
                p.charabase.mp += 2 * level;
                p.charabase.hp += 2 * level;
                self.player = Some(p);
                self
            }
            None => self,
        }
    }

    /// build BattleData
    pub fn build(self) -> BattleData {
        BattleData::new(
            self.uuid,
            self.player.unwrap(),
            self.enemy.unwrap(),
            self.mode,
            self.datatime,
            self.elapesd_turns,
        )
    }
}
