use std::path::Path;

use crate::setting::CharaData;
use tokio::fs;

pub(crate) async fn new_enemy<T: AsRef<Path>>(toml_path: T) -> Option<CharaData> {
    let chara_data = {
        let file_content = fs::read_to_string(toml_path).await.unwrap();

        toml::from_str(file_content.as_str())
    }
    .unwrap();

    Some(chara_data)
}
