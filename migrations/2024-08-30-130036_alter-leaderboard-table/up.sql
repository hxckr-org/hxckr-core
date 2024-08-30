-- Your SQL goes here
ALTER TABLE leaderboard ALTER COLUMN id SET DEFAULT nextval('leaderboard_id_seq');
ALTER TABLE leaderboard ALTER COLUMN id SET NOT NULL;
