use crate::service::database::{
    conn::get_connection_pool,
    models::{Challenge, Leaderboard, Progress, Repository},
};
use crate::shared::primitives::Status;
use anyhow::{Context, Result};
use log::warn;
use serde_json::json;
use uuid::Uuid;

pub async fn update_progress(soft_serve_url: &str, user_id: &Uuid) -> Result<Progress> {
    let pool = get_connection_pool();
    let mut conn = pool.get().context("Failed to get connection from pool")?;
    let repo = Repository::get_repo(&mut conn, None, None, None, Some(soft_serve_url)).context(
        format!("Failed to find repository with URL: {}", soft_serve_url),
    )?;
    let repo = repo
        .first()
        .ok_or_else(|| anyhow::anyhow!("Repository not found with URL: {}", soft_serve_url))?;

    let challenge = Challenge::get_challenge(&mut conn, Some(&repo.challenge_id), None, None, None)
        .context(format!(
            "Failed to find challenge for challenge ID: {}",
            repo.challenge_id
        ))?;

    let progress = Progress::get_progress(&mut conn, None, None, None, Some(&repo.id))
        .context(format!("Failed to find progress for user ID: {}", user_id))?;
    if progress.status == Status::Completed.to_str().to_string() {
        warn!("Challenge is already completed");
        return Ok(progress.clone());
    }

    // check progress.progress_details object for "current_step"
    // if new_current_step is equal to module_count, then set status to completed
    let current_step = progress
        .progress_details
        .as_ref()
        .and_then(|details| details["current_step"].as_i64())
        .unwrap_or(0);
    let mut new_status = Status::InProgress.to_str().to_string();
    let new_current_step = current_step + 1;
    let module_count = challenge.module_count as i64;

    if new_current_step == module_count {
        new_status = Status::Completed.to_str().to_string();
    }

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

    // update leaderboard
    if new_status != Status::Completed.to_str().to_string() {
        let user_leaderboard = Leaderboard::get_leaderboard(&mut conn, Some(user_id))
            .context("Failed to get leaderboard")?;

        let new_score = user_leaderboard[0].score + 1;
        Leaderboard::update(&mut conn, user_id, Some(new_score), None, None)
            .context("Failed to update leaderboard")?;
    }
    Ok(updated_progress)
}
