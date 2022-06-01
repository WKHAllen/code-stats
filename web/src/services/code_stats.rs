use super::super::types::{CodeStatsRequest, CodeStatsResponse};
use super::command::tauri_command;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct CommandArgs {
    pub request: CodeStatsRequest,
}

pub async fn get_code_stats(path: &Path) -> CodeStatsResponse {
    let request = CommandArgs {
        request: CodeStatsRequest {
            path: path.to_str().unwrap().to_owned(),
        },
    };

    let response = tauri_command("get_code_stats", &request).await.unwrap();

    response.into_serde().unwrap()
}
