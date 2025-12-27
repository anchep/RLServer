-- Add token column to verification_codes table
ALTER TABLE verification_codes ADD COLUMN IF NOT EXISTS token VARCHAR(255) DEFAULT '' NOT NULL;

-- Create index for token column
CREATE INDEX IF NOT EXISTS idx_verification_codes_token ON verification_codes(token);