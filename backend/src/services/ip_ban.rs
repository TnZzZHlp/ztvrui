use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

/// IP失败记录
#[derive(Debug, Clone)]
struct FailureRecord {
    /// 失败次数
    count: u32,
    /// 记录开始时间（用于判断是否超过1小时）
    first_failure: DateTime<Utc>,
    /// 封禁结束时间（如果被封禁）
    banned_until: Option<DateTime<Utc>>,
}

/// IP封禁服务 - 简化版
#[derive(Clone)]
pub struct IpBanService {
    /// 失败记录：IP地址 -> 失败记录
    records: Arc<RwLock<HashMap<IpAddr, FailureRecord>>>,
}

impl Default for IpBanService {
    fn default() -> Self {
        Self::new()
    }
}

impl IpBanService {
    pub fn new() -> Self {
        Self {
            records: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Check if the IP is blocked
    pub async fn is_banned(&self, ip: &IpAddr) -> bool {
        let mut records = self.records.write().await;

        if let Some(record) = records.get(ip) {
            if let Some(banned_until) = record.banned_until {
                let now = Utc::now();

                // If the ban has expired, clear the ban status
                if now >= banned_until {
                    if let Some(rec) = records.get_mut(ip) {
                        rec.banned_until = None;
                        rec.count = 0;
                    }
                    false
                } else {
                    true
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Record of failed attempts
    pub async fn record_failure(&self, ip: &IpAddr) {
        let now = Utc::now();
        let mut records = self.records.write().await;

        let record = records.entry(*ip).or_insert(FailureRecord {
            count: 0,
            first_failure: now,
            banned_until: None,
        });

        // Check if it has been more than an hour, and if so, reset the count.
        if now - record.first_failure > Duration::hours(1) {
            record.count = 1;
            record.first_failure = now;
            record.banned_until = None;
        } else {
            record.count += 1;

            // If you fail 5 times, you will be banned for 1 hour.
            if record.count >= 5 {
                record.banned_until = Some(now + Duration::hours(1));
                tracing::warn!(
                    "IP {} has been banned for 1 hour due to {} failed login attempts",
                    ip,
                    record.count
                );
            }
        }
    }

    /// 清除失败记录
    pub async fn record_success(&self, ip: &IpAddr) {
        let mut records = self.records.write().await;
        records.remove(ip);
    }

    /// 获取IP剩余的封禁时间
    pub async fn get_ban_remaining_seconds(&self, ip: &IpAddr) -> Option<i64> {
        let records = self.records.read().await;

        if let Some(record) = records.get(ip) {
            if let Some(banned_until) = record.banned_until {
                let now = Utc::now();
                let remaining = banned_until - now;

                if remaining.num_seconds() > 0 {
                    return Some(remaining.num_seconds());
                }
            }
        }

        None
    }
}
