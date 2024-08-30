-- Your SQL goes here
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger
        WHERE tgname = 'set_updated_at'
        AND tgrelid = 'leaderboard'::regclass
    ) THEN
        PERFORM diesel_manage_updated_at('leaderboard');
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger
        WHERE tgname = 'set_updated_at'
        AND tgrelid = 'users'::regclass
    ) THEN
        PERFORM diesel_manage_updated_at('users');
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger
        WHERE tgname = 'set_updated_at'
        AND tgrelid = 'challenges'::regclass
    ) THEN
        PERFORM diesel_manage_updated_at('challenges');
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger
        WHERE tgname = 'set_updated_at'
        AND tgrelid = 'exercises'::regclass
    ) THEN
        PERFORM diesel_manage_updated_at('exercises');
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger
        WHERE tgname = 'set_updated_at'
        AND tgrelid = 'progress'::regclass
    ) THEN
        PERFORM diesel_manage_updated_at('progress');
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger
        WHERE tgname = 'set_updated_at'
        AND tgrelid = 'repositories'::regclass
    ) THEN
        PERFORM diesel_manage_updated_at('repositories');
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger
        WHERE tgname = 'set_updated_at'
        AND tgrelid = 'submissions'::regclass
    ) THEN
        PERFORM diesel_manage_updated_at('submissions');
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger
        WHERE tgname = 'set_updated_at'
        AND tgrelid = 'badges'::regclass
    ) THEN
        PERFORM diesel_manage_updated_at('badges');
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger
        WHERE tgname = 'set_updated_at'
        AND tgrelid = 'user_badges'::regclass
    ) THEN
        PERFORM diesel_manage_updated_at('user_badges');
    END IF;
END $$;
