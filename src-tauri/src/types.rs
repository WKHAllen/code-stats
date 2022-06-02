use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct DirStats {
    pub path: PathBuf,
    pub dirs: Vec<DirStats>,
    pub files: Vec<FileStats>,
    pub depth: usize,
    pub file_counts: HashMap<String, usize>,
    pub line_counts: HashMap<String, usize>,
    pub char_counts: HashMap<String, usize>,
}

#[derive(Serialize, Deserialize, Debug)]
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
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectoryInfo {
    pub path: PathBuf,
    pub dirs: Vec<String>,
    pub files: Vec<String>,
}
