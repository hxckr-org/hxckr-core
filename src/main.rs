use anyhow::Result;
use env_logger::Env;
use service::database::{
    conn::establish_connection,
    models::{Leaderboard, User},
};

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

    let user = match User::get_user(connection, Some("extheo2"), None, None) {
        Ok(user) => user,
        Err(e) => {
            println!("Error getting user: {}", e);
            return Err(e);
        }
    };

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

    // let progress = match Progress::get_progress(
    //     connection,
    //     None,
    //     Some("ac02c63b-ab39-4248-976f-1e2e415a8574".to_string()),
    //     None,
    // ) {
    //     Ok(progress) => progress,
    //     Err(e) => {
    //         println!("Error getting progress: {}", e);
    //         return Err(e);
    //     }
    // };
    // println!("Progress found: {:?}", progress);

    // let new_repository = Repository::new(
    //     &user.id.to_string(),
    //     "0d420322-7d8a-4fbd-9a78-6636da0f3ec5",
    //     "https://github.com/extheo/extheo",
    // );

    // let repository = match Repository::create_repo(connection, new_repository) {
    //     Ok(repository) => repository,
    //     Err(e) => {
    //         println!("Error creating repository: {}", e);
    //         return Err(e);
    //     }
    // };
    // println!("Repository created: {:?}", repository);

    // let new_submission = Submission::new(
    //     "03c985c7-2923-4a9c-ac75-700bc6bc6a8b",
    //     &user.id.to_string(),
    //     SubmissionStatus::Pending,
    //     &repository.id.to_string(),
    //     "03c985c729234a9cac75700bc6bc6a8b",
    // );

    // let submission = match Submission::create_submission(connection, new_submission) {
    //     Ok(submission) => submission,
    //     Err(e) => {
    //         println!("Error creating submission: {}", e);
    //         return Err(e);
    //     }
    // };
    // println!("Submission created: {:?}", submission);

    // let new_leaderboard = Leaderboard::new(&user.id.to_string(), None, 200);

    // let leaderboard = match Leaderboard::create(connection, new_leaderboard) {
    //     Ok(leaderboard) => leaderboard,
    //     Err(e) => {
    //         println!("Error getting leaderboard: {}", e);
    //         return Err(e);
    //     }
    // };
    // println!("Leaderboard created: {:?}", leaderboard);

    let user_leaderboard = match Leaderboard::get_leaderboard(connection, Some(user.id.to_string()))
    {
        Ok(leaderboard) => leaderboard,
        Err(e) => {
            println!("Error getting leaderboard: {}", e);
            return Err(e);
        }
    };
    println!("user leaderboard: {:?}", &user_leaderboard);

    let data = r#"
        {
            "achievement_1": false,
            "achievement_2": false,
            "achievement_3": [
                "achievement_3_1",
                "achievement_3_2",
                "achievement_3_3"
            ]
        }"#;
    let v: serde_json::Value = serde_json::from_str(data)?;

    let updated_leaderboard =
        match Leaderboard::update(connection, &user.id.to_string(), 300, Some(v)) {
            Ok(leaderboard) => leaderboard,
            Err(e) => {
                println!("Error updating leaderboard: {}", e);
                return Err(e);
            }
        };
    println!("updated leaderboard: {:?}", &updated_leaderboard);
    Ok(())
}
