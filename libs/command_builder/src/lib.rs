pub mod commander;
pub mod action_manage;

pub mod standard_ui_components {
    use serenity::model::prelude::ReactionType;

    pub const YES: &str  = "👍";
    pub const NO: &str = "👎";
    pub const PLAY: &str = "🎮";
    pub const ITEM: &str = "🧰";
    pub const SAVE: &str  = "📝";
    pub const GUARD: &str = "🛡️";

    pub fn to_discord_emoji(base_emoji: &str) -> ReactionType {
        ReactionType::Unicode(base_emoji.to_string())
    }
}
