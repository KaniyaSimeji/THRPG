pub mod extension;

use serde::{Deserialize, Serialize};
use std::path::Path;
use url::Url;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtensionConfig {
    meta: ExtensionMeta,
    dependencies: Option<Vec<ExtensionDependency>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtensionMeta {
    #[serde(alias = "type")]
    extension_type: Extensiontype,
    #[serde(alias = "name")]
    extension_name: String,
    #[serde(alias = "author")]
    extension_author: Vec<String>,
    #[serde(alias = "version")]
    extension_version: String,
    #[serde(alias = "license")]
    extension_license: Option<String>,
    #[serde(alias = "docs")]
    extension_docs_url: Option<String>,
    #[serde(alias = "website")]
    extension_website: Option<String>,
    #[serde(alias = "repository")]
    extension_repository: Option<String>,
    #[serde(alias = "story_original")]
    extension_story_origin: Option<String>,
    #[serde(alias = "description")]
    extension_description: String,
    #[serde(alias = "readme")]
    extension_readme: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Extensiontype {
    Story,
    Contents,
    Features,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtensionDependency {
    #[serde(alias = "name")]
    extension_name: String,
    require_version: Option<String>,
    mandatory: bool,
}

impl Extensiontype {
    pub fn try_from_types(value: &str) -> anyhow::Result<Extensiontype> {
        match value {
            "Story" | "story" => Ok(Self::Story),
            "Contents" | "contents" => Ok(Self::Contents),
            "Features" | "features" => Ok(Self::Features),
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
        let parse_result = semver::Version::parse(&self.meta.extension_version)?;
        Ok(parse_result)
    }

    pub fn init(&self) {
        match &self.meta.extension_type {
            Extensiontype::Story => todo!(),
            Extensiontype::Contents => todo!(),
            Extensiontype::Features => todo!(),
        }
    }

    /// get extension author
    pub fn author(&self) -> &Vec<String> {
        &self.meta.extension_author
    }

    /// get extension name
    pub fn name(&self) -> &str {
        &self.meta.extension_name
    }

    /// get extension type
    /// extension type: [Extensiontype](Extensiontype)
    pub fn extension_type(&self) -> &Extensiontype {
        &self.meta.extension_type
    }

    /// get extension version
    pub fn version(&self) -> &str {
        &self.meta.extension_version
    }

    pub fn try_from_url(&self) -> Option<Url> {
        if let Some(url_str) = &self.meta.extension_docs_url {
            Some(Url::parse(url_str).unwrap())
        } else {
            None
        }
    }
}
