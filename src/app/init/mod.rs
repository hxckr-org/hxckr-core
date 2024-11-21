use log::{error, info};

use crate::service::database::{
    conn::DbPool,
    models::{Leaderboard, User},
};

pub async fn initialize_leaderboards(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.get()?;

    // Get all users
    let users = User::get_all_users(&mut conn)?;

    if users.is_empty() {
        info!("No users found in database - skipping leaderboard initialization");
        return Ok(());
    }

    let mut initialized_count = 0;
    for user in users {
        // Check if user has a leaderboard entry
        match Leaderboard::get_leaderboard(&mut conn, Some(&user.id)) {
            Ok(leaderboard) => {
                if leaderboard.is_empty() {
                    // Create new leaderboard for user
                    let new_leaderboard = Leaderboard::new(&user.id, None, 0, 0);
                    Leaderboard::create(&mut conn, new_leaderboard)?;
                    initialized_count += 1;
                }
            }
            Err(e) => {
                error!("Error checking leaderboard for user {}: {}", user.id, e);
                // Create new leaderboard for user
                let new_leaderboard = Leaderboard::new(&user.id, None, 0, 0);
                Leaderboard::create(&mut conn, new_leaderboard)?;
                initialized_count += 1;
            }
        }
    }

    if initialized_count > 0 {
        info!(
            "Initialized {} missing leaderboard records",
            initialized_count
        );
    } else {
        info!("All users have leaderboard records - no initialization needed");
    }

    Ok(())
}
