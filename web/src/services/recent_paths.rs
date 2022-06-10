use super::super::types::AppConfig;
use super::config;
use std::path::PathBuf;

pub const MAX_RECENT_PATHS: usize = 16;

#[allow(dead_code)]
pub fn get_recent_paths() -> Vec<PathBuf> {
    config::load_config().recent_paths
}

pub fn add_recent_path(app_config: &AppConfig, path: &PathBuf) -> AppConfig {
    let mut app_config_mut = app_config.clone();
    app_config_mut.recent_paths.retain(|p| *p != path.clone());
    app_config_mut.recent_paths.insert(0, path.clone());

    if app_config_mut.recent_paths.len() > MAX_RECENT_PATHS {
        app_config_mut.recent_paths = app_config_mut.recent_paths[..MAX_RECENT_PATHS].to_vec();
    }

    config::save_config(&app_config_mut);
    app_config_mut
}

pub fn remove_recent_path(app_config: &AppConfig, path: &PathBuf) -> AppConfig {
    let mut app_config_mut = app_config.clone();
    app_config_mut.recent_paths.retain(|p| *p != path.clone());

    config::save_config(&app_config_mut);
    app_config_mut
}
