-- Your SQL goes here
ALTER TABLE leaderboard ADD COLUMN IF NOT EXISTS expected_total_score INTEGER NOT NULL DEFAULT 0;
