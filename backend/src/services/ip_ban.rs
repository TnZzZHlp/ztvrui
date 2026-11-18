use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

const FAILURE_WINDOW_HOURS: i64 = 1;
const MAX_FAILURES: u32 = 5;
const BAN_DURATION_DAYS: i64 = 1;
const PRUNE_THRESHOLD: usize = 1024;
const MAX_RECORDS: usize = 10_000;

#[derive(Debug, Clone)]
struct FailureRecord {
    count: u32,
    first_failure: DateTime<Utc>,
    banned_until: Option<DateTime<Utc>>,
}

#[derive(Clone)]
pub struct IpBanService {
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

    /// Get the ban key for the IP address.
    /// For IPv4 (and IPv4-mapped IPv6), use the canonical IPv4 address.
    /// For native IPv6, use the /48 subnet.
    fn get_ban_key(ip: &IpAddr) -> IpAddr {
        match ip.to_canonical() {
            IpAddr::V4(ipv4) => IpAddr::V4(ipv4),
            IpAddr::V6(ipv6) => IpAddr::V6(std::net::Ipv6Addr::from(
                u128::from(ipv6) & (u128::MAX << 80),
            )),
        }
    }

    /// Check if the IP is currently blocked.
    pub async fn is_banned(&self, ip: &IpAddr) -> bool {
        let ban_key = Self::get_ban_key(ip);
        let now = Utc::now();

        {
            let records = self.records.read().await;
            match records.get(&ban_key).and_then(|r| r.banned_until) {
                None => return false,
                Some(banned_until) if now < banned_until => return true,
                Some(_) => {}
            }
        }

        // Only acquire write lock when an expired ban needs cleanup
        let mut records = self.records.write().await;
        if let Some(record) = records.get(&ban_key) {
            if let Some(banned_until) = record.banned_until {
                if Utc::now() < banned_until {
                    return true;
                }
            }
            records.remove(&ban_key);
        }
        false
    }

    /// Record a failed authentication/verification attempt.
    pub async fn record_failure(&self, ip: &IpAddr) {
        let now = Utc::now();
        let ban_key = Self::get_ban_key(ip);
        let mut records = self.records.write().await;

        if !records.contains_key(&ban_key) && records.len() >= PRUNE_THRESHOLD {
            let window = Duration::hours(FAILURE_WINDOW_HOURS);
            records.retain(|_, rec| {
                rec.banned_until.is_some_and(|until| until > now)
                    || (rec.banned_until.is_none() && now - rec.first_failure <= window)
            });

            if records.len() >= MAX_RECORDS {
                let evict_key = records
                    .iter()
                    .filter(|(_, rec)| rec.banned_until.is_none())
                    .min_by_key(|(_, rec)| rec.first_failure)
                    .map(|(key, _)| *key)
                    .or_else(|| {
                        records
                            .iter()
                            .min_by_key(|(_, rec)| rec.banned_until.unwrap_or(rec.first_failure))
                            .map(|(key, _)| *key)
                    });
                if let Some(key) = evict_key {
                    records.remove(&key);
                }
            }
        }

        let record = records.entry(ban_key).or_insert(FailureRecord {
            count: 0,
            first_failure: now,
            banned_until: None,
        });

        // Preserve active ban if already banned
        if record.banned_until.is_some_and(|until| until > now) {
            return;
        }

        // Reset if ban expired or failure window elapsed
        if record.banned_until.is_some()
            || now - record.first_failure > Duration::hours(FAILURE_WINDOW_HOURS)
        {
            record.count = 1;
            record.first_failure = now;
            record.banned_until = None;
        } else {
            record.count += 1;

            if record.count >= MAX_FAILURES {
                record.banned_until = Some(now + Duration::days(BAN_DURATION_DAYS));
                tracing::warn!(
                    "IP {} has been banned for 1 day due to {} failed attempts",
                    ip.to_canonical(),
                    record.count
                );
            }
        }
    }

    /// Clear failure records after a successful attempt.
    pub async fn record_success(&self, ip: &IpAddr) {
        let ban_key = Self::get_ban_key(ip);
        let mut records = self.records.write().await;
        records.remove(&ban_key);
    }

    /// Get the remaining ban duration in seconds for the IP, if banned.
    pub async fn get_ban_remaining_seconds(&self, ip: &IpAddr) -> Option<i64> {
        let ban_key = Self::get_ban_key(ip);
        let records = self.records.read().await;

        if let Some(record) = records.get(&ban_key) {
            if let Some(banned_until) = record.banned_until {
                let remaining = banned_until - Utc::now();
                if remaining.num_seconds() > 0 {
                    return Some(remaining.num_seconds());
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn bans_after_five_failures_and_clears_on_success() {
        let service = IpBanService::new();
        let ip = IpAddr::from_str("203.0.114.9").unwrap();
        let mapped = IpAddr::from_str("::ffff:203.0.114.9").unwrap();

        for _ in 0..4 {
            service.record_failure(&ip).await;
            assert!(!service.is_banned(&ip).await);
        }

        service.record_failure(&ip).await;
        assert!(service.is_banned(&ip).await);
        assert!(service.is_banned(&mapped).await);
        assert!(service.get_ban_remaining_seconds(&ip).await.unwrap_or(0) > 0);

        service.record_success(&mapped).await;
        assert!(!service.is_banned(&ip).await);
    }

    #[tokio::test]
    async fn ipv6_addresses_share_48_subnet_ban() {
        let service = IpBanService::new();
        let ip1 = IpAddr::from_str("2001:db8:abcd:0001::1").unwrap();
        let ip2 = IpAddr::from_str("2001:db8:abcd:9999::2").unwrap();
        let other = IpAddr::from_str("2001:db8:abce::1").unwrap();

        for _ in 0..5 {
            service.record_failure(&ip1).await;
        }

        assert!(service.is_banned(&ip2).await);
        assert!(!service.is_banned(&other).await);
    }
}
