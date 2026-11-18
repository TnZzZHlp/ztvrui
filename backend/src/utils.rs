use axum::http::HeaderMap;
use std::net::{IpAddr, SocketAddr};

/// Constant-time byte slice comparison to prevent timing side-channel leaks.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let max_len = a.len().max(b.len());
    let mut diff = a.len() ^ b.len();
    for i in 0..max_len {
        let byte_a = a.get(i).copied().unwrap_or(0);
        let byte_b = b.get(i).copied().unwrap_or(0);
        diff |= (byte_a ^ byte_b) as usize;
    }
    std::hint::black_box(diff) == 0
}

/// Extract the real client IP address, only trusting forwarding headers from
/// configured trusted proxies (or private/loopback peers when unconfigured).
pub fn extract_real_ip(
    headers: &HeaderMap,
    fallback_addr: SocketAddr,
    trusted_proxies: Option<&[IpAddr]>,
) -> IpAddr {
    let connect_ip = fallback_addr.ip().to_canonical();

    let is_trusted_proxy = match trusted_proxies {
        Some(proxies) => proxies
            .iter()
            .any(|proxy| proxy.to_canonical() == connect_ip),
        None => is_private_ip(&connect_ip),
    };

    if !is_trusted_proxy {
        tracing::debug!("Using direct peer IP from ConnectInfo: {}", connect_ip);
        return connect_ip;
    }

    if let Some(real_ip) = headers.get("X-Real-IP") {
        if let Ok(ip_str) = real_ip.to_str() {
            if let Ok(ip) = ip_str.trim().parse::<IpAddr>() {
                let canonical = ip.to_canonical();
                tracing::debug!("Got IP from X-Real-IP header: {}", canonical);
                return canonical;
            }
        }
    }

    if let Some(forwarded_for) = headers.get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded_for.to_str() {
            if let Some(first_ip) = forwarded_str.split(',').next() {
                if let Ok(ip) = first_ip.trim().parse::<IpAddr>() {
                    let canonical = ip.to_canonical();
                    tracing::debug!("Got IP from X-Forwarded-For header: {}", canonical);
                    return canonical;
                }
            }
        }
    }

    tracing::debug!("Using fallback peer IP from ConnectInfo: {}", connect_ip);
    connect_ip
}

/// Check if an IP address is a private, loopback, link-local, or non-routable IP.
pub fn is_private_ip(ip: &IpAddr) -> bool {
    match ip.to_canonical() {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            ipv4.is_private()        // 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
                || ipv4.is_loopback()    // 127.0.0.0/8
                || ipv4.is_link_local()  // 169.254.0.0/16
                || ipv4.is_broadcast()   // 255.255.255.255
                || ipv4.is_unspecified() // 0.0.0.0
                // Carrier-grade NAT: 100.64.0.0/10
                || (octets[0] == 100 && (octets[1] & 0xc0) == 64)
                // Documentation addresses: 192.0.2.0/24, 198.51.100.0/24, 203.0.113.0/24
                || (octets[0] == 192 && octets[1] == 0 && octets[2] == 2)
                || (octets[0] == 198 && octets[1] == 51 && octets[2] == 100)
                || (octets[0] == 203 && octets[1] == 0 && octets[2] == 113)
        }
        IpAddr::V6(ipv6) => {
            ipv6.is_unspecified()        // ::
                || ipv6.is_loopback()        // ::1
                || ipv6.is_unique_local()    // fc00::/7
                || ipv6.is_unicast_link_local() // fe80::/10
                || ipv6.is_multicast()       // ff00::/8
                // Documentation addresses: 2001:db8::/32
                || (ipv6.segments()[0] == 0x2001 && ipv6.segments()[1] == 0x0db8)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;
    use std::str::FromStr;

    #[test]
    fn public_ipv4_and_mapped_ipv6_are_not_private() {
        let public_v4 = IpAddr::from_str("8.8.8.8").unwrap();
        let mapped_public_v6 = IpAddr::from_str("::ffff:8.8.8.8").unwrap();

        assert!(!is_private_ip(&public_v4));
        assert!(!is_private_ip(&mapped_public_v6));
    }

    #[test]
    fn private_and_cgnat_ips_are_private() {
        for addr in [
            "127.0.0.1",
            "::ffff:127.0.0.1",
            "10.0.0.1",
            "172.16.0.5",
            "192.168.1.1",
            "100.64.0.1",
            "::1",
            "fd00::1",
            "fe80::1",
        ] {
            let ip = IpAddr::from_str(addr).unwrap();
            assert!(is_private_ip(&ip), "Expected {addr} to be private");
        }
    }

    #[test]
    fn extract_real_ip_ignores_spoofed_headers_from_public_or_mapped_clients() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Real-IP", HeaderValue::from_static("1.1.1.1"));
        headers.insert("X-Forwarded-For", HeaderValue::from_static("2.2.2.2"));

        let public_peer = SocketAddr::from_str("8.8.8.8:12345").unwrap();
        let mapped_public_peer = SocketAddr::from_str("[::ffff:8.8.8.8]:12345").unwrap();

        assert_eq!(
            extract_real_ip(&headers, public_peer, None),
            IpAddr::from_str("8.8.8.8").unwrap()
        );
        assert_eq!(
            extract_real_ip(&headers, mapped_public_peer, None),
            IpAddr::from_str("8.8.8.8").unwrap()
        );
    }

    #[test]
    fn extract_real_ip_uses_headers_from_trusted_private_proxy() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Forwarded-For",
            HeaderValue::from_static(" 9.9.9.9 , 10.0.0.2 "),
        );

        let loopback_peer = SocketAddr::from_str("127.0.0.1:4567").unwrap();
        assert_eq!(
            extract_real_ip(&headers, loopback_peer, None),
            IpAddr::from_str("9.9.9.9").unwrap()
        );

        headers.insert("X-Real-IP", HeaderValue::from_static("::ffff:4.4.4.4"));
        assert_eq!(
            extract_real_ip(&headers, loopback_peer, None),
            IpAddr::from_str("4.4.4.4").unwrap()
        );
    }

    #[test]
    fn extract_real_ip_respects_explicit_trusted_proxies() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Real-IP", HeaderValue::from_static("5.5.5.5"));

        let loopback_peer = SocketAddr::from_str("127.0.0.1:4567").unwrap();
        assert_eq!(
            extract_real_ip(&headers, loopback_peer, Some(&[])),
            IpAddr::from_str("127.0.0.1").unwrap()
        );

        let proxy_list = [IpAddr::from_str("127.0.0.1").unwrap()];
        assert_eq!(
            extract_real_ip(&headers, loopback_peer, Some(&proxy_list)),
            IpAddr::from_str("5.5.5.5").unwrap()
        );
    }

    #[test]
    fn constant_time_eq_handles_all_cases() {
        assert!(constant_time_eq(b"secret-key", b"secret-key"));
        assert!(constant_time_eq(b"", b""));
        assert!(!constant_time_eq(b"secret-key", b"secret-kez"));
        assert!(!constant_time_eq(b"short", b"longer-value"));
        assert!(!constant_time_eq(b"", b"a"));
    }
}
