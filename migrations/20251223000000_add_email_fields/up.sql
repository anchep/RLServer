-- 添加邮箱相关字段到users表
ALTER TABLE users ADD COLUMN email VARCHAR(255) NOT NULL DEFAULT '';
ALTER TABLE users ADD COLUMN email_verified BOOLEAN NOT NULL DEFAULT false;

-- 添加邮箱唯一约束
ALTER TABLE users ADD CONSTRAINT unique_email UNIQUE (email);
