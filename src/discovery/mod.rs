//! 资产发现模块
//! 
//! 提供内网资产探测功能，包括：
//! - 主机存活检测
//! - 端口扫描
//! - 服务识别

use std::net::IpAddr;
use std::collections::HashMap;
use tokio::time::Duration;
use thiserror::Error;

/// 资产发现错误类型
#[derive(Debug, Error)]
pub enum DiscoveryError {
    /// 网络错误
    #[error("Network error: {0}")]
    NetworkError(String),
    
    /// 超时错误
    #[error("Operation timeout")]
    TimeoutError,
    
    /// 参数错误
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
}

/// 主机信息
#[derive(Debug, Clone)]
pub struct Host {
    /// IP地址
    pub ip: IpAddr,
    /// 主机名
    pub hostname: Option<String>,
    /// 是否存活
    pub is_alive: bool,
    /// 开放端口及服务信息
    pub ports: HashMap<u16, ServiceInfo>,
}

/// 服务信息
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    /// 服务名称
    pub name: String,
    /// 服务版本
    pub version: Option<String>,
    /// 服务横幅信息
    pub banner: Option<String>,
}

/// 资产发现配置
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// 目标网段
    pub target: String,
    /// 端口范围
    pub ports: Vec<u16>,
    /// 超时时间
    pub timeout: Duration,
    /// 并发数
    pub concurrency: usize,
}

/// 资产发现特征
#[async_trait::async_trait]
pub trait Discovery {
    /// 执行资产发现
    async fn discover(&self, config: &DiscoveryConfig) -> Result<Vec<Host>, DiscoveryError>;
    
    /// 检测主机存活
    async fn check_alive(&self, ip: IpAddr) -> Result<bool, DiscoveryError>;
    
    /// 扫描端口
    async fn scan_ports(&self, ip: IpAddr, ports: &[u16]) -> Result<HashMap<u16, ServiceInfo>, DiscoveryError>;
    
    /// 识别服务
    async fn identify_service(&self, ip: IpAddr, port: u16) -> Result<ServiceInfo, DiscoveryError>;
}

// 重导出子模块
pub mod scanner;
pub mod service;