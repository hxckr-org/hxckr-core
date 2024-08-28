-- Your SQL goes here
ALTER TABLE exercises
ADD COLUMN status VARCHAR(255) NOT NULL CHECK (status IN ('completed', 'in_progress', 'not_started')) DEFAULT 'not_started';
