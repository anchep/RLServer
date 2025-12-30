-- 删除后台会话表
DROP TABLE IF EXISTS admin_sessions;

-- 删除后台操作日志表
DROP TABLE IF EXISTS admin_logs;

-- 删除后台管理员表
DROP TABLE IF EXISTS admin_users;

-- 从users表中删除note和status字段
ALTER TABLE users DROP COLUMN IF EXISTS note;
ALTER TABLE users DROP COLUMN IF EXISTS status;

-- 从software表中删除status字段
ALTER TABLE software DROP COLUMN IF EXISTS status;

-- 从recharge_cards表中删除price字段
ALTER TABLE recharge_cards DROP COLUMN IF EXISTS price;
