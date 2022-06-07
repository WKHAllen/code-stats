use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

pub struct AppConfig {}

pub struct Color(pub u8, pub u8, pub u8);

#[allow(dead_code)]
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub fn from_html(html: &str) -> io::Result<Self> {
        let err = io::Error::new(io::ErrorKind::InvalidInput, "Invalid HTML color code");

        if html.len() != 7 {
            Err(err)
        } else if !html.starts_with("#") {
            Err(err)
        } else {
            let r_html = html[1..3].to_owned();
            let g_html = html[3..5].to_owned();
            let b_html = html[5..7].to_owned();

            let r = match u8::from_str_radix(&r_html, 16) {
                Ok(value) => Ok(value),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid HTML color code",
                )),
            }?;
            let g = match u8::from_str_radix(&g_html, 16) {
                Ok(value) => Ok(value),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid HTML color code",
                )),
            }?;
            let b = match u8::from_str_radix(&b_html, 16) {
                Ok(value) => Ok(value),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Invalid HTML color code",
                )),
            }?;

            Ok(Self(r, g, b))
        }
    }

    pub fn to_html(&self) -> String {
        format!("#{:x}{:x}{:x}", self.0, self.1, self.2)
    }

    pub fn r(&self) -> u8 {
        self.0
    }

    pub fn g(&self) -> u8 {
        self.1
    }

    pub fn b(&self) -> u8 {
        self.2
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandArgs<T> {
    pub request: T,
}

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
