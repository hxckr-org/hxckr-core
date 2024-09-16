-- This file should undo anything in `up.sql`
ALTER TABLE repositories DROP COLUMN IF EXISTS soft_serve_url;