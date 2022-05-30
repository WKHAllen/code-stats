use serde::{Deserialize, Serialize};
use std::io;

pub struct AppConfig {
    pub message: String,
}

pub struct Color(pub u8, pub u8, pub u8);

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
pub struct CodeStatsRequest {
    pub path: String,
    pub exclude_dirs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeStatsResponse {
    pub request: CodeStatsRequest,
}
