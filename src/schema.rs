// @generated automatically by Diesel CLI.

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
