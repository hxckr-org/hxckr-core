use std::str::FromStr;
use uuid::Uuid;

pub fn string_to_uuid(id: &str) -> Result<Uuid, &'static str> {
    Uuid::from_str(id).map_err(|_| "Invalid UUID")
}
