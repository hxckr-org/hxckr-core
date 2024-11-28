-- Your SQL goes here
ALTER TABLE challenges ADD COLUMN IF NOT EXISTS repo_urls JSONB NOT NULL DEFAULT '{"rust": ""}';