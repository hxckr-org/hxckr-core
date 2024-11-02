use actix_ws::Message;
use anyhow::{Error, Result};
use futures_util::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use log::error;
use serde::{Deserialize, Serialize};

use crate::app::{
    progress::update_progress::update_progress, repo::match_repo::match_repo_for_webhook,
    websockets::manager::WebSocketManagerHandle,
};

#[derive(Debug, Deserialize, Serialize)]
struct WebhookHandlerConsumerEvent {
    event_type: String,
    #[serde(rename = "repoUrl")]
    repo_url: Option<String>,
    branch: Option<String>,
    #[serde(rename = "commitSha")]
    commit_sha: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TestRunnerConsumerEvent {
    event_type: String,
    #[serde(rename = "commitSha")]
    commit_sha: String,
    #[serde(rename = "repoUrl")]
    repo_url: String,
    success: bool,
    output: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TestRunnerWrapper {
    result: TestRunnerConsumerEvent,
}

pub async fn consume_queue(manager_handle: WebSocketManagerHandle) -> Result<(), Error> {
    let rabbitmq_url = std::env::var("RABBITMQ_URL").map_err(|_| {
        error!("RABBITMQ_URL is not set");
        anyhow::anyhow!("RabbitMQ URL is not set")
    })?;
    let webhook_handler_rabbitmq_queue_name = std::env::var("WEBHOOK_HANDLER_RABBITMQ_QUEUE_NAME")
        .map_err(|_| {
            error!("WEBHOOK_HANDLER_RABBITMQ_QUEUE_NAME is not set");
            anyhow::anyhow!("Webhook handler RabbitMQ queue name is not set")
        })?;
    let test_runner_rabbitmq_queue_name = std::env::var("TEST_RUNNER_RABBITMQ_QUEUE_NAME")
        .map_err(|_| {
            error!("TEST_RUNNER_RABBITMQ_QUEUE_NAME is not set");
            anyhow::anyhow!("Test runner RabbitMQ queue name is not set")
        })?;

    let conn = Connection::connect(&rabbitmq_url, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    let webhook_handle = manager_handle.clone();
    let webhook_channel = channel.clone();
    let test_handle = manager_handle.clone();
    let test_channel = channel;

    // Spawn two separate tasks for concurrent queue processing
    let webhook_consumer = tokio::spawn(async move {
        consume_webhook_queue(
            webhook_channel,
            webhook_handle,
            &webhook_handler_rabbitmq_queue_name,
        )
        .await
    });

    let test_consumer = tokio::spawn(async move {
        consume_test_queue(test_channel, test_handle, &test_runner_rabbitmq_queue_name).await
    });

    // Wait for both consumers
    tokio::try_join!(webhook_consumer, test_consumer)?;

    Ok(())
}

async fn consume_webhook_queue(
    channel: lapin::Channel,
    manager_handle: WebSocketManagerHandle,
    queue_name: &str,
) -> Result<(), Error> {
    let mut consumer = channel
        .basic_consume(
            queue_name,
            "backend_consumer_webhook_handler",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let webhook_handler_payload: WebhookHandlerConsumerEvent =
                serde_json::from_slice(&delivery.data)?;
            let message = serde_json::to_string(&webhook_handler_payload)?;
            match webhook_handler_payload.event_type.to_lowercase().as_str() {
                "push" => {
                    let repo_url = match webhook_handler_payload.repo_url.as_ref() {
                        Some(url) if !url.is_empty() => url,
                        _ => {
                            error!("Repository URL is missing or empty");
                            return Err(anyhow::anyhow!("Repository URL is missing or empty"));
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
    }
    Ok(())
}

async fn consume_test_queue(
    channel: lapin::Channel,
    manager_handle: WebSocketManagerHandle,
    queue_name: &str,
) -> Result<(), Error> {
    let mut consumer = channel
        .basic_consume(
            queue_name,
            "backend_consumer_test_runner",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(delivery) = consumer.next().await {
        if let Ok(delivery) = delivery {
            let wrapper: TestRunnerWrapper = serde_json::from_slice(&delivery.data)?;
            let test_runner_payload = wrapper.result;

            if test_runner_payload.repo_url.is_empty() {
                error!("Repository URL is missing or empty");
                return Err(anyhow::anyhow!("Repository URL is missing or empty"));
            }
            let session = match match_repo_for_webhook(&test_runner_payload.repo_url).await {
                Ok(session) => session,
                Err(e) => {
                    error!("Failed to match repository: {:?}", e);
                    return Err(anyhow::anyhow!("Repository not found"));
                }
            };
            if test_runner_payload.success {
                let updated_progress = match update_progress(&session.user_id).await {
                    Ok(progress) => progress,
                    Err(e) => {
                        error!("Failed to update progress: {:?}", e);
                        return Err(anyhow::anyhow!("Failed to update progress"));
                    }
                };
                let combined_message =
                    serde_json::to_string(&(test_runner_payload, updated_progress))?;
                if let Err(e) = manager_handle
                    .broadcast_to_session(&session.token, Message::Text(combined_message.into()))
                    .await
                {
                    error!("Failed to broadcast test runner event: {:?}", e);
                    return Err(anyhow::anyhow!("Failed to broadcast test runner event"));
                }
            } else {
                let message = serde_json::to_string(&test_runner_payload)?;
                if let Err(e) = manager_handle
                    .broadcast_to_session(&session.token, Message::Text(message.into()))
                    .await
                {
                    error!("Failed to broadcast test runner event: {:?}", e);
                    return Err(anyhow::anyhow!("Failed to broadcast test runner event"));
                }
            }
            delivery.ack(BasicAckOptions::default()).await?;
        }
    }
    Ok(())
}
