use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct CharaConfig {
    pub charabase: CharaBase,
    pub attack: Vec<CharaAttack>,
    pub meta: CharaMeta,
    pub(crate) inside_info: InsideInfo
}

#[derive(Copy, Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharaBase {
    pub power: i16,
    pub guard: i16,
    pub speed: i16,
    pub hp: i16,
    pub mp: i16,
}

#[derive(Deserialize, Serialize, PartialEq, PartialOrd, Debug, Clone)]
pub struct CharaAttack {
    pub name: String,
    pub damage: u32,
    pub hit_rate: f32,
    pub abnormal_state: Option<AbnormalState>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharaMeta {
    pub name: String,
    pub levelup_exp: LevelupExpType,
    pub species_type: SpeciesType,
    pub get_exp: u32,
    pub skill_type: SkillType,
}

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Serialize)]
pub struct InsideInfo {
    regex: String,
    alias: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum LevelupExpType {
    Early,
    Normal,
    Late,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum SpeciesType {
    /// Try to use as detailed an element as possible
    /// It is preferable to use it when you add a character in the extension
    Human {
        description: String,
    },
    /// Try to use as detailed an element as possible
    /// It is preferable to use it when you add a character in the extension
    Yokai {
        description: String,
    },
    ShrineMaiden,
    YokaiWhoManipulatesDarkness,
    YokaiWhoUsageOfQi,
    Fairy,
    Magician,
    Witch,
    Maid,
    Vampire,
    Yukionna,
    Shikigami,
    Poltergeists,
    /// Half-human half-phantom
    HanzinHanrei,
    Ghost,
    Oni,
    NightSparrow,
    WereHakutaku,
    Rabbit,
    Tengu,
    Doll,
    Shinigami,
    Yamaxanadu,
    Kami,
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

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum SkillType {
    Lucky { level: LuckyLevel },
    Effort,
}

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Serialize)]
pub enum LuckyLevel {
    LuckyOne,
    LuckyTwo,
    LuckyThree,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum AbnormalState {
    Slowed,
    Poisoned,
    Unlucky,
}

impl CharaConfig {
    pub async fn charas_new<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Self>> {
        let files = thrpg_utils::dir_files(path.as_ref()).await?;
        let mut vec = Vec::new();
        for file_path in files {
            vec.push(thrpg_utils::read_to_toml(file_path).await?);
        }
        Ok(vec)
    }


    pub fn charas_new_noasync<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Self>> {
    let files = thrpg_utils::dir_files_noasync(path.as_ref())?;
        let mut vec = Vec::new();
        for file_path in files {
            vec.push(thrpg_utils::read_to_toml_noasync(file_path)?);
        }
        Ok(vec)
    }

    /// Create a structure from file names
    /// `chara/{file names}.toml`
    pub async fn from_file_name<T: ToString>(name: T) -> anyhow::Result<Self> {
        let content = thrpg_utils::read_to_toml(format!("chara/{}.toml", name.to_string())).await?;
        Ok(content)
    }

    /// Create a structure from file names
    /// `chara/{file names}.toml`
    pub fn from_file_name_noasync<T: ToString>(name: T) -> anyhow::Result<Self> {
        let content = thrpg_utils::read_to_toml_noasync(format!("chara/{}.toml", name.to_string()))?;
        Ok(content)
    }
}

