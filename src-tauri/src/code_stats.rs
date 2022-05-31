use super::types::{CodeStatsRequest, CodeStatsResponse, DirStats, FileStats};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

impl FileStats {
    pub fn collect(
        path: &Path,
        _relative_path: &Path,
        depth: usize,
        included: &HashSet<PathBuf>,
    ) -> io::Result<Self> {
        let mut stats = Self {
            path: path.to_path_buf(),
            depth,
            language: "".to_owned(),
            line_count: 0,
            char_count: 0,
        };

        if included.contains(&path.to_path_buf()) {
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

impl DirStats {
    fn find_gitignores(path: &Path) -> io::Result<Vec<PathBuf>> {
        let mut gitignore_files = vec![];

        let entries = fs::read_dir(path.clone())?;

        for entry in entries.into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().unwrap().is_dir() {
                gitignore_files.extend(Self::find_gitignores(entry.path().as_path())?);
            } else if entry.file_type().unwrap().is_file() && entry.file_name() == ".gitignore" {
                gitignore_files.push(entry.path());
            }
        }

        Ok(gitignore_files)
    }

    fn gitignore_applies(path: &Path, gitignore_path: &Path) -> bool {
        path != gitignore_path.parent().unwrap()
            && path.starts_with(gitignore_path.parent().unwrap())
    }

    fn file_included(path: &Path, gitignores: &Vec<(PathBuf, HashSet<PathBuf>)>) -> bool {
        for (gitignore_path, included) in gitignores.iter() {
            if Self::gitignore_applies(path, gitignore_path.as_path()) && !included.contains(path) {
                return false;
            }
        }

        true
    }

    fn find_included_files(
        path: &Path,
        gitignores: &Vec<(PathBuf, HashSet<PathBuf>)>,
    ) -> io::Result<HashSet<PathBuf>> {
        let mut included = HashSet::new();

        let entries = fs::read_dir(path.clone()).unwrap();

        for entry in entries.into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().unwrap().is_dir() {
                included.extend(Self::find_included_files(
                    entry.path().as_path(),
                    gitignores,
                )?);
            } else if entry.file_type().unwrap().is_file() {
                if Self::file_included(entry.path().as_path(), &gitignores) {
                    included.insert(entry.path());
                }
            }
        }

        Ok(included)
    }

    fn included_files(path: &Path) -> io::Result<HashSet<PathBuf>> {
        let gitignore_files = Self::find_gitignores(path)?;
        let gitignores: Vec<(PathBuf, HashSet<PathBuf>)> = gitignore_files
            .iter()
            .map(|g| {
                (
                    g.to_owned(),
                    HashSet::from_iter(
                        gitignore::File::new(g)
                            .unwrap()
                            .included_files()
                            .unwrap()
                            .iter()
                            .cloned(),
                    ),
                )
            })
            .collect();

        Self::find_included_files(path, &gitignores)
    }

    fn collect_stats(
        path: &Path,
        relative_path: &Path,
        depth: usize,
        included: &HashSet<PathBuf>,
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

        if path.file_name().unwrap().to_str().unwrap() != ".git" {
            let entries = fs::read_dir(path.clone())?;

            for entry in entries.into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().unwrap().is_dir() {
                    let new_path_buf = path.join(Path::new(entry.file_name().to_str().unwrap()));
                    let new_path = new_path_buf.as_path();
                    let new_relative_path_buf =
                        relative_path.join(Path::new(entry.file_name().to_str().unwrap()));
                    let new_relative_path = new_relative_path_buf.as_path();

                    let dir =
                        DirStats::collect_stats(new_path, new_relative_path, depth + 1, included)?;

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

                    let file =
                        FileStats::collect(new_path, new_relative_path, depth + 1, included)?;

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

    pub fn collect(path: &Path) -> io::Result<Self> {
        let included = Self::included_files(path)?;
        Self::collect_stats(path, Path::new("/"), 0, &included)
    }
}

#[tauri::command(async)]
pub fn get_code_stats(request: CodeStatsRequest) -> CodeStatsResponse {
    let stats = DirStats::collect(Path::new(&request.path));

    match stats {
        Ok(value) => CodeStatsResponse::Ok(value),
        Err(_err) => CodeStatsResponse::Error,
    }
}
