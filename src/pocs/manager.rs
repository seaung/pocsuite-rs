#[derive(Debug, Default)]
pub struct PocRegistry;

impl PocRegistry {
    pub async fn list(&self) -> Vec<String> {
        // TODO: 实现从文件系统或其他来源加载POC插件列表
        vec!["example".to_string(), "redis".to_string()]
    }
}

#[derive(Debug, Default)]
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