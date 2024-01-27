//! Code statistics services.

use super::TaskPool;
use crate::services::*;
use ignore::Walk;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap};
use std::io;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::spawn;
use tokio::sync::mpsc::channel;

/// The default task pool size.
const TASK_POOL_SIZE: usize = 20;

/// A wrapper around a file or directory name. This is necessary so that file
/// and directory names will disregard case when ordering.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name(String);

impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Borrow<str> for Name {
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}

impl Borrow<String> for Name {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

impl From<String> for Name {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.to_lowercase().cmp(&other.0.to_lowercase()) {
            Ordering::Equal => self.0.cmp(other),
            other => other,
        }
    }
}

/// Tallied statistics for a single file.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FileCounts {
    /// The number of lines in the file.
    pub lines: usize,
    /// The number of bytes in the file.
    pub bytes: usize,
}

/// Code statistics for a single file.
#[derive(Debug, Clone, PartialEq)]
pub struct FileStats {
    /// The language of the file.
    pub language: String,
    /// The tallied statistics.
    pub counts: FileCounts,
}

/// Tallied statistics for a directory.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DirCounts {
    /// The number of files written in a given language.
    pub files: usize,
    /// The number of lines written in a given language.
    pub lines: usize,
    /// The number of bytes written in a given language.
    pub bytes: usize,
}

/// Code statistics for a directory.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DirStats {
    /// A mapping of subdirectory names to their code statistics.
    pub dirs: BTreeMap<Name, DirStats>,
    /// A mapping of file names to their code statistics.
    pub files: BTreeMap<Name, FileStats>,
    /// A mapping of languages to their tallied statistics.
    pub counts: HashMap<String, DirCounts>,
}

impl DirStats {
    /// Inserts a new directory into the data structure.
    fn insert_dir(&mut self, path: &Path) {
        if let Some(first) = path.iter().next().and_then(|s| s.to_str()) {
            let rest = path.strip_prefix(first).unwrap();

            self.dirs
                .entry(Name::from(first))
                .or_default()
                .insert_dir(rest);
        }
    }

    /// Inserts a new file with its statistics into the data structure.
    fn insert_file(&mut self, path: &Path, stats: FileStats) {
        let mut path_iter = path.iter();

        if let Some(first) = path_iter.next().unwrap().to_str() {
            match path_iter.next() {
                None => {
                    self.files.insert(Name::from(first), stats);
                }
                Some(_) => {
                    let rest = path.strip_prefix(first).unwrap();

                    self.dirs
                        .entry(Name::from(first))
                        .or_default()
                        .insert_file(rest, stats);
                }
            }
        }
    }

    /// Calculates stats for the directory and updates them in-place.
    fn tally_dir_stats(&mut self) {
        self.dirs.values_mut().for_each(|dir| dir.tally_dir_stats());

        self.files.values().for_each(|file| {
            let entry = self.counts.entry(file.language.clone()).or_default();
            entry.files += 1;
            entry.lines += file.counts.lines;
            entry.bytes += file.counts.bytes;
        });

        self.dirs.values().for_each(|dir| {
            dir.counts.iter().for_each(|(language, counts)| {
                let entry = self.counts.entry(language.clone()).or_default();
                entry.files += counts.files;
                entry.lines += counts.lines;
                entry.bytes += counts.bytes;
            })
        });
    }

    /// Gets a subsection of statistics given a path.
    pub fn stats_slice<P>(&self, subpath: P) -> io::Result<&DirStats>
    where
        P: AsRef<Path>,
    {
        let subpath = subpath.as_ref();

        match subpath.iter().next() {
            Some(next_component) => {
                let next_stats = match self.dirs.get(&Name::from(next_component.to_str().unwrap()))
                {
                    Some(value) => Ok(value),
                    None => Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "invalid directory subpath",
                    )),
                }?;
                let next_subpath = subpath.strip_prefix(next_component).unwrap();
                next_stats.stats_slice(next_subpath)
            }
            None => Ok(self),
        }
    }

    /// Gets the extension of the most prevalent language in the directory.
    pub fn primary_language(&self) -> Option<Language> {
        let mut stats_vec = self.counts.iter().collect::<Vec<_>>();
        stats_vec.sort_by(|(_, counts1), (_, counts2)| counts2.bytes.cmp(&counts1.bytes));

        if !stats_vec.is_empty() {
            Some(
                stats_vec
                    .iter()
                    .find_map(|(language, _)| {
                        let lang = Language::new(language);
                        lang.is_known().then_some(lang)
                    })
                    .unwrap_or(Language::Unknown),
            )
        } else {
            None
        }
    }
}

