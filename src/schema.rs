// 导入Diesel必要的类型和宏
use diesel::prelude::*;
use diesel::sql_types::*;

// 定义表结构
table! {
    login_logs (id) {
        id -> Int4,
        user_id -> Int4,
        login_time -> Timestamptz,
        hardware_code -> Varchar,
        software_version -> Varchar,
        ip_address -> Varchar,
        status -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    online_users (id) {
        id -> Int4,
        user_id -> Int4,
        session_token -> Varchar,
        login_time -> Timestamptz,
        hardware_code -> Varchar,
        software_version -> Varchar,
        ip_address -> Varchar,
        last_activity_at -> Timestamptz,
        status_interval -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    recharge_cards (id) {
        id -> Int4,
        card_code -> Varchar,
        amount -> Int4,
        vip_level -> Int4,
        duration_days -> Int4,
        is_used -> Bool,
        used_at -> Nullable<Timestamptz>,
        used_by -> Nullable<Int4>,
        created_at -> Timestamptz,
    }
}

table! {
    recharge_logs (id) {
        id -> Int4,
        user_id -> Int4,
        card_code -> Varchar,
        vip_level -> Int4,
        duration_days -> Int4,
        recharge_time -> Timestamptz,
        created_at -> Timestamptz,
    }
}

table! {
    software (id) {
        id -> Int4,
        name -> Varchar,
        required_vip_level -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        email -> Varchar,
        email_verified -> Bool,
        vip_level -> Int4,
        vip_expires_at -> Nullable<Timestamptz>,
        last_login_at -> Nullable<Timestamptz>,
        last_login_hardware -> Nullable<Varchar>,
        last_login_version -> Nullable<Varchar>,
        last_login_ip -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    verification_codes (id) {
        id -> Int4,
        user_id -> Int4,
        email -> Varchar,
        code -> Varchar,
        expires_at -> Timestamptz,
        used -> Bool,
        created_at -> Timestamptz,
    }
}

// 导出表，以便在其他文件中使用
allow_tables_to_appear_in_same_query!(login_logs, online_users, recharge_cards, recharge_logs, software, users, verification_codes,);
