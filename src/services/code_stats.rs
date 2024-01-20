//! Code statistics services.

use super::TaskPool;
use ignore::Walk;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Code statistics for a single file.
#[derive(Debug, Clone, PartialEq)]
pub struct FileStats {
    /// The language of the file.
    pub language: String,
    /// The number of lines in the file.
    pub line_count: usize,
    /// The number of bytes in the file.
    pub byte_count: usize,
}

/// Code statistics for a directory.
#[derive(Debug, Clone, PartialEq)]
pub struct DirStats {
    /// A mapping of subdirectory names to their code statistics.
    pub dirs: HashMap<String, DirStats>,
    /// A mapping of file names to their code statistics.
    pub files: HashMap<String, FileStats>,
    /// A mapping of languages to the number of files written in them.
    pub file_counts: HashMap<String, usize>,
    /// A mapping of languages to the number of lines written in them.
    pub line_counts: HashMap<String, usize>,
    /// A mapping of languages to the number of bytes written in them.
    pub byte_counts: HashMap<String, usize>,
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
    let line_count = data.iter().fold(0, |total, this_char| {
        if *this_char == b'\n' {
            total + 1
        } else {
            total
        }
    }) + 1;
    let byte_count = data.len();

    Ok(FileStats {
        language,
        line_count,
        byte_count,
    })
}

/// Collects code statistics for a given directory.
async fn directory_stats<P>(path: P) -> io::Result<DirStats>
where
    P: AsRef<Path>,
{
    todo!()
}

/// Statistics on a codebase.
pub struct CodeStats {
    /// The path to the codebase.
    path: PathBuf,
    /// The statistics.
    stats: DirStats,
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
                dirs: HashMap::new(),
                files: HashMap::new(),
                file_counts: HashMap::new(),
                line_counts: HashMap::new(),
                byte_counts: HashMap::new(),
            },
        }
    }
}

/// Collects code statistics for the given directory.
pub async fn collect_stats<P>(path: P) -> io::Result<CodeStats>
where
    P: AsRef<Path>,
{
    todo!()
}
