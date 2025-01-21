//! 服务识别模块

use std::net::IpAddr;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::discovery::{ServiceInfo, DiscoveryError};

/// 服务识别器
#[derive(Debug, Default)]
pub struct ServiceIdentifier;

impl ServiceIdentifier {
    pub fn new() -> Self {
        Self
    }
    
    /// 获取服务横幅信息
    async fn get_banner(&self, ip: IpAddr, port: u16) -> Result<Option<String>, DiscoveryError> {
        let addr = format!("{ip}:{port}");
        let mut stream = TcpStream::connect(&addr)
            .await
            .map_err(|e| DiscoveryError::NetworkError(e.to_string()))?;
            
        // 发送简单的探测数据
        stream.write_all(b"\r\n")
            .await
            .map_err(|e| DiscoveryError::NetworkError(e.to_string()))?;
            
        // 读取响应
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer)
            .await
            .map_err(|e| DiscoveryError::NetworkError(e.to_string()))?;
            
        if n > 0 {
            Ok(Some(String::from_utf8_lossy(&buffer[..n]).to_string()))
        } else {
            Ok(None)
        }
    }
    
    /// 根据端口和横幅信息识别服务
    pub async fn identify(&self, ip: IpAddr, port: u16) -> Result<ServiceInfo, DiscoveryError> {
        let banner = self.get_banner(ip, port).await?;
        
        // 根据端口号和横幅信息判断服务类型
        let (name, version) = match port {
            21 => ("ftp".to_string(), None),
            22 => ("ssh".to_string(), self.parse_ssh_version(&banner)),
            23 => ("telnet".to_string(), None),
            25 => ("smtp".to_string(), None),
            80 | 443 => ("http".to_string(), self.parse_http_version(&banner)),
            3306 => ("mysql".to_string(), self.parse_mysql_version(&banner)),
            5432 => ("postgresql".to_string(), None),
            6379 => ("redis".to_string(), self.parse_redis_version(&banner)),
            27017 => ("mongodb".to_string(), None),
            _ => (format!("unknown_{}", port), None)
        };
        
        Ok(ServiceInfo {
            name,
            version,
            banner,
        })
    }
    
    /// 解析SSH版本信息
    fn parse_ssh_version(&self, banner: &Option<String>) -> Option<String> {
        banner.as_ref().and_then(|b| {
            if b.starts_with("SSH-") {
                b.split_whitespace().nth(1).map(String::from)
            } else {
                None
            }
        })
    }
    
    /// 解析HTTP版本信息
    fn parse_http_version(&self, banner: &Option<String>) -> Option<String> {
        banner.as_ref().and_then(|b| {
            b.lines().find(|line| line.starts_with("Server: "))
                .map(|line| line[8..].to_string())
        })
    }
    
    /// 解析MySQL版本信息
    fn parse_mysql_version(&self, banner: &Option<String>) -> Option<String> {
        banner.as_ref().and_then(|b| {
            if b.contains("mysql") {
                Some(b.trim().to_string())
            } else {
                None
            }
        })
    }
    
    /// 解析Redis版本信息
    fn parse_redis_version(&self, banner: &Option<String>) -> Option<String> {
        banner.as_ref().and_then(|b| {
            if b.contains("redis") {
                Some(b.trim().to_string())
            } else {
                None
            }
        })
    }
}