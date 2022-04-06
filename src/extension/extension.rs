use super::config::ExtensionConfig;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// extensions info
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ExtensionStore {
    // Directory to load extensions
    extension_store_dir_path: PathBuf,
    // An array of extensions in the directory
    extensions: Vec<PathBuf>,
}

impl ExtensionStore {
    /// Get extensions from the extensions directory
    pub async fn extension_files() -> anyhow::Result<Self> {
        let mut dir_stream = tokio::fs::read_dir("extensions")
            .await
            .context("Not found extensions directory")?;
        let mut entries: Vec<PathBuf> = Vec::new();

        while let Some(entry) = dir_stream.next_entry().await? {
            let dir_entrey = entry.path();

            entries.push(dir_entrey);
        }

        Ok(Self {
            extension_store_dir_path: PathBuf::from("extensions"),
            extensions: entries,
        })
    }

    /// Get extension store dir path
    /// Often `extensions`
    pub fn extension_store_dir_path(&self) -> &Path {
        &self.extension_store_dir_path
    }

    /// Get extensions
    pub fn extensions(&self) -> &Vec<PathBuf> {
        &self.extensions
    }

    /// import extensions
    pub fn import(&self) -> Vec<ExtensionConfig> {
        let result: Vec<ExtensionConfig> = self
            .extensions
            .iter()
            .map(|p| p.join("manifest.toml"))
            .filter(|p| p.as_path().is_file())
            .map(|p| ExtensionConfig::parse(p).unwrap())
            .collect();
        result
    }
}

/// ExtensionInfo
#[derive(Debug, Clone)]
pub struct ExtensionClientSetting {
    config: ExtensionConfig,
    authority: AuthorityTypes,
}

impl ExtensionClientSetting {
    pub fn config(&self) -> &ExtensionConfig {
        &self.config
    }

    pub fn authority(&self) -> &AuthorityTypes {
        &self.authority
    }
}

/// Privileges granted to extensions
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AuthorityTypes {
    /// Standard Settings
    Standard,
    /// Strict Settings
    Strict,
    /// Tolerant Settings
    Tolerant,
}

impl Default for AuthorityTypes {
    fn default() -> Self {
        Self::Standard
    }
}
