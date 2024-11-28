-- Your SQL goes here
DO $$ 
BEGIN 
    -- Delete any progress records that don't have a repository
    DELETE FROM progress 
    WHERE repository_id IS NULL;

    -- For each repository without progress, create a progress record
    INSERT INTO progress (
        id, 
        user_id, 
        challenge_id, 
        repository_id, 
        status, 
        progress_details,
        created_at,
        updated_at
    )
    SELECT 
        gen_random_uuid(),
        r.user_id,
        r.challenge_id,
        r.id,
        'not_started',
        '{"current_step": 1}'::jsonb,
        now(),
        now()
    FROM repositories r
    WHERE NOT EXISTS (
        SELECT 1 FROM progress p 
        WHERE p.repository_id = r.id
    );

    -- Make repository_id NOT NULL
    ALTER TABLE progress ALTER COLUMN repository_id SET NOT NULL;
END $$;