use rand::{distributions::Alphanumeric, Rng};
use std::str::FromStr;
use uuid::Uuid;

pub fn string_to_uuid(id: &str) -> Result<Uuid, &'static str> {
    Uuid::from_str(id).map_err(|_| "Invalid UUID")
}

pub fn generate_session_token() -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    format!("hxckr_{}", random_string)
}
