use serde::Deserialize;

#[derive(Deserialize)]
pub struct CharaData {
    pub profile: Profile,
}

#[derive(Deserialize)]
pub struct Profile {
    pub name: String,
    pub power: u32,
    pub guard: u32,
    pub speed: u32,
    pub hp: u32,
    pub mp: u32,
    pub spellcard: String,
}
