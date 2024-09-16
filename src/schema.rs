// @generated automatically by Diesel CLI.

diesel::table! {
    badges (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    challenges (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        #[max_length = 255]
        repo_url -> Varchar,
        #[max_length = 255]
        difficulty -> Varchar,
        #[max_length = 255]
        mode -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    exercises (id) {
        id -> Uuid,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        #[max_length = 255]
        difficulty -> Varchar,
        #[max_length = 255]
        test_runner -> Varchar,
        challenge_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 255]
        status -> Varchar,
    }
}

diesel::table! {
    leaderboard (id) {
        id -> Int4,
        user_id -> Uuid,
        achievements -> Nullable<Jsonb>,
        score -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    progress (id) {
        id -> Uuid,
        user_id -> Uuid,
        challenge_id -> Uuid,
        status -> Varchar,
        progress_details -> Nullable<Jsonb>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    repositories (id) {
        id -> Uuid,
        user_id -> Uuid,
        challenge_id -> Uuid,
        #[max_length = 255]
        repo_url -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        soft_serve_url -> Text,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Text,
        #[max_length = 255]
        provider -> Varchar,
        created_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    submissions (id) {
        id -> Uuid,
        user_id -> Uuid,
        exercise_id -> Uuid,
        #[max_length = 255]
        commit_id -> Varchar,
        repository_id -> Uuid,
        #[max_length = 255]
        status -> Varchar,
        feedback -> Nullable<Text>,
        submitted_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_badges (id) {
        id -> Int4,
        user_id -> Uuid,
        badge_id -> Int4,
        awarded_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        github_username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        profile_pic_url -> Varchar,
        #[max_length = 255]
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(exercises -> challenges (challenge_id));
diesel::joinable!(leaderboard -> users (user_id));
diesel::joinable!(progress -> challenges (challenge_id));
diesel::joinable!(progress -> users (user_id));
diesel::joinable!(repositories -> challenges (challenge_id));
diesel::joinable!(repositories -> users (user_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(submissions -> exercises (exercise_id));
diesel::joinable!(submissions -> repositories (repository_id));
diesel::joinable!(submissions -> users (user_id));
diesel::joinable!(user_badges -> badges (badge_id));
diesel::joinable!(user_badges -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    badges,
    challenges,
    exercises,
    leaderboard,
    progress,
    repositories,
    sessions,
    submissions,
    user_badges,
    users,
);
