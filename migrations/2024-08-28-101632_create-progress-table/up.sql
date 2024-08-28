-- Your SQL goes here
CREATE TABLE progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    challenge_id UUID NOT NULL REFERENCES challenges(id),
    status VARCHAR NOT NULL CHECK (status IN ('completed', 'in_progress', 'not_started')) DEFAULT 'not_started',
    progress_details JSONB,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE INDEX idx_progress_user_id ON progress(user_id);
CREATE INDEX idx_progress_challenge_id ON progress(challenge_id);
