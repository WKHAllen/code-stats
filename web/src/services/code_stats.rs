use super::super::types::{CodeStatsRequest, CodeStatsResponse};
use super::command::tauri_command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CommandArgs {
    pub request: CodeStatsRequest,
}

pub async fn get_code_stats(path: &str) -> CodeStatsResponse {
    let request = CommandArgs {
        request: CodeStatsRequest {
            path: path.to_owned(),
        },
    };

    let response = tauri_command("get_code_stats", &request).await.unwrap();
    web_sys::console::log_1(&response);

    response.into_serde().unwrap()
}
