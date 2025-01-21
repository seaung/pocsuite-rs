use log::info;
use clap::Parser;
use pocsuite_rs::core::config::{ConfigManager, print_completions, Args, Commands};
use pocsuite_rs::ui::{show_banner, create_spinner};

#[tokio::main]
async fn main() {
    env_logger::init();
    show_banner();

    let args = Args::parse();
    
    if let Some(Commands::Completion { shell }) = args.command {
        print_completions(shell);
        return;
    }
    
    let spinner = create_spinner("正在加载配置...");
    let config = ConfigManager::init();
    spinner.finish_with_message("配置加载完成！");
    
    info!("Starting pocsuite-rs with config: {:?}", config);
}
