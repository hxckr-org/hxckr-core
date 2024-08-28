// @generated automatically by Diesel CLI.

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
diesel::joinable!(progress -> challenges (challenge_id));
diesel::joinable!(progress -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    challenges,
    exercises,
    progress,
    users,
);
