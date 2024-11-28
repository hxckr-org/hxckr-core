DO $$ 
BEGIN 
    -- Add the column if it doesn't exist
    IF NOT EXISTS (SELECT 1 
                   FROM information_schema.columns 
                   WHERE table_name='progress' AND column_name='repository_id') THEN
        ALTER TABLE progress ADD COLUMN repository_id UUID REFERENCES repositories(id);

        -- Update existing progress records with their repository IDs
        -- Keep only the latest progress record for each repository
        WITH latest_progress AS (
            SELECT DISTINCT ON (r.id) 
                r.id as repo_id,
                p.id as progress_id
            FROM repositories r
            JOIN progress p ON p.user_id = r.user_id AND p.challenge_id = r.challenge_id
            ORDER BY r.id, p.created_at DESC
        )
        UPDATE progress p
        SET repository_id = lp.repo_id
        FROM latest_progress lp
        WHERE p.id = lp.progress_id;

        -- Delete duplicate progress records
        DELETE FROM progress 
        WHERE repository_id IS NULL;
    END IF;

    -- Add unique constraint if it doesn't exist
    IF NOT EXISTS (SELECT 1 
                   FROM information_schema.table_constraints 
                   WHERE constraint_name='unique_repository_progress') THEN
        ALTER TABLE progress ADD CONSTRAINT unique_repository_progress UNIQUE (repository_id);
    END IF;
END $$;