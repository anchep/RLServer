use chrono::{DateTime, TimeZone, Utc, FixedOffset};

// 创建上海时区(UTC+8)
pub const SHANGHAI_TZ: FixedOffset = FixedOffset::east_opt(8 * 3600).unwrap();

// 获取当前上海时间
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

// 将UTC时间转换为上海时间
pub fn to_shanghai_time(utc_time: DateTime<Utc>) -> DateTime<FixedOffset> {
    utc_time.with_timezone(&SHANGHAI_TZ)
}

// 将时间戳转换为上海时间
pub fn timestamp_to_shanghai_time(timestamp: i64) -> DateTime<FixedOffset> {
    let utc_time = DateTime::from_timestamp(timestamp, 0).unwrap_or(Utc::now());
    utc_time.with_timezone(&SHANGHAI_TZ)
}
