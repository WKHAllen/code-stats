use super::super::types::AppConfig;
use super::local_storage;

pub fn load_config() -> AppConfig {
    AppConfig {
        recent_paths: local_storage::get_item("recent_paths")
            .unwrap_or(Ok(vec![]))
            .unwrap(),
    }
}

pub fn save_config(config: &AppConfig) {
    local_storage::set_item("recent_paths", &config.recent_paths).unwrap();
}
