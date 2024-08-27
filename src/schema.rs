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

diesel::allow_tables_to_appear_in_same_query!(
    challenges,
    users,
);
