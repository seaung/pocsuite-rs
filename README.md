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

要开发新的 POC 插件，需要实现 `AsyncPoc` trait：

```rust
use async_trait::async_trait;
use pocsuite_rs::core::{AsyncPoc, PocConfig, PocResult, PocError};

#[derive(Debug)]
pub struct MyPoc {
    // 插件特定字段
}

impl MyPoc {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AsyncPoc for MyPoc {
    fn name(&self) -> String {
        "My Vulnerability".to_string()
    }
    
    fn description(&self) -> String {
        "This is my vulnerability description".to_string()
    }
    
    async fn verify(&self, config: &PocConfig) -> Result<PocResult, PocError> {
        // 实现漏洞验证逻辑
        todo!("实现漏洞验证逻辑")
    }
    
    async fn exploit(&self, config: &PocConfig) -> Result<PocResult, PocError> {
        // 实现漏洞利用逻辑
        todo!("实现漏洞利用逻辑")
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

## 资产扫描

### 基本用法

```rust
use pocsuite_rs::discovery::{DiscoveryConfig, PortScanner};
use std::time::Duration;

#[tokio::main]
async fn main() {
    // 创建端口扫描器
    let scanner = PortScanner::new();
    
    // 配置扫描参数
    let config = DiscoveryConfig {
        target: "192.168.1.0/24".to_string(),  // 目标网段
        ports: vec![21, 22, 80, 443, 3306, 6379],  // 要扫描的端口
        timeout: Duration::from_secs(5),  // 超时时间
        concurrency: 100,  // 并发数
    };
    
    // 执行扫描
    match scanner.discover(&config).await {
        Ok(hosts) => {
            for host in hosts {
                println!("发现主机: {}", host.ip);
                for (port, service) in host.ports {
                    println!("  端口 {}: {} {}", 
                        port,
                        service.name,
                        service.version.unwrap_or_default()
                    );
                }
            }
        }
        Err(e) => eprintln!("扫描失败: {}", e),
    }
}
```

### 子模块使用

#### 主机存活检测

```rust
use std::net::IpAddr;

// 检测单个主机是否存活
async fn check_host(scanner: &PortScanner, ip: IpAddr) {
    match scanner.check_alive(ip).await {
        Ok(true) => println!("主机 {} 存活", ip),
        Ok(false) => println!("主机 {} 未响应", ip),
        Err(e) => eprintln!("检测失败: {}", e),
    }
}
```

#### 端口扫描

```rust
// 扫描指定端口
async fn scan_host_ports(scanner: &PortScanner, ip: IpAddr) {
    let ports = vec![80, 443, 3306, 6379];
    match scanner.scan_ports(ip, &ports).await {
        Ok(open_ports) => {
            for (port, service) in open_ports {
                println!("发现开放端口 {}: {}", port, service.name);
            }
        }
        Err(e) => eprintln!("扫描失败: {}", e),
    }
}
```

#### 服务识别

```rust
use pocsuite_rs::discovery::service::ServiceIdentifier;

// 识别服务
async fn identify_port_service(ip: IpAddr, port: u16) {
    let identifier = ServiceIdentifier::new();
    match identifier.identify(ip, port).await {
        Ok(service) => {
            println!("服务名称: {}", service.name);
            if let Some(version) = service.version {
                println!("版本信息: {}", version);
            }
            if let Some(banner) = service.banner {
                println!("横幅信息: {}", banner);
            }
        }
        Err(e) => eprintln!("服务识别失败: {}", e),
    }
}
```

## 命令行使用

### 基本命令格式

```bash
pocsuite-rs [OPTIONS] <COMMAND> [ARGS]
```

### 命令

- `scan`: 执行漏洞检测
- `discover`: 执行资产扫描

### 全局选项

- `-c, --config <FILE>`: 指定配置文件路径
- `-v, --verbose`: 显示详细输出信息
- `-h, --help`: 显示帮助信息
- `-V, --version`: 显示版本信息

### 漏洞检测模式

```bash
# 对单个目标进行漏洞检测
pocsuite-rs scan -t http://example.com

# 从文件加载多个目标
pocsuite-rs scan -f targets.txt

# 指定POC插件
pocsuite-rs scan -t http://example.com -p redis_unauthorized

# 设置超时时间（秒）
pocsuite-rs scan -t http://example.com --timeout 30

# 启用漏洞利用模式
pocsuite-rs scan -t http://example.com -p redis_unauthorized --exploit
```

### 资产扫描模式

资产扫描模式（discover）用于发现目标网络中的存活主机和开放端口。

#### 基本参数

| 参数 | 说明 | 是否必需 | 示例 |
|------|------|----------|------|
| `-t, --target` | 目标IP或网段 | 是（与-f二选一） | 192.168.1.0/24 |
| `-f, --file` | 包含目标的文件 | 是（与-t二选一） | targets.txt |
| `-p, --ports` | 要扫描的端口 | 否 | 80,443 或 1-1000 |
| `--threads` | 并发线程数 | 否 | 200 |
| `-o, --output` | 结果输出文件 | 否 | result.json |

#### 使用示例

```bash
# 扫描单个目标
pocsuite-rs discover -t 192.168.1.1

# 扫描网段
pocsuite-rs discover -t 192.168.1.0/24

# 从文件加载多个目标
pocsuite-rs discover -f targets.txt

# 指定端口范围
pocsuite-rs discover -t 192.168.1.0/24 -p 1-1000

# 指定特定端口
pocsuite-rs discover -t 192.168.1.0/24 -p 22,80,443,3306

# 设置并发数
pocsuite-rs discover -t 192.168.1.0/24 --threads 200

# 导出扫描结果
pocsuite-rs discover -t 192.168.1.0/24 -o result.json

# 组合使用
pocsuite-rs discover -f targets.txt -p 80,443,3306 --threads 300 -o results.json
```

#### 输出格式

当使用 `-o` 参数指定输出文件时，扫描结果将以JSON格式保存，包含以下信息：
- 目标IP地址
- 主机存活状态
- 开放端口列表
- 端口对应的服务信息

```json
[
  {
    "ip": "192.168.1.1",
    "hostname": null,
    "is_alive": true,
    "ports": {
      "80": {
        "name": "http",
        "version": null,
        "banner": null
      }
    }
  }
]
```

### 配置文件示例

```yaml
# config.yaml
target: http://example.com
timeout: 30
headers:
  User-Agent: "Mozilla/5.0"
  Cookie: "session=xxx"
plugins:
  - redis_unauthorized
  - mysql_weak_password
output: result.json
```

使用配置文件：

```bash
pocsuite-rs scan -c config.yaml