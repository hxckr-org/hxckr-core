use actix_web::{web, Error, HttpMessage, HttpRequest, HttpResponse};
use actix_ws::{Message, Session};
use futures_util::StreamExt;
use std::time::Duration;
use tokio::time::{interval, Instant};

use super::manager::{ConnId, SessionToken, WebSocketManagerHandle};
use crate::app::auth::middleware::SessionInfo;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn websocket_handler(
    req: HttpRequest,
    body: web::Payload,
    manager_handle: web::Data<WebSocketManagerHandle>,
) -> Result<HttpResponse, Error> {
    let (response, session, mut msg_stream) = actix_ws::handle(&req, body)?;

    let session_token = match req.extensions().get::<SessionInfo>() {
        Some(session_info) => session_info.token.clone(),
        None => {
            return Err(Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unauthorized",
            )))
        }
    };

    let conn_id = manager_handle
        .connect(&session_token, &session)
        .await
        .map_err(Error::from)?;

    log::info!("WebSocket connected: Connection ID {:?}", conn_id);

    actix_web::rt::spawn(async move {
        let mut last_heartbeat = Instant::now();
        let mut interval = interval(HEARTBEAT_INTERVAL);
        let mut session = Some(session);

        loop {
            tokio::select! {
                Some(msg) = msg_stream.next() => {
                    match msg {
                        Ok(msg) => {
                            if handle_message(msg, &mut session, &manager_handle, conn_id, &session_token).await.is_err() {
                                break;
                            }
                            last_heartbeat = Instant::now();
                        }
                        Err(e) => {
                            log::error!("Error in websocket: {:?}", e);
                            break;
                        }
                    }
                }
                _ = interval.tick() => {
                    if Instant::now().duration_since(last_heartbeat) > CLIENT_TIMEOUT {
                        log::info!("Client timeout: Connection ID {:?}", conn_id);
                        break;
                    }
                    if let Some(session) = session.as_mut() {
                        if session.ping(b"").await.is_err() {
                            break;
                        }
                    }
                }
                else => break,
            }
        }

        log::info!("WebSocket connection closed: Connection ID {:?}", conn_id);
        manager_handle.disconnect(conn_id).await.ok();
    });

    Ok(response)
}

async fn handle_message(
    msg: Message,
    session: &mut Option<Session>,
    manager_handle: &WebSocketManagerHandle,
    conn_id: ConnId,
    session_token: &SessionToken,
) -> Result<(), Error> {
    match msg {
        Message::Ping(bytes) => {
            if let Some(session) = session.as_mut() {
                session
                    .pong(&bytes)
                    .await
                    .map_err(|e| Error::from(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
            }
        }
        Message::Pong(_) => {
            manager_handle.update_heartbeat(conn_id).await?;
        }
        Message::Text(text) => {
            log::info!("Text message received: {:?}", text);
            let message = Message::Text(text);
            manager_handle
                .send_message_to_session(session_token, message, conn_id)
                .await?;
        }
        Message::Binary(bin) => {
            log::info!("Binary message received: {:?}", bin);
            let message = Message::Binary(bin);
            manager_handle
                .send_message_to_session(session_token, message, conn_id)
                .await?;
        }
        Message::Close(reason) => {
            log::info!("Close message received: {:?}", reason);
            manager_handle.disconnect(conn_id).await?;
            if let Some(sess) = session.take() {
                sess.close(reason)
                    .await
                    .map_err(|e| Error::from(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
            }
            return Ok(());
        }
        _ => {}
    }
    Ok(())
}
