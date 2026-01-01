use chrono::{DateTime, Duration, Utc};
use ipnet::Ipv6Net;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

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

    /// Get the ban key for the IP address
    /// For IPv4, use the original IP address
    /// For IPv6, use the /48 subnet
    fn get_ban_key(ip: &IpAddr) -> IpAddr {
        match ip {
            IpAddr::V4(ipv4) => IpAddr::V4(*ipv4),
            IpAddr::V6(ipv6) => {
                // Convert the IPv6 address into a /48 network, then obtain the network address.
                match Ipv6Net::new(*ipv6, 48) {
                    Ok(net) => IpAddr::V6(net.network()),
                    Err(_) => {
                        // quick fallback to the original IP if network creation fails
                        tracing::warn!("Failed to create /48 network for IPv6 address: {}", ipv6);
                        IpAddr::V6(*ipv6)
                    }
                }
            }
        }
    }
    /// Check if the IP is blocked
    pub async fn is_banned(&self, ip: &IpAddr) -> bool {
        let ban_key = Self::get_ban_key(ip);
        let mut records = self.records.write().await;

        if let Some(record) = records.get(&ban_key) {
            if let Some(banned_until) = record.banned_until {
                let now = Utc::now();

                // If the ban has expired, clear the ban status
                if now >= banned_until {
                    if let Some(rec) = records.get_mut(&ban_key) {
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
        let ban_key = Self::get_ban_key(ip);
        let mut records = self.records.write().await;

        let record = records.entry(ban_key).or_insert(FailureRecord {
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

            // If you fail 5 times, you will be banned for 1 day.
            if record.count >= 5 {
                record.banned_until = Some(now + Duration::days(1));
                tracing::warn!(
                    "IP {} has been banned for 1 day due to {} failed login attempts",
                    ip,
                    record.count
                );
            }
        }
    }

    /// Clear failure records
    pub async fn record_success(&self, ip: &IpAddr) {
        let ban_key = Self::get_ban_key(ip);
        let mut records = self.records.write().await;
        records.remove(&ban_key);
    }

    /// Get the remaining ban time of the IP
    pub async fn get_ban_remaining_seconds(&self, ip: &IpAddr) -> Option<i64> {
        let ban_key = Self::get_ban_key(ip);
        let records = self.records.read().await;

        if let Some(record) = records.get(&ban_key) {
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
