use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct CharaConfig {
    pub charabase: CharaBase,
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

#[derive(Deserialize, PartialEq, PartialOrd, Debug, Clone)]
pub struct CharaAttack {
    pub name: String,
    pub damage: u32,
    pub hit_rate: f32, // 0.08(8%) etc...
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
    pub species_description: Option<String>,
    pub get_exp: u32,
    pub skill_type: SkillType,
}

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum LevelupExpType {
    Early,
    Normal,
    Late,
}

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum SpeciesType {
    #[deprecated(note = "Minimal use as it is too abstracted!  \
        Please select a specific race")]
    Human,
    #[deprecated(note = "Minimal use as it is too abstracted!  \
        Please select a specific race")]
    Yokai,
    Maiden,
    YokaiWhoManipulatesDarkness,
    YokaiWhoUsageOfQi,
    Fairy,
    Magician,
    Witch,
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

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum SkillType {
    Lucky(LuckyLevel),
    Effort,
}

#[derive(Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum LuckyLevel {
    LuckyOne,
    LuckyTwo,
    LuckyThree,
}
