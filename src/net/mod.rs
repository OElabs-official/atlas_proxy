use std::net::{IpAddr as StdIpAddr, Ipv4Addr, Ipv6Addr};
use serde::{Serialize, Deserialize};

/// IP 地址枚举（用于 TOML 配置文件序列化）
/// 
/// 与 `std::net::IpAddr` 兼容。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Ip {
    V4([u8; 4]),
    V6([u16; 8]),
}

impl Ip {
    /// 从 std::net::IpAddr 创建
    pub fn from_std(ip: StdIpAddr) -> Self {
        match ip {
            StdIpAddr::V4(v4) => {
                let bytes = v4.octets();
                Ip::V4([bytes[0], bytes[1], bytes[2], bytes[3]])
            }
            StdIpAddr::V6(v6) => {
                let segments = v6.segments();
                Ip::V6([segments[0], segments[1], segments[2], segments[3], segments[4], segments[5], segments[6], segments[7]])
            }
        }
    }

    /// 转换为 std::net::IpAddr
    pub fn to_std(&self) -> StdIpAddr {
        match self {
            Ip::V4(bytes) => StdIpAddr::V4(Ipv4Addr::new(bytes[0], bytes[1], bytes[2], bytes[3])),
            Ip::V6(segments) => StdIpAddr::V6(Ipv6Addr::new(segments[0], segments[1], segments[2], segments[3], segments[4], segments[5], segments[6], segments[7])),
        }
    }
}

/// IP 地址类型别名（直接使用 std::net::IpAddr）
pub type IpAddr = StdIpAddr;

/// 端口转发规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortForward {
    pub input: (IpAddr, u16),
    pub output: u16,
}

/// 获取本地 IP 地址
pub fn get_local_ip() -> Vec<IpAddr> {
    let mut output = vec![];
    if let Ok(nets) = local_ip_address::list_afinet_netifas() {
        for (_name, ip) in nets {
            if !ip.is_loopback() {
                output.push(ip);
            }
        }
    }
    output
}

/// 打印 IP 地址列表
pub fn print_ips(input: &[IpAddr]) {
    for ip in input {
        println!("{}", ip);
    }
}
