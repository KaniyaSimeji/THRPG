use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct CharaConfig {
    pub charabase: CharaBase,
    pub attack: Vec<CharaAttack>,
    pub meta: CharaMeta,
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CharaBase {
    pub name: String,
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
    pub hit_rate: f32, // 0.08(8%) etc...
    pub abnormal_state: Option<AbnormalState>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum AbnormalState {
    Slowed,
    Poisoned,
    Unlucky,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharaMeta {
    pub levelup_exp: LevelupExpType,
    pub species_type: SpeciesType,
    pub get_exp: u32,
    pub skill_type: SkillType,
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
