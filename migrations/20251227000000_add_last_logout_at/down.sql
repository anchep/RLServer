-- Remove last_logout_at column from users table
ALTER TABLE users
DROP COLUMN last_logout_at;
