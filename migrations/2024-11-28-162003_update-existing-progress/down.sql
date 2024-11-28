-- This file should undo anything in `up.sql`
DO $$ 
BEGIN 
    -- Make repository_id nullable again
    ALTER TABLE progress ALTER COLUMN repository_id DROP NOT NULL;
END $$;