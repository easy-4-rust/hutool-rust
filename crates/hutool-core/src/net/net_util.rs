//! 对齐: `cn.hutool.core.net.NetUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/net/NetUtil.java
//!
//! 基于 `std::net` 的本机端口 / IPv4 / 主机名辅助（无强制系统探测依赖）。

use std::net::{
    IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket,
};
use std::time::Duration;

use idna::domain_to_ascii;

use crate::net::ipv4_util::Ipv4Util;

/// 对齐 Java 类: `cn.hutool.core.net.NetUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct NetUtil;

impl NetUtil {
    /// 对齐 Java: `NetUtil.PORT_RANGE_MIN`
    pub const PORT_RANGE_MIN: u16 = 1024;
    /// 对齐 Java: `NetUtil.PORT_RANGE_MAX`
    pub const PORT_RANGE_MAX: u16 = 65_535;

    /// 对齐 Java: `NetUtil.longToIpv4(long)`
    pub fn long_to_ipv4(long_ip: u32) -> String {
        Ipv4Util::long_to_ipv4(long_ip)
    }

    /// 对齐 Java: `NetUtil.ipv4ToLong(String)`
    pub fn ipv4_to_long(str_ip: &str) -> Option<u32> {
        Ipv4Util::ipv4_to_long(str_ip)
    }

    /// 对齐 Java: `NetUtil.isValidPort(int)`
    pub fn is_valid_port(port: i32) -> bool {
        (0..=65_535).contains(&port)
    }

    /// 对齐 Java: `NetUtil.isUsableLocalPort(int)`
    pub fn is_usable_local_port(port: i32) -> bool {
        if !Self::is_valid_port(port) {
            return false;
        }
        TcpListener::bind(("127.0.0.1", port as u16)).is_ok()
    }

    /// 对齐 Java: `NetUtil.getUsableLocalPort()`
    pub fn get_usable_local_port() -> Option<u16> {
        Self::get_usable_local_port_in_range(Self::PORT_RANGE_MIN, Self::PORT_RANGE_MAX)
    }

    /// 对齐 Java: `NetUtil.getUsableLocalPort(int minPort)`
    pub fn get_usable_local_port_from(min_port: u16) -> Option<u16> {
        Self::get_usable_local_port_in_range(min_port, Self::PORT_RANGE_MAX)
    }

    /// 对齐 Java: `NetUtil.getUsableLocalPort(int minPort, int maxPort)`
    pub fn get_usable_local_port_in_range(min_port: u16, max_port: u16) -> Option<u16> {
        if min_port > max_port {
            return None;
        }
        if let Ok(listener) = TcpListener::bind(("127.0.0.1", 0)) {
            if let Ok(addr) = listener.local_addr() {
                let port = addr.port();
                if (min_port..=max_port).contains(&port) {
                    return Some(port);
                }
            }
        }
        for port in min_port..=max_port {
            if Self::is_usable_local_port(port as i32) {
                return Some(port);
            }
        }
        None
    }

    /// 对齐 Java: `NetUtil.getUsableLocalPorts(int, int, int)`
    pub fn get_usable_local_ports(num_requested: usize, min_port: u16, max_port: u16) -> Vec<u16> {
        let mut ports = Vec::new();
        let mut cursor = min_port;
        while ports.len() < num_requested && cursor <= max_port {
            if Self::is_usable_local_port(cursor as i32) {
                ports.push(cursor);
            }
            if cursor == u16::MAX {
                break;
            }
            cursor += 1;
        }
        ports
    }

    /// 对齐 Java: `NetUtil.isInnerIP(String)`
    pub fn is_inner_ip(ip_address: &str) -> bool {
        Ipv4Util::is_inner_ip(ip_address)
    }

    /// 对齐 Java: `NetUtil.hideIpPart(String)`
    pub fn hide_ip_part(ip: &str) -> String {
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() == 4 {
            format!("{}.{}.*.*", parts[0], parts[1])
        } else {
            ip.to_string()
        }
    }

    /// 对齐 Java: `NetUtil.hideIpPart(long)`
    pub fn hide_ip_part_long(ip: u32) -> String {
        Self::hide_ip_part(&Self::long_to_ipv4(ip))
    }

    /// 对齐 Java: `NetUtil.buildInetSocketAddress` / `createAddress`
    pub fn create_address(host: &str, port: u16) -> Option<SocketAddr> {
        let host = host.trim();
        if host.is_empty() {
            return Some(SocketAddr::from(([127, 0, 0, 1], port)));
        }
        if let Ok(ip) = host.parse::<IpAddr>() {
            return Some(SocketAddr::new(ip, port));
        }
        if let Some((h, p)) = host.rsplit_once(':') {
            if let Ok(parsed_port) = p.parse::<u16>() {
                return (h, parsed_port).to_socket_addrs().ok()?.next();
            }
        }
        (host, port).to_socket_addrs().ok()?.next()
    }

    /// 对齐 Java: `NetUtil.getIpByHost(String)`
    pub fn get_ip_by_host(host_name: &str) -> Option<String> {
        (host_name, 0u16)
            .to_socket_addrs()
            .ok()?
            .next()
            .map(|addr| addr.ip().to_string())
    }

