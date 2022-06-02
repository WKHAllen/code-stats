use super::super::types::{CodeStatsRequest, CodeStatsResponse, CommandArgs};
use super::command::tauri_command;
use std::path::Path;

pub async fn get_code_stats(path: &Path) -> CodeStatsResponse {
    let request = CommandArgs {
        request: CodeStatsRequest {
            path: path.to_path_buf(),
        },
    };

    let response = tauri_command("get_code_stats", &request).await.unwrap();

    response.into_serde().unwrap()
}
