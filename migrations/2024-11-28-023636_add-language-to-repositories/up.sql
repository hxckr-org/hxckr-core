-- Your SQL goes here
ALTER TABLE repositories ADD COLUMN IF NOT EXISTS language VARCHAR(255) NOT NULL DEFAULT 'rust';