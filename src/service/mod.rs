pub mod database {
    pub mod conn;
    pub mod models;
}

pub mod repository {
    pub mod users;
    pub mod challenge;
    pub mod exercise;
    pub mod progress;
    pub mod repo;
    pub mod submission;
    pub mod session;
    pub mod leaderboard;
    pub mod badge;
}

pub mod queue;
