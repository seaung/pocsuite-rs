//! pocsuite-rs 的核心模块
//! 
//! 本模块定义了框架的核心类型和特征，包括:
//! - POC执行时的错误类型
//! - POC配置结构
//! - POC执行结果结构
//! - POC插件需要实现的特征

use std::collections::HashMap;
use thiserror::Error;

/// POC执行过程中可能出现的错误类型
#[derive(Debug, Error)]
pub enum PocError {
    /// HTTP请求错误，包装reqwest库的错误类型
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    /// URL解析错误，包装url库的错误类型
    #[error("Invalid URL: {0}")]
    UrlError(#[from] url::ParseError),
    
    /// POC执行错误，包含错误描述信息
    #[error("POC execution error: {0}")]
    ExecutionError(String),
}

/// POC执行配置
/// 
/// 包含执行POC时需要的各种配置参数:
/// - target: 目标URL
/// - timeout: 超时时间(秒)
/// - headers: 自定义HTTP请求头
/// - verify: 是否只进行验证
/// - exploit: 是否进行漏洞利用
#[derive(Debug)]
pub struct PocConfig {
    pub target: String,
    pub timeout: u64,
    pub headers: HashMap<String, String>,
    pub verify: bool,
    pub exploit: bool,
}

/// POC执行结果
/// 
/// 包含POC执行的结果信息:
/// - success: 是否成功
/// - name: POC名称
/// - target: 目标URL
/// - details: 详细结果信息
#[derive(Debug)]
pub struct PocResult {
    pub success: bool,
    pub name: String,
    pub target: String,
    pub details: Option<String>,
}

/// POC插件的基本特征
pub trait Poc {
    /// 获取POC名称
    fn get_name(&self) -> String;
    
    /// 获取漏洞描述
    fn get_description(&self) -> String;
}

/// POC插件的异步操作特征
#[async_trait::async_trait]
pub trait AsyncPoc: Poc {
    /// 验证目标是否存在漏洞
    async fn verify(&self, config: &PocConfig) -> Result<PocResult, PocError>;
    
    /// 利用漏洞获取权限
    async fn exploit(&self, config: &PocConfig) -> Result<PocResult, PocError>;
}
pub mod config;