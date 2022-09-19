use crate::extension_config::ExtensionConfig;
use anyhow::Context;
use serde::Deserialize;
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
            let dir_entry = entry.path();

            entries.push(dir_entry);
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

    /// Get extensions path
    pub fn extensions_path(&self) -> &Vec<PathBuf> {
        &self.extensions
    }

    /// Get counted extensions
    pub fn count_extensions(&self) -> u32 {
        self.extensions.len() as u32
    }

    /// import extensions
    pub fn import(&self) -> Vec<ExtensionConfig> {
        let result: Vec<ExtensionConfig> = self
            .extensions
            .iter()
            .map(|p| p.join("manifest.toml"))
            .filter(|p| p.as_path().is_file())
            .filter_map(|p| ExtensionConfig::parse(p).ok())
            .collect();
        result
    }
}
