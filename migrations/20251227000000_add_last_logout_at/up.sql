-- Add last_logout_at column to users table
ALTER TABLE users
ADD COLUMN last_logout_at TIMESTAMP WITH TIME ZONE;
