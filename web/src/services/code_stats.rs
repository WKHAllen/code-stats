use super::super::types::{CodeStatsRequest, CodeStatsResponse, CommandArgs, DirStats};
use super::command::tauri_command;
use super::lang;
use std::io;
use std::path::Path;

pub async fn get_code_stats(path: &Path) -> CodeStatsResponse {
    let request = CommandArgs {
        request: CodeStatsRequest {
            path: path.to_path_buf(),
        },
    };

    let response = tauri_command("get_code_stats", &request).await.unwrap();

    response.into_serde().unwrap()
}

pub fn get_stats_subpath<'a>(stats: &'a DirStats, subpath: &Path) -> io::Result<&'a DirStats> {
    match subpath.components().next() {
        Some(next_component) => {
            let next_stats = match stats
                .dirs
                .get(&next_component.as_os_str().to_str().unwrap().to_owned())
            {
                Some(value) => Ok(value),
                None => Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "invalid directory subpath",
                )),
            }?;
            let next_subpath = subpath.strip_prefix(next_component).unwrap();
            get_stats_subpath(next_stats, next_subpath)
        }
        None => Ok(stats),
    }
}

pub fn get_dir_primary_lang(stats: &DirStats) -> Option<String> {
    let mut stats_vec = stats.char_counts.iter().map(|x| x).collect::<Vec<_>>();
    stats_vec.sort_by(|(_ext1, count1), (_ext2, count2)| count2.cmp(count1));

    if stats_vec.len() > 0 {
        Some(
            stats_vec
                .iter()
                .find(|(ext, _count)| lang::known_language(ext))
                .unwrap_or(&(&"Other".to_owned(), &0))
                .0
                .clone(),
        )
    } else {
        None
    }
}
