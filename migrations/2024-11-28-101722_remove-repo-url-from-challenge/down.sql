-- This file should undo anything in `up.sql`
ALTER TABLE challenges ADD COLUMN IF NOT EXISTS repo_url VARCHAR(255) NOT NULL DEFAULT '';