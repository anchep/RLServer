use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// 验证码表
#[derive(Queryable, Identifiable, Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::verification_codes)]
#[diesel(treat_none_as_null = true)]
pub struct VerificationCode {
    pub id: i32,
    pub user_id: i32,
    pub email: String,
    pub code: String,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub used: bool,
    pub created_at: DateTime<Utc>,
}