-- Your SQL goes here
ALTER TABLE challenges ADD COLUMN IF NOT EXISTS module_count INT NOT NULL DEFAULT 0;