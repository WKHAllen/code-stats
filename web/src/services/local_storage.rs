use serde::{de::DeserializeOwned, ser::Serialize};
use web_sys::Storage;

fn local_storage() -> Storage {
    web_sys::window().unwrap().local_storage().unwrap().unwrap()
}

pub fn get_item<D>(key: &str) -> Option<serde_json::Result<D>>
where
    D: DeserializeOwned,
{
    match local_storage().get_item(key).unwrap() {
        Some(value) => Some(serde_json::from_str(value.as_str())),
        None => None,
    }
}

pub fn set_item<S>(key: &str, value: S) -> serde_json::Result<()>
where
    S: Serialize,
{
    match serde_json::to_string(&value) {
        Ok(string_value) => Ok(local_storage().set_item(key, &string_value).unwrap()),
        Err(err) => Err(err),
    }
}

#[allow(dead_code)]
pub fn remove_item(key: &str) {
    local_storage().remove_item(key).unwrap()
}

#[allow(dead_code)]
pub fn clear() {
    local_storage().clear().unwrap()
}
