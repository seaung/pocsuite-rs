use clap::{Parser, CommandFactory};
use std::collections::HashMap;
use clap_complete::{generate, Generator, Shell};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Target URL
    #[arg(short, long)]
    pub target: Option<String>,

    /// Request timeout in seconds
    #[arg(short, long, default_value = "30")]
    pub timeout: u64,

    /// Verify mode
    #[arg(short, long)]
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
        
        /// Port range to scan (e.g. 1-1000 or 22,80,443)
        #[arg(short, long)]
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
        #[arg(short, long)]
        ports: Option<String>,
        
        /// Number of concurrent threads
        #[arg(long, default_value = "100")]
        threads: usize,
        
        /// Output file path for scan results
        #[arg(short, long)]
        output: Option<String>,
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
        
        // 首先尝试从配置文件加载配置
        let mut config = if let Some(config_path) = args.config.as_ref() {
            match PocConfig::from_file(Path::new(config_path)) {
                Ok(config) => config,
                Err(_) => {
                    // 尝试加载YAML配置
                    if let Some(yaml_config) = ConfigManager::load_yaml_config(config_path) {
                        PocConfig {
                            target: yaml_config.target,
                            timeout: yaml_config.timeout.unwrap_or(30),
                            headers: yaml_config.headers.unwrap_or_default(),
                            verify: yaml_config.verify.unwrap_or(false),
                            exploit: yaml_config.exploit.unwrap_or(false),
                            poc_name: yaml_config.poc_name,
                            plugins: yaml_config.plugins.unwrap_or_default(),
                        }
                    } else {
                        PocConfig::default()
                    }
                }
            }
        } else {
            PocConfig::default()
        };
    
        // 首先处理全局参数
        if let Some(target) = &args.target {
            config.target = Some(target.clone());
        }
        if let Some(poc) = &args.poc {
            config.poc_name = Some(poc.clone());
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

        // 然后处理子命令中的参数，它们会覆盖全局参数
        if let Some(Commands::Scan { target, poc, verify, exploit, .. }) = &args.command {
            if let Some(t) = target {
                config.target = Some(t.clone());
            }
            if let Some(p) = poc {
                config.poc_name = Some(p.clone());
            }
            if *verify {
                config.verify = true;
            }
            if *exploit {
                config.exploit = true;
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