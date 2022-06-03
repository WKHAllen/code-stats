use super::super::types::{CommandArgs, DirectoryInfoRequest, DirectoryInfoResponse};
use super::command::tauri_command;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
struct EmptyArgs {}

pub async fn get_directory_info(path: &Path) -> DirectoryInfoResponse {
    let request = CommandArgs {
        request: DirectoryInfoRequest {
            path: path.to_path_buf(),
        },
    };

    let response = tauri_command("get_directory_info", &request).await.unwrap();

    response.into_serde().unwrap()
}

pub async fn get_home_directory() -> Option<PathBuf> {
    let response = tauri_command("get_home_directory", &EmptyArgs {})
        .await
        .unwrap();

    response.into_serde().unwrap()
}
