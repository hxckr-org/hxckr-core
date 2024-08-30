-- Your SQL goes here
CREATE TABLE exercises (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    difficulty VARCHAR(255) CHECK (difficulty IN ('easy', 'medium', 'hard')) NOT NULL,
    test_runner VARCHAR(255) NOT NULL,
    challenge_id UUID NOT NULL REFERENCES challenges(id),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX idx_exercises_challenge_id ON exercises(challenge_id);
