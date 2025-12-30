use diesel::prelude::*;
use r2d2::Pool;
use diesel::r2d2::ConnectionManager;
use crate::schema::software;
use crate::database::models::Software;
use crate::errors::ServiceError;
use chrono::{Utc, DateTime};

// 获取所有软件列表
pub fn get_all_software(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<Vec<Software>, ServiceError> {
    let mut conn = pool.get()?;
    
    let software_list = software::table
        .order_by(software::id.desc())
        .load::<Software>(&mut conn)?;
    
    Ok(software_list)
}

// 根据ID获取软件详情
pub fn get_software_by_id(
    pool: &Pool<ConnectionManager<PgConnection>>,
    software_id: i32,
) -> Result<Software, ServiceError> {
    let mut conn = pool.get()?;
    
    let software = software::table
        .filter(software::id.eq(software_id))
        .first::<Software>(&mut conn)?;
    
    Ok(software)
}

// 创建软件
pub fn create_software(
    pool: &Pool<ConnectionManager<PgConnection>>,
    name: &str,
    chinese_name: &str,
    description: &str,
    detailed_description: &str,
    executable_name: &str,
    md5_checksum: &str,
    requires_admin: bool,
    required_vip_level: i32,
    status: bool,
) -> Result<Software, ServiceError> {
    let mut conn = pool.get()?;
    
    // 检查软件名称是否已存在
    let existing_software = software::table
        .filter(software::name.eq(name))
        .first::<Software>(&mut conn)
        .optional()?;
    
    if existing_software.is_some() {
        return Err(ServiceError::BadRequest("软件名称已存在".to_string()));
    }
    
    // 检查可执行文件名称是否已存在
    let existing_executable = software::table
        .filter(software::executable_name.eq(executable_name))
        .first::<Software>(&mut conn)
        .optional()?;
    
    if existing_executable.is_some() {
        return Err(ServiceError::BadRequest("可执行文件名称已存在".to_string()));
    }
    
    // 创建软件
    let new_software = diesel::insert_into(software::table)
        .values((
            software::name.eq(name),
            software::chinese_name.eq(chinese_name),
            software::description.eq(description),
            software::detailed_description.eq(detailed_description),
            software::executable_name.eq(executable_name),
            software::md5_checksum.eq(md5_checksum),
            software::requires_admin.eq(requires_admin),
            software::required_vip_level.eq(required_vip_level),
            software::status.eq(status),
            software::created_at.eq(Utc::now()),
            software::updated_at.eq(Utc::now()),
        ))
        .get_result::<Software>(&mut conn)?;
    
    Ok(new_software)
}

// 更新软件
pub fn update_software(
    pool: &Pool<ConnectionManager<PgConnection>>,
    software_id: i32,
    name: &str,
    chinese_name: &str,
    description: &str,
    detailed_description: &str,
    executable_name: &str,
    md5_checksum: &str,
    requires_admin: bool,
    required_vip_level: i32,
) -> Result<Software, ServiceError> {
    let mut conn = pool.get()?;
    
    // 检查软件是否存在
    let existing_software = software::table
        .filter(software::id.eq(software_id))
        .first::<Software>(&mut conn)
        .optional()?;
    
    if existing_software.is_none() {
        return Err(ServiceError::NotFound("软件不存在".to_string()));
    }
    
    // 检查软件名称是否已被其他软件使用
    let duplicate_name = software::table
        .filter(software::name.eq(name))
        .filter(software::id.ne(software_id))
        .first::<Software>(&mut conn)
        .optional()?;
    
    if duplicate_name.is_some() {
        return Err(ServiceError::BadRequest("软件名称已被其他软件使用".to_string()));
    }
    
    // 检查可执行文件名称是否已被其他软件使用
    let duplicate_executable = software::table
        .filter(software::executable_name.eq(executable_name))
        .filter(software::id.ne(software_id))
        .first::<Software>(&mut conn)
        .optional()?;
    
    if duplicate_executable.is_some() {
        return Err(ServiceError::BadRequest("可执行文件名称已被其他软件使用".to_string()));
    }
    
    // 更新软件
    let updated_software = diesel::update(software::table.filter(software::id.eq(software_id)))
        .set((
            software::name.eq(name),
            software::chinese_name.eq(chinese_name),
            software::description.eq(description),
            software::detailed_description.eq(detailed_description),
            software::executable_name.eq(executable_name),
            software::md5_checksum.eq(md5_checksum),
            software::requires_admin.eq(requires_admin),
            software::required_vip_level.eq(required_vip_level),
            software::updated_at.eq(Utc::now()),
        ))
        .get_result::<Software>(&mut conn)?;
    
    Ok(updated_software)
}

// 删除软件
pub fn delete_software(
    pool: &Pool<ConnectionManager<PgConnection>>,
    software_id: i32,
) -> Result<(), ServiceError> {
    let mut conn = pool.get()?;
    
    // 检查软件是否存在
    let existing_software = software::table
        .filter(software::id.eq(software_id))
        .first::<Software>(&mut conn)
        .optional()?;
    
    if existing_software.is_none() {
        return Err(ServiceError::NotFound("软件不存在".to_string()));
    }
    
    // 删除软件
    diesel::delete(software::table.filter(software::id.eq(software_id)))
        .execute(&mut conn)?;
    
    Ok(())
}

// 切换软件状态
pub fn toggle_software_status(
    pool: &Pool<ConnectionManager<PgConnection>>,
    software_id: i32,
) -> Result<Software, ServiceError> {
    let mut conn = pool.get()?;
    
    // 检查软件是否存在
    let existing_software = software::table
        .filter(software::id.eq(software_id))
        .first::<Software>(&mut conn)?;
    
    // 切换状态
    let new_status = !existing_software.status;
    
    // 更新软件状态
    let updated_software = diesel::update(software::table.filter(software::id.eq(software_id)))
        .set((
            software::status.eq(new_status),
            software::updated_at.eq(Utc::now()),
        ))
        .get_result::<Software>(&mut conn)?;
    
    Ok(updated_software)
}



// 获取软件数量统计
pub fn get_software_stats(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<(i64, i64), ServiceError> {
    let mut conn = pool.get()?;
    
    // 总软件数
    let total = software::table
        .count()
        .first::<i64>(&mut conn)?;
    
    // 活跃软件数
    let active = software::table
        .filter(software::status.eq(true))
        .count()
        .first::<i64>(&mut conn)?;
    
    Ok((total, active))
}
