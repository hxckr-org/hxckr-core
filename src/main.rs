use anyhow::Result;
use env_logger::Env;
use service::database::{conn::establish_connection, models::Progress};

mod schema;
mod service;
mod shared;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let connection = &mut establish_connection()?;

    // let new_user_1 = User::new(
    //     "extheo",
    //     "test@test.com",
    //     "extheoisah",
    //     "https://avatars.githubusercontent.com/u/60826700?v=4",
    //     "admin",
    // );
    // let new_user_2 = User::new(
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
    // let challenge = match get_challenge(
    //     connection,
    //     "00000000-0000-0000-0000-000000000000".to_string(),
    // ) {
    //     Ok(challenge) => challenge,
    //     Err(e) => {
    //         println!("Error getting challenge: {}", e);
    //         return Err(e);
    //     }
    // };
    // println!("Challenge: {:?}", challenge);

    // let new_exercise_1 = Exercise::new(
    //     "Exercise 1",
    //     "Exercise 1 description",
    //     Difficulty::Easy.to_str(),
    //     "https://github.com/extheo/extheo/tree/main/challenges/challenge_1/exercise_1",
    //     "0d420322-7d8a-4fbd-9a78-6636da0f3ec5",
    // );

    // let new_exercise_2 = Exercise::new(
    //     "Exercise 2",
    //     "Exercise 2 description",
    //     Difficulty::Easy.to_str(),
    //     "https://github.com/extheo/extheo/tree/main/challenges/challenge_1/exercise_2",
    //     "0d420322-7d8a-4fbd-9a78-6636da0f3ec5",
    // );

    // for new_exercise in [new_exercise_1, new_exercise_2] {
    //     match create_exercise(connection, new_exercise) {
    //         Ok(exercise) => println!("Exercise created successfully: {:?}", exercise),
    //         Err(e) => println!("Error creating exercise: {}", e),
    //     }
    // }

    // let exercise = match Exercise::get_exercise(
    //     connection,
    //     None,
    //     Some("0d420322-7d8a-4fbd-9a78-6636da0f3ec5".to_string()),
    // ) {
    //     Ok(exercise) => exercise,
    //     Err(e) => {
    //         println!("Error getting exercise: {}", e);
    //         return Err(e);
    //     }
    // };
    // println!("Exercise: {:?}", exercise);

    // let new_progress = Progress::new(
    //     "ac02c63b-ab39-4248-976f-1e2e415a8574",
    //     "0d420322-7d8a-4fbd-9a78-6636da0f3ec5",
    //     ProgressStatus::InProgress.to_str(),
    // );
    // let progress = match Progress::create_progress(connection, new_progress) {
    //     Ok(progress) => progress,
    //     Err(e) => {
    //         println!("Error creating progress: {}", e);
    //         return Err(e);
    //     }
    // };
    // println!("Progress created: {:?}", progress);

    let progress = match Progress::get_progress(
        connection,
        None,
        Some("ac02c63b-ab39-4248-976f-1e2e415a8574".to_string()),
        None,
    ) {
        Ok(progress) => progress,
        Err(e) => {
            println!("Error getting progress: {}", e);
            return Err(e);
        }
    };
    println!("Progress found: {:?}", progress);

    Ok(())
}
