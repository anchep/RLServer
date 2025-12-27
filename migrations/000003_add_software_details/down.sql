-- 从software表中删除软件详情字段
ALTER TABLE software
DROP COLUMN IF EXISTS chinese_name,
DROP COLUMN IF EXISTS description,
DROP COLUMN IF EXISTS detailed_description,
DROP COLUMN IF EXISTS executable_name,
DROP COLUMN IF EXISTS md5_checksum,
DROP COLUMN IF EXISTS requires_admin;
