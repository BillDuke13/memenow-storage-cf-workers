use chrono::Utc;
use crate::models::UserRole;
use crate::config::Config;
use std::path::Path;
use rand::Rng;
use worker::*;

pub fn generate_r2_key(config: &Config, user_role: &UserRole, user_id: &str, content_type: &str, file_name: &str) -> String {
    let date = Utc::now().format("%Y%m%d").to_string();
    let role = user_role.as_str().to_lowercase();
    let content_category = content_type.split('/').next().unwrap_or("unknown");

    let file_path = Path::new(file_name);
    let file_stem = file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let file_extension = file_path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let unique_id = generate_unique_identifier();

    let mut key = config.path_format.clone();
    key = key.replace("{role}", &role);
    key = key.replace("{user_id}", user_id);
    key = key.replace("{date}", &date);
    key = key.replace("{content_type}", content_category);
    key = key.replace("{unique_id}", &unique_id);
    key = key.replace("{extension}", file_extension);
    key = key.replace("{file_name}", file_stem);

    key
}

pub fn generate_unique_identifier() -> String {
    let random_part: u64 = rand::thread_rng().gen();
    format!("{:x}", random_part)
}

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

pub fn json_response<T: serde::Serialize>(data: &T) -> Result<Response> {
    Response::from_json(data)
}

pub fn calculate_etag(data: &[u8]) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

pub fn generate_request_id() -> String {
    format!("{}-{}", Utc::now().timestamp_millis(), generate_unique_identifier())
}
