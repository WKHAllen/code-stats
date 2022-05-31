use super::types::{CodeStatsRequest, CodeStatsResponse, DirStats, FileStats};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn excluded(path: &Path, gitignores: &Vec<PathBuf>) -> bool {
    path.file_name().unwrap().to_str().unwrap().starts_with(".")
        || gitignores
            .iter()
            .filter(|g| gitignore::File::new(g).unwrap().is_excluded(path).unwrap())
            .collect::<Vec<_>>()
            .len()
            > 0
}

impl DirStats {
    pub fn collect(
        path: &Path,
        relative_path: &Path,
        depth: usize,
        request: &CodeStatsRequest,
        mut gitignores: Vec<PathBuf>,
    ) -> io::Result<Self> {
        let mut stats = Self {
            path: path.to_path_buf(),
            dirs: vec![],
            files: vec![],
            depth,
            file_counts: HashMap::new(),
            line_counts: HashMap::new(),
            char_counts: HashMap::new(),
        };

        if !excluded(path, &gitignores) {
            let entries = fs::read_dir(path.clone())?;

            let local_gitignore = path.join(".gitignore");

            if local_gitignore.exists() {
                gitignores.push(local_gitignore);
            }

            for entry in entries.into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().unwrap().is_dir() {
                    let new_path_buf = path.join(Path::new(entry.file_name().to_str().unwrap()));
                    let new_path = new_path_buf.as_path();
                    let new_relative_path_buf =
                        relative_path.join(Path::new(entry.file_name().to_str().unwrap()));
                    let new_relative_path = new_relative_path_buf.as_path();

                    let dir = DirStats::collect(
                        new_path,
                        new_relative_path,
                        depth + 1,
                        &request,
                        gitignores.clone(),
                    )?;

                    for (language, count) in dir.file_counts.iter() {
                        *stats.file_counts.entry(language.to_string()).or_insert(0) += count;
                    }

                    for (language, count) in dir.line_counts.iter() {
                        *stats.line_counts.entry(language.to_string()).or_insert(0) += count;
                    }

                    for (language, count) in dir.char_counts.iter() {
                        *stats.char_counts.entry(language.to_string()).or_insert(0) += count;
                    }

                    stats.dirs.push(dir);
                } else if entry.file_type().unwrap().is_file() {
                    let new_path_buf = path.join(Path::new(entry.file_name().to_str().unwrap()));
                    let new_path = new_path_buf.as_path();
                    let new_relative_path_buf =
                        relative_path.join(Path::new(entry.file_name().to_str().unwrap()));
                    let new_relative_path = new_relative_path_buf.as_path();

                    let file = FileStats::collect(
                        new_path,
                        new_relative_path,
                        depth + 1,
                        &request,
                        gitignores.clone(),
                    )?;

                    *stats
                        .file_counts
                        .entry(file.language.to_string())
                        .or_insert(0) += 1;
                    *stats
                        .line_counts
                        .entry(file.language.to_string())
                        .or_insert(0) += file.line_count;
                    *stats
                        .char_counts
                        .entry(file.language.to_string())
                        .or_insert(0) += file.char_count;

                    stats.files.push(file);
                }
            }
        }

        Ok(stats)
    }
}

impl FileStats {
    pub fn collect(
        path: &Path,
        relative_path: &Path,
        depth: usize,
        request: &CodeStatsRequest,
        gitignores: Vec<PathBuf>,
    ) -> io::Result<Self> {
        println!("Reading {}", path.display());

        let mut stats = Self {
            path: path.to_path_buf(),
            depth,
            language: "".to_owned(),
            line_count: 0,
            char_count: 0,
        };

        if !excluded(path, &gitignores) {
            let file_contents = fs::read_to_string(path);

            if let Ok(contents) = file_contents {
                stats.language = path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap()
                    .to_owned();
                stats.line_count = contents.matches("\n").count();
                stats.char_count = contents.chars().count();
            }
        }

        Ok(stats)
    }
}

#[tauri::command]
pub fn get_code_stats(request: CodeStatsRequest) -> CodeStatsResponse {
    let stats = DirStats::collect(
        Path::new(&request.path),
        Path::new("/"),
        0,
        &request,
        vec![],
    );

    CodeStatsResponse::Ok(stats.unwrap())
    // match stats {
    //     Ok(value) => CodeStatsResponse::Ok(value),
    //     Err(_err) => CodeStatsResponse::Error,
    // }
}