/// Collects code statistics for a given file.
async fn file_stats<P>(path: P) -> io::Result<FileStats>
where
    P: AsRef<Path>,
{
    let mut file = File::open(&path).await?;
    let size = file.metadata().await.map(|m| m.len() as usize).ok();
    let mut data = Vec::with_capacity(size.unwrap_or(0));
    file.read_to_end(&mut data).await?;

    let language = path
        .as_ref()
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap()
        .to_owned();
    let lines = data.iter().fold(0, |total, this_char| {
        if *this_char == b'\n' {
            total + 1
        } else {
            total
        }
    }) + 1;
    let bytes = data.len();

    Ok(FileStats {
        language,
        counts: FileCounts { lines, bytes },
    })
}

/// Statistics on a codebase.
#[derive(Debug, Clone, PartialEq)]
pub struct CodeStats {
    /// The path to the codebase.
    pub path: PathBuf,
    /// The statistics.
    pub stats: DirStats,
}

impl CodeStats {
    /// Creates a new empty code statistics instance.
    fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            path: path.as_ref().to_path_buf(),
            stats: DirStats {
                dirs: BTreeMap::new(),
                files: BTreeMap::new(),
                counts: HashMap::new(),
            },
        }
    }

    /// Inserts a new directory into the data structure.
    fn insert_dir(&mut self, path: &Path) {
        let relative_path = path.strip_prefix(&self.path).unwrap();
        self.stats.insert_dir(relative_path);
    }

    /// Inserts a new file with its statistics into the data structure.
    fn insert_file(&mut self, path: &Path, stats: FileStats) {
        let relative_path = path.strip_prefix(&self.path).unwrap();
        self.stats.insert_file(relative_path, stats);
    }

    /// Calculates stats for the directory and updates them in-place.
    fn tally_dir_stats(&mut self) {
        self.stats.tally_dir_stats();
    }

    /// Gets a subsection of statistics given a path.
    pub fn stats_slice<P>(&self, subpath: P) -> io::Result<&DirStats>
    where
        P: AsRef<Path>,
    {
        self.stats.stats_slice(subpath)
    }

    /// Gets the extension of the most prevalent language in the directory.
    #[allow(dead_code)]
    pub fn primary_language(&self) -> Option<Language> {
        self.stats.primary_language()
    }
}

/// A code statistics item.
#[derive(Debug, Clone, PartialEq)]
struct StatsItem {
    /// The full path to the item.
    path: PathBuf,
    /// The statistics. `None` indicates that the path is a directory.
    stats: Option<FileStats>,
}

/// Collects code statistics for the given directory.
pub async fn collect_stats<P>(path: P) -> io::Result<CodeStats>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let pool = TaskPool::new(TASK_POOL_SIZE);
    let (stats_sender, mut stats_receiver) = channel::<StatsItem>(TASK_POOL_SIZE);

    let stats_collection_task = spawn({
        let path = path.to_path_buf();
        async move {
            let mut stats = CodeStats::new(path);

            while let Some(stats_item) = stats_receiver.recv().await {
                match stats_item.stats {
                    Some(file_stats) => {
                        stats.insert_file(&stats_item.path, file_stats);
                    }
                    None => {
                        stats.insert_dir(&stats_item.path);
                    }
                }
            }

            stats.tally_dir_stats();
            stats
        }
    });

    for entry in Walk::new(path).flatten() {
        let entry_path = entry.into_path();

        if entry_path.is_file() {
            let stats_sender = stats_sender.clone();

            pool.queue(async move {
                if let Ok(stats) = file_stats(&entry_path).await {
                    stats_sender
                        .send(StatsItem {
                            path: entry_path,
                            stats: Some(stats),
                        })
                        .await
                        .unwrap();
                }
            })
            .await;
        } else if entry_path.is_dir() {
            stats_sender
                .send(StatsItem {
                    path: entry_path,
                    stats: None,
                })
                .await
                .unwrap();
        }
    }

    drop(stats_sender);
    pool.finish().await;

    let stats = stats_collection_task.await.unwrap();

    Ok(stats)
}
