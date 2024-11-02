use crate::service::database::{
    conn::get_connection_pool,
    models::{Challenge, Progress},
};
use crate::shared::primitives::Status;
use anyhow::{Context, Result};
use serde_json::json;
use uuid::Uuid;

pub async fn update_progress(user_id: &Uuid) -> Result<Progress> {
    let pool = get_connection_pool();
    let mut conn = pool.get().context("Failed to get connection from pool")?;
    let progress = Progress::get_progress(&mut conn, None, Some(user_id), None)
        .context(format!("Failed to find progress for user ID: {}", user_id))?;
    let progress = progress
        .first()
        .ok_or_else(|| anyhow::anyhow!("Progress not found for user ID: {}", user_id))?;
    let challenge = Challenge::get_challenge(&mut conn, Some(&progress.challenge_id), None)
        .context(format!(
            "Failed to find challenge for challenge ID: {}",
            progress.challenge_id
        ))?;
    if progress.status == Status::Completed.to_str().to_string() {
        return Err(anyhow::anyhow!("Challenge is already completed"));
    }

    // check progress.progress_details object for "current_step"
    // if current_step is equal to module_count, then set status to completed
    // else, set current_step to current_step + 1
    let current_step = progress
        .progress_details
        .as_ref()
        .and_then(|details| details["current_step"].as_i64())
        .unwrap_or(0);
    let mut new_status = Status::InProgress.to_str().to_string();
    let mut new_current_step = current_step;
    let module_count = challenge.module_count as i64;
    println!("Module count: {}", &module_count);
    println!("Is module count equal to current step: {}", module_count == current_step);
    if current_step == module_count {
        new_status = Status::Completed.to_str().to_string();
        println!("Challenge is completed. Setting status to completed");
    } else {
        new_current_step = current_step + 1;
    }
    println!("New current step: {}", new_current_step);
    println!("New status: {}", new_status);
    let updated_progress = Progress::update_progress(
        &mut conn,
        &progress.id,
        Status::from_str(&new_status).expect("Invalid status"),
        Some(json!({ "current_step": new_current_step })),
    )
    .context(format!(
        "Failed to update progress for user ID: {}",
        user_id
    ))?;
    Ok(updated_progress)
}
