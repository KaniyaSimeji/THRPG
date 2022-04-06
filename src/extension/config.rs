use serde::{Deserialize, Serialize};
use std::path::Path;
use url::Url;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtensionConfig {
    extension_type: Extensiontype,
    extension_name: String,
    extension_author: Vec<String>,
    extension_version: String,
    extension_license: Option<String>,
    extension_docs_url: Option<String>,
    extension_author_url: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

    /// get extension author
    pub fn author(&self) -> &Vec<String> {
        &self.extension_author
    }

    /// get extension name
    pub fn name(&self) -> &str {
        &self.extension_name
    }

    /// get extension type
    /// extension type: [Extensiontype](Extensiontype)
    pub fn extension_type(&self) -> &Extensiontype {
        &self.extension_type
    }

    /// get extension version
    pub fn version(&self) -> &str {
        &self.extension_version
    }

    pub fn try_from_url(&self) -> Option<Url> {
        if let Some(url_str) = &self.extension_docs_url {
            Some(Url::parse(url_str).unwrap())
        } else {
            None
        }
    }
}
