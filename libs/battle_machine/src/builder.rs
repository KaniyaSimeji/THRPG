use std::path::{PathBuf, Path};

use crate::{
    chara::CharaConfig,
    rpg_core::BattleData,
    mode::PlayMode
};
use chrono::prelude::{Local, NaiveDateTime};
use rand::prelude::IteratorRandom;
use thrpg_database::playdata::Model;
use uuid::Uuid;

/// Structure for making battles from fragmentary information
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BattleBuilder {
    uuid: Uuid,
    datatime: NaiveDateTime,
    mode: PlayMode,
    player: Option<CharaConfig>,
    enemy: Option<CharaConfig>,
    elapsed_turns: u32,
}

#[derive(Debug)]
/// Settings for randomly selecting a character
pub struct RandomOption {
    toml_dir_path: PathBuf,
    exclude_charas: Option<Vec<String>>,
}

impl Default for RandomOption {
    fn default() -> Self {
        Self {
            toml_dir_path: PathBuf::new().join("chara/"),
            exclude_charas: None,
        }
    }
}

impl Default for BattleBuilder {
    fn default() -> Self {
        Self {
            datatime: Local::now().naive_local(),
            mode: PlayMode::Simple,
            elapsed_turns: 0,
            enemy: None,
            player: None,
            uuid: Uuid::new_v4(),
        }
    }
}

impl BattleBuilder {
    pub fn new(
        mode: PlayMode,
        player: Option<CharaConfig>,
        enemy: Option<CharaConfig>,
        elapsed_turns: Option<u32>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            datatime: Local::now().naive_local(),
            mode,
            player,
            enemy,
            elapsed_turns: elapsed_turns.unwrap_or_default(),
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
    pub async fn enemy_random(&mut self, random_options: RandomOption, charas: Vec<CharaConfig>) -> &mut Self {
        let chara = random_options.chara_random(charas).ok();
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
            self.elapsed_turns,
        )
    }
}

impl RandomOption {
    pub fn new() -> Self {
        Self::default()
    }

    /// Default value :`chara/`
    pub fn path<P:AsRef<Path>>(&mut self, toml_dit_path: P) -> &mut Self {
        self.toml_dir_path = toml_dit_path.as_ref().to_path_buf();
        self
    }

    pub fn exclude_charas<T>(&mut self, charas: T) -> &mut Self
    where T: FnOnce(&mut Vec<String>) -> &mut Vec<String>
    {
        let mut vec = Vec::new();
        charas(&mut vec);
        self.exclude_charas = Some(vec);
        self
    }

    pub fn chara_random(self, charas: Vec<CharaConfig>) -> anyhow::Result<CharaConfig> {
        let mut rng = rand::thread_rng();
        let mut chara: CharaConfig;
        if let Some(f) = self.exclude_charas {
                loop {
                    chara = charas.iter().choose(&mut rng).unwrap().clone();
                    if f.iter()
                        .any(|f| f == &chara.meta.name)
                        != true
                    {
                        break;
                    }
            }
        } else {
            chara = charas.iter().choose(&mut rng).unwrap().clone();
        }

        Ok(chara)

    }
}

impl TryFrom<Model> for BattleBuilder {
    type Error = anyhow::Error;
    fn try_from(model: Model) -> Result<Self, Self::Error> {
        let builder = BattleBuilder::new(
            PlayMode::try_from_value(&model.play_mode)?,
            // base type is CharaConfig
            Some(
                CharaConfig::from_file_name_noasync(
                    &model.player["charabase"]["name"].as_str().unwrap(),
                )?,
            ),
            // base type is CharaConfig
            Some(
                CharaConfig::from_file_name_noasync(&model.enemy["charabase"]["name"].as_str().unwrap())?,
            ),
            Some(model.elapesd_turns),
        );
        Ok(builder)
    }
}