    /// 对齐 Java: `NetUtil.getLocalhostStr()`
    pub fn get_localhost_str() -> String {
        Self::get_localhost()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "127.0.0.1".to_string())
    }

    /// 对齐 Java: `NetUtil.getLocalhost()` — UDP connect 探测本机出网地址。
    pub fn get_localhost() -> Option<IpAddr> {
        if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
            if socket.connect("8.8.8.8:80").is_ok() {
                if let Ok(addr) = socket.local_addr() {
                    let ip = addr.ip();
                    if !ip.is_unspecified() {
                        return Some(ip);
                    }
                }
            }
        }
        Some(IpAddr::V4(Ipv4Addr::LOCALHOST))
    }

    /// 对齐 Java: `NetUtil.getLocalHostName()`
    pub fn get_local_host_name() -> String {
        std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("COMPUTERNAME"))
            .unwrap_or_else(|_| "localhost".to_string())
    }

    /// 对齐 Java: `NetUtil.localIpv4s()` — 至少包含 loopback。
    pub fn local_ipv4s() -> Vec<String> {
        let mut ips = vec![Ipv4Addr::LOCALHOST.to_string()];
        if let Some(IpAddr::V4(v4)) = Self::get_localhost() {
            if !v4.is_loopback() {
                ips.push(v4.to_string());
            }
        }
        ips.sort();
        ips.dedup();
        ips
    }

    /// 对齐 Java: `NetUtil.localIpv6s()`
    pub fn local_ipv6s() -> Vec<String> {
        vec![Ipv6Addr::LOCALHOST.to_string()]
    }

    /// 对齐 Java: `NetUtil.localIps()`
    pub fn local_ips() -> Vec<String> {
        let mut ips = Self::local_ipv4s();
        ips.extend(Self::local_ipv6s());
        ips.sort();
        ips.dedup();
        ips
    }

    /// 对齐 Java: `NetUtil.isInRange(String ip, String cidr)`
    pub fn is_in_range(ip: &str, cidr: &str) -> bool {
        let Some((base, bits)) = cidr.split_once('/') else {
            return false;
        };
        let Ok(mask_bit) = bits.parse::<i32>() else {
            return false;
        };
        let Some(ip_long) = Ipv4Util::ipv4_to_long(ip) else {
            return false;
        };
        let Some(begin) = Ipv4Util::get_begin_ip_long(base, mask_bit) else {
            return false;
        };
        let Some(end) = Ipv4Util::get_end_ip_long(base, mask_bit) else {
            return false;
        };
        (begin..=end).contains(&ip_long)
    }

    /// 对齐 Java: `NetUtil.idnToASCII(String)`
    pub fn idn_to_ascii(unicode: &str) -> Option<String> {
        domain_to_ascii(unicode).ok()
    }

    /// 对齐 Java: `NetUtil.isUnknown(String)`
    pub fn is_unknown(check_string: &str) -> bool {
        let s = check_string.trim();
        s.is_empty() || s.eq_ignore_ascii_case("unknown")
    }

    /// 对齐 Java: `NetUtil.getMultistageReverseProxyIp(String)`
    pub fn get_multistage_reverse_proxy_ip(ip: &str) -> String {
        if !ip.contains(',') {
            return ip.trim().to_string();
        }
        ip.split(',')
            .map(str::trim)
            .find(|part| !Self::is_unknown(part))
            .unwrap_or(ip)
            .to_string()
    }

    /// 对齐 Java: `NetUtil.toAbsoluteUrl(String, String)`
    pub fn to_absolute_url(absolute_base_path: &str, relative_path: &str) -> String {
        if relative_path.starts_with("http://") || relative_path.starts_with("https://") {
            return relative_path.to_string();
        }
        let base = absolute_base_path.trim_end_matches('/');
        let rel = relative_path.trim_start_matches('/');
        format!("{base}/{rel}")
    }

    /// 对齐 Java: `NetUtil.isOpen(InetSocketAddress, int timeout)`
    pub fn is_open(addr: SocketAddr, timeout_ms: u64) -> bool {
        TcpStream::connect_timeout(&addr, Duration::from_millis(timeout_ms)).is_ok()
    }

    /// IPv6 文本合法性（BigInteger 互转的前置校验）。
    pub fn is_valid_ipv6(ipv6_str: &str) -> bool {
        ipv6_str.parse::<Ipv6Addr>().is_ok()
    }

    /// 对齐 Java: `NetUtil.ping(String)` — TCP:80 连接探测近似。
    pub fn ping(ip: &str) -> bool {
        Self::ping_with_timeout(ip, 200)
    }

    /// 对齐 Java: `NetUtil.ping(String, int)`
    pub fn ping_with_timeout(ip: &str, timeout_ms: u64) -> bool {
        if let Ok(addr) = format!("{ip}:80").parse::<SocketAddr>() {
            return Self::is_open(addr, timeout_ms);
        }
        if let Some(addr) = Self::create_address(ip, 80) {
            return Self::is_open(addr, timeout_ms);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_and_ip_helpers() {
        assert!(NetUtil::is_valid_port(8080));
        assert!(!NetUtil::is_valid_port(70_000));
        assert_eq!(NetUtil::long_to_ipv4(0x7F000001), "127.0.0.1");
        assert!(NetUtil::is_inner_ip("10.0.0.1"));
        assert_eq!(NetUtil::hide_ip_part("1.2.3.4"), "1.2.*.*");
        assert!(NetUtil::is_in_range("192.168.1.10", "192.168.1.0/24"));
        assert!(NetUtil::is_unknown(" unknown "));
        assert_eq!(
            NetUtil::get_multistage_reverse_proxy_ip("unknown, 1.2.3.4"),
            "1.2.3.4"
        );
        let port = NetUtil::get_usable_local_port().expect("usable port");
        assert!(NetUtil::is_usable_local_port(port as i32));
    }
}
