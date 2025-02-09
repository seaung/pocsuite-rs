use std::sync::Arc;
use crate::core::AsyncPoc;

#[derive(Default)]
pub struct PocRegistry {
    pocs: Arc<tokio::sync::RwLock<std::collections::HashMap<String, Arc<Box<dyn AsyncPoc + Send + Sync>>>>>,
}

impl PocRegistry {
    pub fn new() -> Self {
        Self {
            pocs: Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn get(&self, name: &str) -> Option<Arc<Box<dyn AsyncPoc + Send + Sync>>> {
        let pocs = self.pocs.read().await;
        pocs.get(name).cloned()
    }

    pub async fn list(&self) -> Vec<String> {
        // TODO: 实现从文件系统或其他来源加载POC插件列表
        vec!["example".to_string(), "redis".to_string()]
    }
}

#[derive(Default)]
pub struct PocManager {
    registry: PocRegistry,
}

#[derive(Debug)]
pub struct PocInfo {
    pub name: String,
    pub vuln_type: String,
    pub description: String,
}

impl PocManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn get_poc(&self, name: &str) -> Result<Arc<Box<dyn AsyncPoc + Send + Sync>>, Box<dyn std::error::Error>> {
        self.registry.get(name).await
            .ok_or_else(|| Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, format!("POC {} not found", name))) as Box<dyn std::error::Error>)
    }
    
    pub async fn list(&self) -> Vec<PocInfo> {
        let poc_names = self.registry.list().await;
        poc_names.into_iter().map(|name| {
            PocInfo {
                name,
                vuln_type: String::from("未知"),
                description: String::from("暂无描述"),
            }
        }).collect()
    }
    
    pub async fn search(&self, keyword: &str) -> Vec<PocInfo> {
        let all_pocs = self.list().await;
        all_pocs.into_iter()
            .filter(|poc| poc.name.contains(keyword))
            .collect()
    }
}