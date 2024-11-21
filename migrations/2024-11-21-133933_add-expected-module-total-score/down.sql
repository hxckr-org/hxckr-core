-- This file should undo anything in `up.sql`
ALTER TABLE leaderboard DROP COLUMN IF EXISTS expected_total_score;