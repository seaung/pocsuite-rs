//! 端口扫描器实现

use std::net::IpAddr;
use std::collections::HashMap;
use tokio::net::TcpStream;
use tokio::time::timeout;
use crate::discovery::{Discovery, DiscoveryConfig, DiscoveryError, Host, ServiceInfo};

/// 端口扫描器
#[derive(Debug, Default)]
pub struct PortScanner;

impl PortScanner {
    pub fn new() -> Self {
        Self
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
impl Discovery for PortScanner {
    async fn discover(&self, config: &DiscoveryConfig) -> Result<Vec<Host>, DiscoveryError> {
        // 解析目标网段
        let network = config.target.parse::<ipnetwork::IpNetwork>()
            .map_err(|e| DiscoveryError::InvalidParameter(e.to_string()))?;
            
        let mut hosts = Vec::new();
        
        // 遍历网段中的每个IP
        for ip in network.iter() {
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
    
    async fn identify_service(&self, ip: IpAddr, port: u16) -> Result<ServiceInfo, DiscoveryError> {
        // TODO: 实现更复杂的服务识别逻辑
        Ok(ServiceInfo {
            name: format!("unknown_{}", port),
            version: None,
            banner: None,
        })
    }
}