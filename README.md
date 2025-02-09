# Pocsuite-rs

## 免责声明

本框架仅用于安全研究和学习目的。使用本框架进行测试时，您应当遵守《中华人民共和国网络安全法》等相关法律法规，并获得相关授权。

- 严禁对未授权的目标进行测试
- 严禁用于非法用途和破坏
- 如果您在使用本框架的过程中存在任何非法行为，您需自行承担相应后果，作者不承担任何法律及连带责任

在使用本框架前，请您务必审慎阅读、充分理解各条款内容，一旦您开始使用本框架，即视为您已完全理解并接受本免责声明。

## 简介

Pocsuite-rs 是一个用 Rust 编写的开源漏洞检测和利用框架。它提供了一个强大的平台，用于开发和执行漏洞验证（POC）和漏洞利用（Exploit）脚本。

## 特性

- 高性能：基于 Rust 语言开发，具有卓越的性能和内存安全性
- 异步支持：使用 tokio 运行时，支持异步 I/O 操作
- 插件系统：灵活的 POC 插件系统，支持自定义漏洞检测模块
- HTTP 客户端：内置 HTTP 客户端，支持常见的 Web 漏洞检测
- 并发执行：支持多个目标的并发扫描

## 环境要求

- Rust 1.70.0 或更高版本
- Cargo 包管理器
- Git（用于克隆代码库）

## 从源码构建

1. 克隆代码库：

```bash
git clone https://github.com/seaung/pocsuite-rs.git
cd pocsuite-rs
```

2. 构建项目：

```bash
cargo build --release
```

3. 运行测试（可选）：

```bash
cargo test
```

编译后的二进制文件将位于 `target/release` 目录下。

## 使用方法

### 基本用法

```rust
use pocsuite_rs::{PocRegistry, ExamplePoc, PocConfig, AsyncPoc};

#[tokio::main]
async fn main() {
    // 创建 POC 注册表
    let mut registry = PocRegistry::new();
    
    // 注册 POC 插件
    let example_poc = ExamplePoc::new();
    registry.register(Box::new(example_poc)).await;
    
    // 配置扫描参数
    let config = PocConfig {
        target: "http://example.com".to_string(),
        timeout: Some(10),
        headers: None,
        verify: true,
        exploit: false,
    };
    
    // 执行所有已注册的 POC
    let results = registry.run_all(&config).await;
    for result in results {
        match result {
            Ok(poc_result) => println!("扫描结果: {:?}", poc_result),
            Err(e) => eprintln!("扫描失败: {}", e),
        }
    }
}
```

## 开发 POC 插件

### 插件结构

一个完整的POC插件通常包含以下组件：

1. 漏洞信息结构（VulnInfo）：
```rust
#[derive(Debug)]
pub struct VulnInfo {
    pub cve_id: Option<String>,      // CVE编号
    pub cwe_id: Option<String>,      // CWE编号
    pub name: String,                // 漏洞名称
    pub description: String,         // 漏洞描述
    pub severity: Severity,          // 严重程度
    pub affected_versions: Vec<String>, // 受影响版本
    pub references: Vec<String>,     // 参考链接
}
```

2. POC实现结构：
```rust
#[derive(Debug)]
pub struct MyPoc {
    client: HttpClient,    // HTTP客户端
    info: VulnInfo,       // 漏洞信息
}
```

### 实现必要的trait

每个POC插件需要实现以下trait：

1. `Poc` trait - 基本信息：
```rust
impl Poc for MyPoc {
    fn get_name(&self) -> String {
        self.info.name.clone()
    }

    fn get_description(&self) -> String {
        self.info.description.clone()
    }
}
```

2. `AsyncPoc` trait - 核心功能：
```rust
#[async_trait]
impl AsyncPoc for MyPoc {
    async fn verify(&self, config: &PocConfig) -> Result<PocResult, PocError> {
        // 1. 版本检测
        if !self.check_version(&config.target).await? {
            return Ok(PocResult {
                success: false,
                name: self.get_name(),
                target: config.target.clone(),
                details: Some(String::from("目标版本不受影响")),
            });
        }

        // 2. 漏洞验证
        let response = self.client.get(&config.target).await?;
        
        Ok(PocResult {
            success: response.status().is_success(),
            name: self.get_name(),
            target: config.target.clone(),
            details: Some(format!("响应状态: {}", response.status())),
        })
    }

    async fn exploit(&self, config: &PocConfig) -> Result<PocResult, PocError> {
        // 1. 先验证漏洞是否存在
        let verify_result = self.verify(config).await?;
        if !verify_result.success {
            return Err(PocError::ExecutionError("目标不存在漏洞".to_string()));
        }

        // 2. 实现漏洞利用逻辑
        todo!("实现漏洞利用逻辑")
    }
}
```

### 最佳实践

1. 版本检测：
- 实现版本检测方法，避免对不受影响的版本进行测试
- 使用正则表达式或其他方式准确匹配版本信息

2. 错误处理：
- 使用 `PocError` 枚举处理各类错误情况
- 提供详细的错误信息，便于调试

3. HTTP请求：
- 使用框架提供的 `HttpClient`，支持超时和自定义头部
- 注意处理各种HTTP响应状态

4. 漏洞验证：
- 优先使用无害的方式验证漏洞
- 详细记录验证过程和结果

5. 漏洞利用：
- 实现前先验证漏洞存在
- 注意环境清理，避免留下后门

### 注册插件

开发完成后，需要将POC插件注册到框架：

```rust
let registry = PocRegistry::new();
let my_poc = MyPoc::new(timeout);
registry.register(my_poc).await;
```

## 项目结构

```
src/
├── core/       # 核心类型和特征定义
├── http/       # HTTP 客户端实现
├── pocs/       # POC 插件实现
├── utils/      # 工具函数
└── lib.rs      # 库入口
```

## 贡献

欢迎提交 Pull Request 或创建 Issue！

## 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 命令行使用
### 漏洞检测模式
#### 基本参数


| 参数 | 说明 | 是否必需 | 示例 |
|------|------|----------|------|
| `-t, --target` | 目标URL或IP | 是（与-f二选一） | http://example.com |
| `-f, --file` | 包含目标的文件 | 是（与-t二选一） | targets.txt |
| `-p, --plugin` | 指定POC插件 | 否 | redis_unauthorized |
| `--timeout` | 超时时间（秒） | 否 | 30 |
| `--exploit` | 启用漏洞利用模式 | 否 | - |
| `-c, --config` | 配置文件路径 | 否 | config.yaml |
| `--headers` | 自定义HTTP头 | 否 | "Cookie: session=xxx" |
| `-o, --output` | 结果输出文件 | 否 | result.json |