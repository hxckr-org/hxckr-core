use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::app::websockets::manager::WebSocketManagerHandle;

#[derive(Deserialize, Serialize)]
pub struct WebhookEvent {
    event_type: String,
    payload: serde_json::Value,
}

pub async fn handle_webhook(
    event: web::Json<WebhookEvent>,
    manager_handle: web::Data<WebSocketManagerHandle>,
) -> impl Responder {
    let message = serde_json::to_string(&event.0).unwrap_or_default();
    
    if let Err(e) = manager_handle
        .broadcast_to_all(actix_ws::Message::Text(message.into()))
        .await
    {
        log::error!("Failed to broadcast webhook event: {:?}", e);
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}