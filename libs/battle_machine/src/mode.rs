use serde::{Deserialize, Serialize};

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
