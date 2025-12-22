-- Drop tables in reverse order of creation to avoid foreign key constraints
DROP TABLE IF EXISTS online_users;
DROP TABLE IF EXISTS login_logs;
DROP TABLE IF EXISTS recharge_logs;
DROP TABLE IF EXISTS recharge_cards;
DROP TABLE IF EXISTS software;
DROP TABLE IF EXISTS users;