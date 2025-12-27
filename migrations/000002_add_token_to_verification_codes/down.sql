-- Remove token column from verification_codes table
ALTER TABLE verification_codes DROP COLUMN IF EXISTS token;

-- Drop index for token column
DROP INDEX IF EXISTS idx_verification_codes_token;