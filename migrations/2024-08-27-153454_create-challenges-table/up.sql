-- Your SQL goes here
CREATE TABLE challenges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    repo_url VARCHAR(255) NOT NULL,
    difficulty VARCHAR(255) CHECK (difficulty IN ('easy', 'medium', 'hard')) NOT NULL,
    mode VARCHAR(255) CHECK (mode IN ('project', 'functional_test')) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);