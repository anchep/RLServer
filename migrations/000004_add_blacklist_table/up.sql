-- 创建黑名单表
CREATE TABLE blacklist (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255),
    hardware_code VARCHAR(255),
    ip_address VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 创建索引，提高查询效率
CREATE INDEX idx_blacklist_username ON blacklist(username);
CREATE INDEX idx_blacklist_hardware_code ON blacklist(hardware_code);
CREATE INDEX idx_blacklist_ip_address ON blacklist(ip_address);
