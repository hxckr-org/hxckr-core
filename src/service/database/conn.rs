use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> Result<PgConnection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let connection = PgConnection::establish(&database_url).context(format!(
        "failed to connect to database at url: {}",
        database_url
    ))?;
    Ok(connection)
}
