-- Your SQL goes here
ALTER TABLE repositories ADD COLUMN IF NOT EXISTS soft_serve_url TEXT NOT NULL DEFAULT '';