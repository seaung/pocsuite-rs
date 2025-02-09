use clap::{Parser, CommandFactory};
use std::collections::HashMap;
use clap_complete::{generate, Generator, Shell};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, disable_version_flag = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Target URL
    #[arg(short, long)]
    pub target: Option<String>,

    /// Request timeout in seconds
    #[arg(short = 'T', long, default_value = "30")]
    pub timeout: u64,

    /// Verify mode
    #[arg(short = 'V', long)]
    pub verify: bool,

    /// Exploit mode
    #[arg(short, long)]
    pub exploit: bool,

    /// POC name to use
    #[arg(short, long)]
    pub poc: Option<String>,

    /// Config file path
    #[arg(short = 'c', long)]
    pub config: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct PocConfig {
    pub target: Option<String>,
    pub timeout: u64,
    pub headers: HashMap<String, String>,
    pub verify: bool,
    pub exploit: bool,
    pub poc_name: Option<String>,
    pub plugins: Vec<String>,
}

impl PocConfig {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: PocConfig = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn merge_with_args(&mut self, args: &Args) {
        if let Some(target) = &args.target {
            self.target = Some(target.clone());
        }
        if args.timeout > 0 {
            self.timeout = args.timeout;
        }
        if args.verify {
            self.verify = true;
        }
        if args.exploit {
            self.exploit = true;
        }
        if let Some(poc) = &args.poc {
            self.poc_name = Some(poc.clone());
        }
    }
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Execute vulnerability scan
    Scan {
        /// Target IP or network range (e.g. 192.168.1.0/24)
        #[arg(short, long)]
        target: Option<String>,
        
        /// File containing target list
        #[arg(short, long)]
        file: Option<String>,
        
        /// Port range to scan (e.g. 1-1000 or 22,80,443)
        #[arg(short = 'P', long)]
        ports: Option<String>,
        
        /// Number of concurrent threads
        #[arg(long, default_value = "100")]
        threads: usize,
        
        /// Output file path for scan results
        #[arg(short, long)]
        output: Option<String>,

        /// POC name to use
        #[arg(short, long)]
        poc: Option<String>,

        /// Verify mode
        #[arg(short, long)]
        verify: bool,

        /// Exploit mode
        #[arg(short, long)]
        exploit: bool,

        /// Config file path
        #[arg(short = 'c', long)]
        config: Option<String>,
    },
    /// Execute asset discovery scan
    Discover {
        /// Target IP or network range (e.g. 192.168.1.0/24)
        #[arg(short, long, required_unless_present = "file")]
        target: Option<String>,
        
        /// File containing target IPs or network ranges
        #[arg(short, long, required_unless_present = "target")]
        file: Option<String>,
        
        /// Port range to scan (e.g. 1-1000 or 22,80,443)
        #[arg(short = 'P', long)]
        ports: Option<String>,
        
        /// Number of concurrent threads
        #[arg(long, default_value = "100")]
        threads: usize,
        
        /// Output file path for scan results
        #[arg(short, long)]
        output: Option<String>,
    },
    /// List all available POC plugins
    List,
    /// Search POC plugins by keyword
    Search {
        /// Search keyword
        #[arg(short, long)]
        keyword: String,
    },
}

#[derive(Deserialize, Debug)]
pub struct YamlConfig {
    pub target: Option<String>,
    pub timeout: Option<u64>,
    pub headers: Option<HashMap<String, String>>,
    pub verify: Option<bool>,
    pub exploit: Option<bool>,
    pub poc_name: Option<String>,
    pub plugins: Option<Vec<String>>,
}

pub struct ConfigManager;

pub fn print_completions<G: Generator>(gen: G) {
    let mut cmd = Args::command();
    let name = cmd.get_name().to_string();
    generate(gen, &mut cmd, name, &mut std::io::stdout());
}

