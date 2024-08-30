-- This file should undo anything in `up.sql`
DROP TRIGGER IF EXISTS diesel_manage_updated_at ON users;
DROP TRIGGER IF EXISTS diesel_manage_updated_at ON challenges;
DROP TRIGGER IF EXISTS diesel_manage_updated_at ON exercises;
DROP TRIGGER IF EXISTS diesel_manage_updated_at ON progress;
DROP TRIGGER IF EXISTS diesel_manage_updated_at ON repositories;
DROP TRIGGER IF EXISTS diesel_manage_updated_at ON submissions;
DROP TRIGGER IF EXISTS diesel_manage_updated_at ON leaderboard;
DROP TRIGGER IF EXISTS diesel_manage_updated_at ON badges;
DROP TRIGGER IF EXISTS diesel_manage_updated_at ON user_badges;