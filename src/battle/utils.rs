use crate::battle::model::CharaConfig;
use anyhow::Context;
use std::path::{Path, PathBuf};

/// read file to toml
fn read_to_toml<T: AsRef<Path>>(toml_path: T) -> anyhow::Result<CharaConfig> {
    let chara_data: CharaConfig = {
        let file_content =
            std::fs::read_to_string(toml_path).context("ファイルを読み込めませんでした")?;

        toml::from_str(file_content.as_str())
    }
    .context("不正なTOMLです")?;

    Ok(chara_data)
}

/// directory files push vec
pub(crate) async fn dir_files<T: AsRef<Path>>(
    toml_dir_path: T,
) -> Result<Vec<CharaConfig>, anyhow::Error> {
    if toml_dir_path.as_ref().is_dir() {
        let mut vec: Vec<PathBuf> = Vec::new();
        let mut entries = tokio::fs::read_dir(toml_dir_path)
            .await
            .context("ディレクトリが読み込めません")?;
        while let Some(entry) = entries.next_entry().await? {
            let dir_entrey = entry.path();

            vec.push(dir_entrey);
        }

        vec.iter().map(read_to_toml).collect()
    } else {
        Err(anyhow::anyhow!("not directory"))
    }
}
