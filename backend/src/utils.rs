use axum::http::HeaderMap;
use std::net::{IpAddr, SocketAddr};

/// Check if an IP address is a private/internal IP
/// Returns true if the IP is not a public routable address
pub fn extract_real_ip(headers: &HeaderMap, fallback_addr: SocketAddr) -> IpAddr {
    let connect_ip = fallback_addr.ip();

    if !is_private_ip(&connect_ip) {
        tracing::debug!("Using public IP from ConnectInfo: {}", connect_ip);
        return connect_ip;
    }

    if let Some(real_ip) = headers.get("X-Real-IP") {
        if let Ok(ip_str) = real_ip.to_str() {
            if let Ok(ip) = ip_str.parse::<IpAddr>() {
                tracing::debug!("Got IP from X-Real-IP header: {}", ip);
                return ip;
            }
        }
    }

    if let Some(forwarded_for) = headers.get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                if let Ok(ip) = first_ip.trim().parse::<IpAddr>() {
                    tracing::debug!("Got IP from X-Forwarded-For header: {}", ip);
                    return ip;
                }
            }
        }
    }

    tracing::debug!("Using fallback private IP from ConnectInfo: {}", connect_ip);
    connect_ip
}

pub fn is_private_ip(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => {
            // Use standard library methods for IPv4
            ipv4.is_private()        // 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
                || ipv4.is_loopback()    // 127.0.0.0/8
                || ipv4.is_link_local()  // 169.254.0.0/16
                || ipv4.is_broadcast()   // 255.255.255.255
                || ipv4.is_unspecified() // 0.0.0.0
                // Check for documentation addresses: 192.0.2.0/24, 198.51.100.0/24, 203.0.113.0/24
                || (ipv4.octets()[0] == 192 && ipv4.octets()[1] == 0 && ipv4.octets()[2] == 2)
                || (ipv4.octets()[0] == 198 && ipv4.octets()[1] == 51 && ipv4.octets()[2] == 100)
                || (ipv4.octets()[0] == 203 && ipv4.octets()[1] == 0 && ipv4.octets()[2] == 113)
        }
        IpAddr::V6(ipv6) => {
            // Use standard library methods for IPv6
            ipv6.is_unspecified()        // ::
                || ipv6.is_loopback()        // ::1
                || ipv6.is_unique_local()    // fc00::/7
                || ipv6.is_unicast_link_local() // fe80::/10
                || ipv6.is_multicast()       // ff00::/8
                // Check for documentation addresses: 2001:db8::/32
                || (ipv6.segments()[0] == 0x2001 && ipv6.segments()[1] == 0x0db8)
                // Check for IPv4-mapped IPv6 addresses (::ffff:0:0/96)
                || matches!(ipv6.segments(), [0, 0, 0, 0, 0, 0xffff, _, _])
        }
    }
}
