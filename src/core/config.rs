use clap::{Parser, CommandFactory};
use std::collections::HashMap;
use crate::core::PocConfig;
use clap_complete::{generate, Generator, Shell};

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
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
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
        
        PocConfig {
            target: args.target.unwrap_or_default(),
            timeout: args.timeout,
            headers: HashMap::new(),
            verify: args.verify,
            exploit: args.exploit,
        }
    }
}