use super::types::{DirectoryInfo, DirectoryInfoRequest, DirectoryInfoResponse};
use std::fs;
use std::io;
use std::path::Path;

fn collect_dir_info(path: &Path) -> io::Result<DirectoryInfo> {
    let mut dirs = vec![];
    let mut files = vec![];

    let entries = fs::read_dir(path)?;

    for entry in entries.into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().unwrap().is_dir() {
            dirs.push(entry.file_name().to_str().unwrap().to_owned());
        } else if entry.file_type().unwrap().is_file() {
            files.push(entry.file_name().to_str().unwrap().to_owned());
        }
    }

    Ok(DirectoryInfo {
        path: path.to_path_buf(),
        dirs,
        files,
    })
}

#[tauri::command(async)]
pub fn get_directory_info(request: DirectoryInfoRequest) -> DirectoryInfoResponse {
    let dir_info = collect_dir_info(&request.path);

    match dir_info {
        Ok(value) => DirectoryInfoResponse::Ok(value),
        Err(err) => DirectoryInfoResponse::Error(err.to_string()),
    }
}
