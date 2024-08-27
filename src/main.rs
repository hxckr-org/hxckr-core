use anyhow::Result;
use env_logger::Env;
use service::{
    database::{conn::establish_connection, models::Challenge},
    repository::challenge::{create_challenge, get_challenge},
};
use shared::primitives::{ChallengeMode, Difficulty};

mod schema;
mod service;
mod shared;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let connection = &mut establish_connection()?;

    // let new_user_1 = NewUser::new(
    //     "extheo",
    //     "test@test.com",
    //     "extheoisah",
    //     "https://avatars.githubusercontent.com/u/60826700?v=4",
    //     "admin",
    // );
    // let new_user_2 = NewUser::new(
    //     "extheo2",
    //     "test2@test.com",
    //     "extheoisah2",
    //     "https://avatars.githubusercontent.com/u/60826700?v=4",
    //     "admin",
    // );

    // for new_user in [new_user_1, new_user_2] {
    //     match create_user(connection, new_user) {
    //         Ok(user) => println!("User created successfully: {:?}", user),
    //         Err(e) => println!("Error creating user: {}", e),
    //     }
    // }

    // let user_to_get = get_user(connection, Some("extheo"), None, None);
    // println!("User to get: {:?}", user_to_get);

    // let new_challenge = Challenge::new(
    //     "Challenge 1",
    //     "Challenge 1 description",
    //     "https://github.com/extheo/extheo/tree/main/challenges/challenge_1",
    //     Difficulty::Easy.to_str(),
    //     ChallengeMode::Project.to_str(),
    // );
    // let challenge = match create_challenge(connection, new_challenge) {
    //     Ok(challenge) => challenge,
    //     Err(e) => {
    //         println!("Error creating challenge: {}", e);
    //         return Err(e);
    //     }
    // };
    // println!("Challenge created: {:?}", challenge);
    let challenge = match get_challenge(
        connection,
        "00000000-0000-0000-0000-000000000000".to_string(),
    ) {
        Ok(challenge) => challenge,
        Err(e) => {
            println!("Error getting challenge: {}", e);
            return Err(e);
        }
    };
    println!("Challenge: {:?}", challenge);
    Ok(())
}
