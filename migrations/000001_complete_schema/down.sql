-- Drop indexes first
DROP INDEX IF EXISTS idx_verification_codes_user_id;
DROP INDEX IF EXISTS idx_verification_codes_email;
DROP INDEX IF EXISTS idx_online_users_last_activity_at;
DROP INDEX IF EXISTS idx_online_users_session_token;
DROP INDEX IF EXISTS idx_online_users_user_id;
DROP INDEX IF EXISTS idx_recharge_cards_is_used;
DROP INDEX IF EXISTS idx_recharge_cards_card_code;
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_users_vip_level;
DROP INDEX IF EXISTS idx_users_username;

-- Drop tables in reverse order of creation (to handle foreign key dependencies)
DROP TABLE IF EXISTS verification_codes;
DROP TABLE IF EXISTS online_users;
DROP TABLE IF EXISTS login_logs;
DROP TABLE IF EXISTS recharge_logs;
DROP TABLE IF EXISTS recharge_cards;
DROP TABLE IF EXISTS software;
DROP TABLE IF EXISTS users;
