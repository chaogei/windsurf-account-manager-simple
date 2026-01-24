#![allow(dead_code)]
use chrono::{DateTime, Utc, Duration};

/// 格式化日期时间为用户友好的字符串
pub fn format_datetime(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 检查token是否过期
pub fn is_token_expired(expires_at: &DateTime<Utc>) -> bool {
    Utc::now() >= *expires_at
}

/// 检查token是否即将过期（5分钟内）
pub fn is_token_expiring_soon(expires_at: &DateTime<Utc>) -> bool {
    Utc::now() + Duration::minutes(5) >= *expires_at
}

/// 计算从现在到指定时间的持续时间（秒）
pub fn seconds_until(target: &DateTime<Utc>) -> i64 {
    (*target - Utc::now()).num_seconds()
}

/// 解析ISO 8601格式的时间字符串
pub fn parse_iso_datetime(s: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    s.parse::<DateTime<Utc>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_datetime() {
        let dt = "2024-01-01T12:00:00Z".parse::<DateTime<Utc>>().unwrap();
        assert_eq!(format_datetime(&dt), "2024-01-01 12:00:00");
    }

    #[test]
    fn test_is_token_expired() {
        let past = Utc::now() - Duration::hours(1);
        let future = Utc::now() + Duration::hours(1);
        
        assert!(is_token_expired(&past));
        assert!(!is_token_expired(&future));
    }
}
