-- Migration: create_users
-- Created at: 2025-12-22 22:00:00

-- +goose Up
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    vip_level INTEGER NOT NULL DEFAULT 0,
    vip_expires_at TIMESTAMPTZ NULL,
    last_login_at TIMESTAMPTZ NULL,
    last_login_hardware VARCHAR(255) NULL,
    last_login_version VARCHAR(255) NULL,
    last_login_ip VARCHAR(50) NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS software (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    required_vip_level INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS recharge_cards (
    id SERIAL PRIMARY KEY,
    card_code VARCHAR(255) UNIQUE NOT NULL,
    amount INTEGER NOT NULL,
    vip_level INTEGER NOT NULL,
    duration_days INTEGER NOT NULL,
    is_used BOOLEAN NOT NULL DEFAULT FALSE,
    used_at TIMESTAMPTZ NULL,
    used_by INTEGER NULL REFERENCES users(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS recharge_logs (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    card_code VARCHAR(255) NOT NULL,
    vip_level INTEGER NOT NULL,
    duration_days INTEGER NOT NULL,
    recharge_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS login_logs (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    login_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    hardware_code VARCHAR(255) NOT NULL,
    software_version VARCHAR(255) NOT NULL,
    ip_address VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS online_users (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    session_token VARCHAR(255) NOT NULL,
    login_time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    hardware_code VARCHAR(255) NOT NULL,
    software_version VARCHAR(255) NOT NULL,
    ip_address VARCHAR(50) NOT NULL,
    last_activity_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status_interval INTEGER NOT NULL DEFAULT 10,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- +goose Down
DROP TABLE IF EXISTS online_users;
DROP TABLE IF EXISTS login_logs;
DROP TABLE IF EXISTS recharge_logs;
DROP TABLE IF EXISTS recharge_cards;
DROP TABLE IF EXISTS software;
DROP TABLE IF EXISTS users;
