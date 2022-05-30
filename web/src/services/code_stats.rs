use super::super::types::{CodeStatsRequest, CodeStatsResponse};
use super::command::tauri_command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    pub request: CodeStatsRequest,
}

pub async fn get_code_stats(path: &str, exclude_dirs: Vec<String>) -> CodeStatsResponse {
    let request = Request {
        request: CodeStatsRequest {
            path: path.to_owned(),
            exclude_dirs,
        },
    };

    let response = tauri_command("get_code_stats", &request).await.unwrap();

    CodeStatsResponse {
        request: response.into_serde::<Request>().unwrap().request,
    }
}
