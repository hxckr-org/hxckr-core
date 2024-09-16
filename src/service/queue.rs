use actix_ws::Message;
use anyhow::{Error, Result};
use futures_util::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use log::error;
use serde::{Deserialize, Serialize};

use crate::app::{
    repo::match_repo::match_repo_for_webhook, websockets::manager::WebSocketManagerHandle,
};

#[derive(Debug, Deserialize, Serialize)]
struct ConsumerEvent {
    event_type: String,
    #[serde(rename = "repoUrl")]
    repo_url: Option<String>,
    branch: Option<String>,
    #[serde(rename = "commitSha")]
    commit_sha: Option<String>,
}

pub async fn consume_queue(manager_handle: WebSocketManagerHandle) -> Result<(), Error> {
    let rabbitmq_url = std::env::var("RABBITMQ_URL").map_err(|_| {
        error!("RABBITMQ_URL is not set");
        anyhow::anyhow!("RabbitMQ URL is not set")
    })?;
    let rabbitmq_queue_name = std::env::var("RABBITMQ_QUEUE_NAME").map_err(|_| {
        error!("RABBITMQ_QUEUE_NAME is not set");
        anyhow::anyhow!("RabbitMQ queue name is not set")
    })?;

    let conn = Connection::connect(&rabbitmq_url, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;
    let mut consumer = channel
        .basic_consume(
            &rabbitmq_queue_name,
            "backend_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            match serde_json::from_slice(&delivery.data) {
                Ok(payload) => {
                    let payload: ConsumerEvent = payload;
                    let message = serde_json::to_string(&payload).map_err(|e| {
                        error!("Failed to serialize payload: {:?}", e);
                        anyhow::anyhow!("Failed to serialize payload")
                    })?;
                    match payload.event_type.to_lowercase().as_str() {
                        "push" => {
                            let repo_url = match payload.repo_url.as_ref() {
                                Some(url) if !url.is_empty() => url,
                                _ => {
                                    error!("Repository URL is missing or empty");
                                    return Err(anyhow::anyhow!(
                                        "Repository URL is missing or empty"
                                    ));
                                }
                            };
                            let session = match match_repo_for_webhook(repo_url).await {
                                Ok(session) => session,
                                Err(e) => {
                                    error!("Failed to match repository: {:?}", e);
                                    return Err(anyhow::anyhow!("Repository not found"));
                                }
                            };
                            // send the event through the websocket to the client
                            if let Err(e) = manager_handle
                                .broadcast_to_session(&session.token, Message::Text(message.into()))
                                .await
                            {
                                error!("Failed to broadcast webhook event: {:?}", e);
                                return Err(anyhow::anyhow!("Failed to broadcast webhook event"));
                            }
                        }
                        _ => {
                            error!("Invalid event type");
                        }
                    }
                    delivery.ack(BasicAckOptions::default()).await?;
                }
                Err(e) => {
                    error!("Failed to deserialize payload: {:?}", e);
                    error!(
                        "Received invalid data from queue: {:?}",
                        String::from_utf8_lossy(&delivery.data)
                    );
                    delivery.nack(BasicNackOptions::default()).await?;
                }
            }
        }
    }

    Ok(())
}
