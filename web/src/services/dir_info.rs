use super::super::types::{CommandArgs, DirectoryInfoRequest, DirectoryInfoResponse};
use super::command::tauri_command;
use std::path::Path;

pub async fn get_directory_info(path: &Path) -> DirectoryInfoResponse {
    let request = CommandArgs {
        request: DirectoryInfoRequest {
            path: path.to_path_buf(),
        },
    };

    let response = tauri_command("get_directory_info", &request).await.unwrap();

    response.into_serde().unwrap()
}