impl ConfigManager {
    pub fn init() -> PocConfig {
        let args = Args::parse();
        
        // 1. 首先创建默认配置（最低优先级）
        let mut config = PocConfig::default();
        
        // 2. 处理全局参数（中等优先级）
        if args.target.is_some() {
            config.target = args.target.clone();
        }
        if args.poc.is_some() {
            config.poc_name = args.poc.clone();
            config.plugins.clear(); // 清空plugins列表，确保poc_name生效
        }
        if args.verify {
            config.verify = true;
        }
        if args.exploit {
            config.exploit = true;
        }
        if args.timeout > 0 {
            config.timeout = args.timeout;
        }
        
        // 3. 处理子命令参数
        if let Some(Commands::Scan { target, poc, verify, exploit, config: scan_config, .. }) = &args.command {
            // 保存命令行中的poc参数
            let cmd_poc = poc.clone();
            
            // 3.1 如果子命令中指定了配置文件，优先使用该配置
            if let Some(config_path) = scan_config {
                let file_config = match PocConfig::from_file(Path::new(config_path)) {
                    Ok(cfg) => Some(cfg),
                    Err(_) => ConfigManager::load_yaml_config(config_path).map(|yaml_config| {
                        let mut cfg = PocConfig {
                            target: yaml_config.target,
                            timeout: yaml_config.timeout.unwrap_or(30),
                            headers: yaml_config.headers.unwrap_or_default(),
                            verify: yaml_config.verify.unwrap_or(false),
                            exploit: yaml_config.exploit.unwrap_or(false),
                            poc_name: yaml_config.poc_name,
                            plugins: yaml_config.plugins.unwrap_or_default(),
                        };
                        // 如果配置文件中指定了plugins，则覆盖poc_name
                        if !cfg.plugins.is_empty() {
                            cfg.poc_name = None;
                        }
                        cfg
                    })
                };
                
                if let Some(file_cfg) = file_config {
                    config = file_cfg;
                    // 确保命令行参数中的poc优先级最高
                    if cmd_poc.is_some() {
                        config.poc_name = cmd_poc.clone();
                    }
                }
            } else if let Some(config_path) = args.config.as_ref() {
                // 3.2 如果全局参数中指定了配置文件，使用该配置
                let file_config = match PocConfig::from_file(Path::new(config_path)) {
                    Ok(cfg) => Some(cfg),
                    Err(_) => ConfigManager::load_yaml_config(config_path).map(|yaml_config| {
                        let mut cfg = PocConfig {
                            target: yaml_config.target,
                            timeout: yaml_config.timeout.unwrap_or(30),
                            headers: yaml_config.headers.unwrap_or_default(),
                            verify: yaml_config.verify.unwrap_or(false),
                            exploit: yaml_config.exploit.unwrap_or(false),
                            poc_name: yaml_config.poc_name,
                            plugins: yaml_config.plugins.unwrap_or_default(),
                        };
                        // 如果配置文件中指定了plugins，则覆盖poc_name
                        if !cfg.plugins.is_empty() {
                            cfg.poc_name = None;
                        }
                        cfg
                    })
                };
                
                if let Some(file_cfg) = file_config {
                    config = file_cfg;
                }
            }
            
            // 3.3 最后应用子命令中的具体参数（最高优先级）
            // 首先设置POC名称，确保其优先级最高
            if cmd_poc.is_some() {
                config.poc_name = cmd_poc;
                config.plugins.clear(); // 清空plugins列表，确保poc_name生效
            }

            // 然后处理目标设置
            if target.is_some() {
                config.target = target.clone();
            } else if let Some(Commands::Scan { file, .. }) = &args.command {
                // 从文件读取目标列表
                if let Some(file_path) = file {
                    match fs::read_to_string(file_path) {
                        Ok(content) => {
                            let targets: Vec<String> = content
                                .lines()
                                .map(|line| line.trim().to_string())
                                .filter(|line| !line.is_empty())
                                .collect();
                            if !targets.is_empty() {
                                config.target = Some(targets.join(","));
                            }
                        }
                        Err(e) => eprintln!("读取目标文件失败: {}", e),
                    }
                }
            }
            if *verify {
                config.verify = true;
            }
            if *exploit {
                config.exploit = true;
            }
        } else if let Some(config_path) = args.config.as_ref() {
            // 4. 如果不是scan子命令但指定了配置文件，加载该配置
            let file_config = match PocConfig::from_file(Path::new(config_path)) {
                Ok(cfg) => Some(cfg),
                Err(_) => ConfigManager::load_yaml_config(config_path).map(|yaml_config| PocConfig {
                    target: yaml_config.target,
                    timeout: yaml_config.timeout.unwrap_or(30),
                    headers: yaml_config.headers.unwrap_or_default(),
                    verify: yaml_config.verify.unwrap_or(false),
                    exploit: yaml_config.exploit.unwrap_or(false),
                    poc_name: yaml_config.poc_name,
                    plugins: yaml_config.plugins.unwrap_or_default(),
                })
            };
            
            if let Some(file_cfg) = file_config {
                config = file_cfg;
            }
        }
        
        config
    }

    fn load_yaml_config(path: &str) -> Option<YamlConfig> {
        if !Path::new(path).exists() {
            eprintln!("配置文件 '{}' 不存在", path);
            return None;
        }

        match fs::read_to_string(path) {
            Ok(contents) => match serde_yaml::from_str(&contents) {
                Ok(config) => Some(config),
                Err(e) => {
                    eprintln!("解析配置文件失败: {}", e);
                    None
                }
            },
            Err(e) => {
                eprintln!("读取配置文件失败: {}", e);
                None
            }
        }
    }
}