-- This file should undo anything in `up.sql`
DO $$ 
BEGIN 
    -- Drop constraint if it exists
    IF EXISTS (SELECT 1 
               FROM information_schema.table_constraints 
               WHERE constraint_name='unique_repository_progress' 
               AND table_name='progress') THEN
        ALTER TABLE progress DROP CONSTRAINT unique_repository_progress;
    END IF;

    -- Drop column if it exists
    IF EXISTS (SELECT 1 
               FROM information_schema.columns 
               WHERE table_name='progress' AND column_name='repository_id') THEN
        ALTER TABLE progress DROP COLUMN repository_id CASCADE;
    END IF;
END $$;