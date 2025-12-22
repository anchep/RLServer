-- 删除邮箱唯一约束
ALTER TABLE users DROP CONSTRAINT IF EXISTS unique_email;

-- 删除邮箱相关字段
ALTER TABLE users DROP COLUMN IF EXISTS email_verified;
ALTER TABLE users DROP COLUMN IF EXISTS email;
