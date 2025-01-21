use log::info;
use clap::Parser;
use pocsuite_rs::core::config::{ConfigManager, print_completions, Args, Commands};
use pocsuite_rs::ui::{show_banner, create_spinner};
use pocsuite_rs::discovery::scanner::Scanner;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    // 根据verbose参数设置日志级别
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", if args.verbose { "debug" } else { "info" });
    env_logger::Builder::from_env(env)
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .init();
    show_banner();

    let args = Args::parse();
    
    match args.command {
        Some(Commands::Completion { shell }) => {
            print_completions(shell);
            return;
        }
        Some(Commands::Discover { target, file, ports, threads, output }) => {
            let spinner = create_spinner("正在初始化资产发现模块...");
            
            // 获取目标列表
            let targets = if let Some(file_path) = file {
                match std::fs::read_to_string(&file_path) {
                    Ok(content) => content.lines().map(|s| s.trim().to_string()).collect::<Vec<_>>(),
                    Err(e) => {
                        eprintln!("读取目标文件失败: {}", e);
                        return;
                    }
                }
            } else if let Some(t) = target {
                vec![t]
            } else {
                eprintln!("错误：必须指定目标或目标文件");
                return;
            };
            
            let scanner = Scanner::new(targets.join(","), ports, threads);
            match scanner.scan().await {
                Ok(hosts) => {
                    spinner.finish_with_message("资产发现完成！");
                    
                    // 打印扫描结果
                    println!("\n扫描结果:\n");
                    println!("IP地址\t\t开放端口\t服务信息");
                    println!("-----------------------------------------");
                    
                    for host in &hosts {
                        if !host.ports.is_empty() {
                            for (port, service) in &host.ports {
                                println!("{:15}\t{:5}\t{}", 
                                    host.ip,
                                    port,
                                    service.name
                                );
                            }
                        }
                    }
                    
                    // 保存结果到文件
                    if let Some(output_path) = output {
                        if let Ok(json) = serde_json::to_string_pretty(&hosts) {
                            if let Err(e) = std::fs::write(&output_path, json) {
                                eprintln!("保存结果到文件失败: {}", e);
                            } else {
                                println!("\n扫描结果已保存到: {}", output_path);
                            }
                        }
                    }
                    
                    info!("发现 {} 个主机，共 {} 个开放端口", 
                        hosts.len(),
                        hosts.iter().map(|h| h.ports.len()).sum::<usize>()
                    );
                }
                Err(e) => {
                    spinner.finish_with_message("资产发现失败！");
                    eprintln!("扫描错误: {}", e);
                }
            }
            return;
        }
        _ => {
            let spinner = create_spinner("正在加载配置...");
            let config = ConfigManager::init();
            spinner.finish_with_message("配置加载完成！");
            
            info!("Starting pocsuite-rs with config: {:?}", config);
        }
    }
}
