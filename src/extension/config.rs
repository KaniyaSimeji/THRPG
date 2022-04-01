use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct ExtensionConfig {
    extension_type: Extensiontype,
    extension_name: String,
    extension_author: Vec<String>,
    extension_version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub enum Extensiontype {
    Story,
    Contents,
    NewFeatures,
}

impl Extensiontype {
    pub fn try_from_types(value: &str) -> anyhow::Result<Extensiontype> {
        match value {
            "Story" | "story" => Ok(Self::Story),
            "Contents" | "contents" => Ok(Self::Contents),
            "NewFeatures" | "newfeatures" => Ok(Self::NewFeatures),
            _ => Err(anyhow::anyhow!(format!("Not match value {}", value))),
        }
    }
}

impl ExtensionConfig {
    pub fn parse<T>(path: T) -> anyhow::Result<Self>
    where
        T: AsRef<Path>,
    {
        let file_content = std::fs::read_to_string(&path)?;
        toml::from_str(&file_content).map_err(|e| {
            anyhow::anyhow!(format!(
                "file error:{:?} in {:?}",
                path.as_ref(),
                e.line_col().unwrap_or((0, 0))
            ))
        })
    }

    pub fn extension_version_to_semver(&self) -> anyhow::Result<semver::Version> {
        let parse_result = semver::Version::parse(&self.extension_version)?;
        Ok(parse_result)
    }

    pub fn init(&self) {
        match &self.extension_type {
            Extensiontype::Story => todo!(),
            Extensiontype::Contents => todo!(),
            Extensiontype::NewFeatures => todo!(),
        }
    }
}
