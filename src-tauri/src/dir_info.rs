use super::types::{DirectoryInfo, DirectoryInfoRequest, DirectoryInfoResponse};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use sysinfo::{DiskExt, System, SystemExt};

fn get_non_root_dir_info(path: &Path) -> io::Result<DirectoryInfo> {
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

fn get_root_dir_info() -> io::Result<DirectoryInfo> {
    match env::consts::OS {
        "windows" => {
            let mut sys = System::new();
            sys.refresh_disks_list();
            let disks = sys
                .disks()
                .into_iter()
                .map(|disk| {
                    disk.mount_point()
                        .join(Path::new("/"))
                        .to_str()
                        .unwrap()
                        .to_owned()
                })
                .collect();

            Ok(DirectoryInfo {
                path: PathBuf::from("/"),
                dirs: disks,
                files: vec![],
            })
        }
        _ => get_non_root_dir_info(Path::new("/")),
    }
}

fn collect_dir_info(path: &Path) -> io::Result<DirectoryInfo> {
    match path.to_str().unwrap() {
        "" | "/" => get_root_dir_info(),
        _ => get_non_root_dir_info(&path),
    }
}

#[tauri::command(async)]
pub fn get_directory_info(request: DirectoryInfoRequest) -> DirectoryInfoResponse {
    let path = if !request.path.ends_with("/") {
        let mut new_path = request.path.to_str().unwrap().to_owned();
        new_path.push_str("/");
        PathBuf::from(new_path)
    } else {
        request.path.clone()
    };

    let dir_info = collect_dir_info(&path);

    match dir_info {
        Ok(value) => DirectoryInfoResponse::Ok(value),
        Err(err) => DirectoryInfoResponse::Error(err.to_string()),
    }
}

#[tauri::command(async)]
pub fn get_home_directory() -> Option<PathBuf> {
    let home_dir = home::home_dir();

    match home_dir {
        Some(path) => Some(PathBuf::from(path.to_str().unwrap().replace("\\", "/"))),
        None => None,
    }
}
