//! 端口扫描器实现

use std::net::IpAddr;
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::time::timeout;
use std::time::Duration;
use std::str::FromStr;
use crate::discovery::{Discovery, DiscoveryConfig, DiscoveryError, Host, ServiceInfo};

/// 端口扫描器
#[derive(Debug, Default)]
pub struct Scanner {
    target: String,
    ports: Option<String>,
    concurrency: usize,
}

impl Scanner {
    pub fn new(target: String, ports: Option<String>, threads: usize) -> Self {
        Self {
            target,
            ports,
            concurrency: threads,
        }
    }

    /// 解析目标字符串，支持多种格式：
    /// - 单个IP: 192.168.1.1
    /// - IP范围: 192.168.1.1-192.168.1.100
    /// - CIDR格式: 192.168.1.0/24
    /// - 多个目标: 192.168.1.1,192.168.1.2
    fn parse_target(&self) -> Result<Vec<IpAddr>, DiscoveryError> {
        let mut ips = Vec::new();
        
        for target in self.target.split(',') {
            let target = target.trim();
            if target.contains('/') {
                // CIDR格式
                let network = target.parse::<ipnetwork::IpNetwork>()
                    .map_err(|e| DiscoveryError::InvalidParameter(format!("无效的CIDR格式: {}", e)))?;
                ips.extend(network.iter());
            } else if target.contains('-') {
                // IP范围格式
                let parts: Vec<&str> = target.split('-').collect();
                if parts.len() != 2 {
                    return Err(DiscoveryError::InvalidParameter(format!("无效的IP范围格式: {}", target)));
                }
                let start_ip = parts[0].trim().parse::<IpAddr>()
                    .map_err(|e| DiscoveryError::InvalidParameter(format!("无效的起始IP: {}", e)))?;
                let end_ip = parts[1].trim().parse::<IpAddr>()
                    .map_err(|e| DiscoveryError::InvalidParameter(format!("无效的结束IP: {}", e)))?;
                // 实现IP范围遍历
                match (start_ip, end_ip) {
                    (IpAddr::V4(start), IpAddr::V4(end)) => {
                        let start_num = u32::from(start);
                        let end_num = u32::from(end);
                        if start_num <= end_num {
                            for ip_num in start_num..=end_num {
                                ips.push(IpAddr::V4(std::net::Ipv4Addr::from(ip_num)));
                            }
                        }
                    },
                    (IpAddr::V6(start), IpAddr::V6(end)) => {
                        let start_segments = start.segments();
                        let end_segments = end.segments();
                        // 简化处理：仅支持最后一个段的范围
                        if start_segments[..7] == end_segments[..7] {
                            let start_num = start_segments[7];
                            let end_num = end_segments[7];
                            if start_num <= end_num {
                                for last_segment in start_num..=end_num {
                                    let mut segments = start_segments;
                                    segments[7] = last_segment;
                                    ips.push(IpAddr::V6(std::net::Ipv6Addr::from(segments)));
                                }
                            }
                        }
                    },
                    _ => return Err(DiscoveryError::InvalidParameter("IP版本不匹配".to_string())),
                }
            } else {
                // 单个IP
                let ip = target.parse::<IpAddr>()
                    .map_err(|e| DiscoveryError::InvalidParameter(format!("无效的IP地址: {}", e)))?;
                ips.push(ip);
            }
        }
        
        Ok(ips)
    }

    pub async fn scan(&self) -> Result<Vec<Host>, DiscoveryError> {
        let config = DiscoveryConfig {
            target: self.target.clone(),
            ports: self.parse_ports()?,
            timeout: Duration::from_secs(5),
            concurrency: self.concurrency,
        };
        self.discover(&config).await
    }

    fn parse_ports(&self) -> Result<Vec<u16>, DiscoveryError> {
        let mut result = Vec::new();
        if let Some(ports_str) = &self.ports {
            for part in ports_str.split(',') {
                if part.contains('-') {
                    let range: Vec<&str> = part.split('-').collect();
                    if range.len() == 2 {
                        let start = u16::from_str(range[0].trim())
                            .map_err(|e| DiscoveryError::InvalidParameter(e.to_string()))?;
                        let end = u16::from_str(range[1].trim())
                            .map_err(|e| DiscoveryError::InvalidParameter(e.to_string()))?;
                        result.extend(start..=end);
                    }
                } else {
                    let port = u16::from_str(part.trim())
                        .map_err(|e| DiscoveryError::InvalidParameter(e.to_string()))?;
                    result.push(port);
                }
            }
        } else {
            // 默认扫描常用端口
            result.extend_from_slice(&[21, 22, 23, 25, 53, 80, 110, 139, 443, 445, 1433, 1521, 3306, 3389, 5432, 6379, 8080]);
        }
        Ok(result)
    }
    
    /// 检查单个端口是否开放
    async fn check_port(&self, ip: IpAddr, port: u16, timeout_secs: u64) -> bool {
        let addr = format!("{ip}:{port}");
        match timeout(
            std::time::Duration::from_secs(timeout_secs),
            TcpStream::connect(&addr)
        ).await {
            Ok(Ok(_)) => true,
            _ => false
        }
    }
}

#[async_trait::async_trait]
impl Discovery for Scanner {
    async fn discover(&self, config: &DiscoveryConfig) -> Result<Vec<Host>, DiscoveryError> {
        // 解析目标
        let ips = self.parse_target()?;
        let mut hosts = Vec::new();
        
        // 遍历所有IP
        for ip in ips {
            // 检查主机存活
            if self.check_alive(ip).await? {
                // 扫描端口
                let ports = self.scan_ports(ip, &config.ports).await?;
                
                hosts.push(Host {
                    ip,
                    hostname: None, // TODO: 实现反向DNS解析
                    is_alive: true,
                    ports,
                });
            }
        }
        
        Ok(hosts)
    }
    
    async fn check_alive(&self, ip: IpAddr) -> Result<bool, DiscoveryError> {
        // 使用ICMP ping或TCP SYN探测主机存活
        // 这里简化处理，只检查80端口
        Ok(self.check_port(ip, 80, 2).await)
    }
    
    async fn scan_ports(&self, ip: IpAddr, ports: &[u16]) -> Result<HashMap<u16, ServiceInfo>, DiscoveryError> {
        let mut open_ports = HashMap::new();
        
        for &port in ports {
            if self.check_port(ip, port, 2).await {
                // 对开放端口进行服务识别
                if let Ok(service) = self.identify_service(ip, port).await {
                    open_ports.insert(port, service);
                }
            }
        }
        
        Ok(open_ports)
    }
    
    async fn identify_service(&self, _ip: IpAddr, port: u16) -> Result<ServiceInfo, DiscoveryError> {
        // TODO: 实现更复杂的服务识别逻辑
        Ok(ServiceInfo {
            name: format!("unknown_{}", port),
            version: None,
            banner: None,
        })
    }
}