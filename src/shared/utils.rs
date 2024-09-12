use actix_ws::{Item, Message};
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

pub fn clone_websocket_message(msg: &Message) -> Message {
    match msg {
        Message::Text(text) => Message::Text(text.clone()),
        Message::Binary(bin) => Message::Binary(bin.clone()),
        Message::Ping(bytes) => Message::Ping(bytes.clone()),
        Message::Pong(bytes) => Message::Pong(bytes.clone()),
        Message::Close(reason) => Message::Close(reason.clone()),
        Message::Continuation(item) => Message::Continuation(clone_websocket_item(item)),
        Message::Nop => Message::Nop,
    }
}

pub fn clone_websocket_item(item: &Item) -> Item {
    match item {
        Item::FirstText(bytes) => Item::FirstText(bytes.clone()),
        Item::FirstBinary(bytes) => Item::FirstBinary(bytes.clone()),
        Item::Continue(bytes) => Item::Continue(bytes.clone()),
        Item::Last(bytes) => Item::Last(bytes.clone()),
    }
}
