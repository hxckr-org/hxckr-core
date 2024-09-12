use crate::app::{
    repo::match_repo::match_repo_for_webhook, websockets::manager::WebSocketManagerHandle,
};
use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use actix_ws::Message;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct PushEvent {
    #[serde(rename = "repoUrl")]
    repo_url: Option<String>,
    #[serde(rename = "branchUrl")]
    branch_url: Option<String>,
    #[serde(rename = "commitSha")]
    commit_sha: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct TestEvent {
    outcome: bool,
    error: Option<String>,
    message: String,
    test_name: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct WebhookEvent {
    event_type: String,
    payload: serde_json::Value,
}

pub async fn handle_webhook(
    event: web::Json<WebhookEvent>,
    manager_handle: web::Data<WebSocketManagerHandle>,
) -> impl Responder {
    let message = serde_json::to_string(&event.0).unwrap_or_default();
    log::info!("Webhook event received: {:?}", message);

    // check the event type and send to the corresponding websocket session
    // all webhook events should be sent to the user's session
    // the websocket server will then route the event to the appropriate client
    match event.event_type.to_lowercase().as_str() {
        "push" => {
            let push_event: PushEvent =
                serde_json::from_value(event.payload.clone()).unwrap_or_default();
            let repo_url = match push_event.repo_url.as_ref() {
                Some(url) if !url.is_empty() => url,
                _ => {
                    log::error!("Repository URL is missing or empty");
                    return HttpResponse::build(StatusCode::BAD_REQUEST).json(json!({
                        "error": "Repository URL is missing or empty"
                    }));
                }
            };

            let session = match match_repo_for_webhook(repo_url).await {
                Ok(session) => session,
                Err(e) => {
                    log::error!("Failed to match repository: {:?}", e);
                    return HttpResponse::build(StatusCode::NOT_FOUND).json(json!({
                        "error": "Repository not found"
                    }));
                }
            };
            log::info!("Matched repository: {:?}", session);
            if let Err(e) = manager_handle
                .broadcast_to_session(&session.token, Message::Text(message.into()))
                .await
            {
                log::error!("Failed to broadcast webhook event: {:?}", e);
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish();
            }
        }
        "test" => {
            let test_event: TestEvent =
                serde_json::from_value(event.payload.clone()).unwrap_or_default();
            log::info!("Test event: {:?}", test_event);
            // TODO: match test event using test_name to a user's session and broadcast to the client
        }
        _ => {
            log::error!("Unknown event type: {:?}", event.event_type);
            return HttpResponse::build(StatusCode::BAD_REQUEST).json(json!({
                "error": "Unknown event type"
            }));
        }
    }

    HttpResponse::Ok().finish()
}
