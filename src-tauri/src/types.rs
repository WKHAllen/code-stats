use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DirStats {
    pub path: PathBuf,
    pub dirs: HashMap<String, DirStats>,
    pub files: HashMap<String, FileStats>,
    pub depth: usize,
    pub file_counts: HashMap<String, usize>,
    pub line_counts: HashMap<String, usize>,
    pub char_counts: HashMap<String, usize>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FileStats {
    pub path: PathBuf,
    pub depth: usize,
    pub language: String,
    pub line_count: usize,
    pub char_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeStatsRequest {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CodeStatsResponse {
    Ok(DirStats),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryInfo {
    pub path: PathBuf,
    pub dirs: Vec<String>,
    pub files: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryInfoRequest {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DirectoryInfoResponse {
    Ok(DirectoryInfo),
    Error(String),
}
