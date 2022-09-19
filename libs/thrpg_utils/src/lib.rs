use std::path::{Path, PathBuf};

/// directory files push vec
pub async fn dir_files<T: AsRef<Path>>(toml_dir_path: T) -> anyhow::Result<Vec<PathBuf>> {
    if toml_dir_path.as_ref().is_dir() {
        let mut vec: Vec<PathBuf> = Vec::new();
        let mut entries = tokio::fs::read_dir(toml_dir_path)
            .await?;
        while let Some(entry) = entries.next_entry().await? {
            let dir_entry = entry.path();

            vec.push(dir_entry);
        }

        Ok(vec)
    } else {
        Err(anyhow::anyhow!("not directory"))
    }
}

/// directory files push vec no async
pub fn dir_files_noasync<T: AsRef<Path>>(toml_dir_path: T) -> anyhow::Result<Vec<PathBuf>> {
    if toml_dir_path.as_ref().is_dir() {
        let mut vec: Vec<PathBuf> = Vec::new();
        let entries = std::fs::read_dir(toml_dir_path)?;
        for entry in entries {
            let dir_entry = entry?.path();

            vec.push(dir_entry);
        }

        Ok(vec)
    } else {
        Err(anyhow::anyhow!("not directory"))
    }
}


pub async fn read_to_toml<T,P>(path: P) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
    P: AsRef<Path>
{
    let content = tokio::fs::read_to_string(path.as_ref()).await?;
    let toml = toml::from_str(&content)?;

    Ok(toml)
}

pub fn read_to_toml_noasync<T,P>(path: P) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
    P: AsRef<Path>
{
    let content = std::fs::read_to_string(path.as_ref())?;
    let toml = toml::from_str(&content)?;

    Ok(toml)
}
