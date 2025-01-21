# Pocsuite-rs

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
git clone https://github.com/your-username/pocsuite-rs.git
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

## 安装

确保你的系统已安装 Rust 工具链，然后执行：

```bash
cargo install pocsuite-rs
```

## 使用方法

### 基本用法

```rust
use pocsuite_rs::{PocRegistry, ExamplePoc, PocConfig};

#[tokio::main]
async fn main() {
    // 创建 POC 注册表
    let registry = PocRegistry::new();
    
    // 注册 POC 插件
    let example_poc = ExamplePoc::new(10); // 10秒超时
    registry.register(example_poc).await;
    
    // 配置扫描参数
    let config = PocConfig {
        target: "http://example.com".to_string(),
        timeout: 10,
        headers: Default::default(),
        verify: true,
        exploit: false,
    };
    
    // 获取并执行 POC
    if let Some(poc) = registry.get("Example Vulnerability").await {
        match poc.verify(&config).await {
            Ok(result) => println!("扫描结果: {:?}", result),
            Err(e) => eprintln!("扫描失败: {}", e),
        }
    }
}
```

## 开发 POC 插件

要开发新的 POC 插件，需要实现 `Poc` trait：

```rust
use async_trait::async_trait;
use pocsuite_rs::core::{Poc, PocConfig, PocResult, PocError};

#[derive(Debug)]
pub struct MyPoc {
    // 插件特定字段
}

#[async_trait]
impl Poc for MyPoc {
    fn get_name(&self) -> String {
        "My Vulnerability".to_string()
    }
    
    fn get_description(&self) -> String {
        "This is my vulnerability description".to_string()
    }
    
    async fn verify(&self, config: &PocConfig) -> Result<PocResult, PocError> {
        // 实现漏洞验证逻辑
    }
    
    async fn exploit(&self, config: &PocConfig) -> Result<PocResult, PocError> {
        // 实现漏洞利用逻辑
    }
}
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