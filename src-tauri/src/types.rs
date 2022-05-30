use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeStatsRequest {
    pub path: String,
    pub exclude_dirs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeStatsResponse {
    pub request: CodeStatsRequest,
}
