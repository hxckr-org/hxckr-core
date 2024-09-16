use crate::service::database::{
    conn::get_connection_pool,
    models::{Repository, Session},
};
use anyhow::{Context, Result};

pub async fn match_repo_for_webhook(repo_url: &str) -> Result<Session> {
    let pool = get_connection_pool();
    let mut conn = pool.get().context("Failed to get connection from pool")?;
    let repo = Repository::get_repo(&mut conn, None, None, None, Some(repo_url))
        .context(format!("Failed to find repository with URL: {}", repo_url))?;
    let repo = repo
        .first()
        .ok_or_else(|| anyhow::anyhow!("Repository not found with URL: {}", repo_url))?;

    let session = Session::get_by_userid(&mut conn, &repo.user_id).context(format!(
        "Failed to find session for user ID: {}",
        repo.user_id
    ))?;
    let session = session.context(format!("Session not found for user ID: {}", repo.user_id))?;

    Ok(session)
}
