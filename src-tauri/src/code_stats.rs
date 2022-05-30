use super::types::{CodeStatsRequest, CodeStatsResponse};

#[tauri::command]
pub fn get_code_stats(request: CodeStatsRequest) -> CodeStatsResponse {
    CodeStatsResponse { request }
}
