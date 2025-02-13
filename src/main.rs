use log::info;
use clap::Parser;
use pocsuite_rs::core::config::{ConfigManager, print_completions, Args, Commands};
use pocsuite_rs::ui::{show_banner, create_spinner};
use pocsuite_rs::discovery::scanner::Scanner;
use pocsuite_rs::pocs::PocManager;
use pocsuite_rs::core::{PocResult, PocConfig};
use pocsuite_rs::ui::table::ResultTable;

#[tokio::main]
async fn main() {
    show_banner();
    
    let args = Args::parse();
    
    // 初始化日志系统
    env_logger::Builder::new()
        .filter_level(if args.verbose { log::LevelFilter::Debug } else { log::LevelFilter::Info })
        .format_timestamp(Some(env_logger::TimestampPrecision::Millis))
        .format_target(true)
        .format_module_path(true)
        .init();
    
    log::debug!("命令行参数: {:?}", args);
    
    match args.command {
        Some(Commands::Completion { shell }) => {
            print_completions(shell);
            return;
        }
        Some(Commands::List) => {
            let spinner = create_spinner("正在加载POC插件列表...");
            let poc_manager = PocManager::new();
            let pocs = poc_manager.list().await;
            spinner.finish_with_message("POC插件列表加载完成！");
            
            println!("
可用的POC插件列表:
");
            println!("{:<30} {:<15} {}", "名称", "漏洞类型", "描述");
            println!("{:-<80}", "");
            
            for poc in pocs {
                println!("{:<30} {:<15} {}",
                    poc.name,
                    poc.vuln_type,
                    poc.description
                );
            }
            return;
        }
        Some(Commands::Search { keyword }) => {
            let spinner = create_spinner("正在搜索POC插件...");
            let poc_manager = PocManager::new();
            let pocs = poc_manager.search(&keyword).await;
            spinner.finish_with_message("POC插件搜索完成！");
            
            if pocs.is_empty() {
                println!("
未找到匹配的POC插件。");
                return;
            }
            
            println!("
搜索结果:
");
            println!("{:<30} {:<15} {}", "名称", "漏洞类型", "描述");
            println!("{:-<80}", "");
            
            for poc in pocs {
                println!("{:<30} {:<15} {}",
                    poc.name,
                    poc.vuln_type,
                    poc.description
                );
            }
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
            
            log::debug!("扫描目标: {}", targets.join(","));
            log::debug!("端口配置: {:?}", ports);
            log::debug!("并发线程数: {}", threads);
            
            let scanner = Scanner::new(targets.join(","), ports, threads);
            match scanner.scan().await {
                Ok(hosts) => {
                    spinner.finish_with_message("资产发现完成！");
                    
                    // 打印扫描结果
                    println!("
扫描结果:
");
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
                                println!("
扫描结果已保存到: {}", output_path);
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
            
            log::debug!("详细配置信息:");
            log::debug!("- 目标: {:?}", config.target);
            log::debug!("- 超时时间: {}秒", config.timeout);
            log::debug!("- 自定义请求头: {:?}", config.headers);
            log::debug!("- 验证模式: {}", config.verify);
            log::debug!("- 利用模式: {}", config.exploit);
            log::debug!("- POC名称: {:?}", config.poc_name);
            log::debug!("- 插件列表: {:?}", config.plugins);
            
            info!("Starting pocsuite-rs with config: {:?}", config);
            
            // 检查是否有目标
            if config.target.is_none() {
                eprintln!("错误：未指定目标");
                return;
            }
            
            // 加载POC插件
            let spinner = create_spinner("正在加载POC插件...");
            let poc_manager = PocManager::new();
            
            // 获取指定的POC插件
            let poc_name = if let Some(name) = config.poc_name {
                name
            } else {
                spinner.finish_with_message("错误：未指定POC插件");
                eprintln!("请使用 -p 参数指定要使用的POC插件");
                return;
            };
            
            // 执行POC检测
            let target = config.target.unwrap();
            info!("正在对目标 {} 执行 {} POC检测...", target, poc_name);
            
            // 创建结果表格
            let mut result_table = ResultTable::new();
            
            // 解析目标列表
            let targets: Vec<String> = target.split(',').map(|s| s.trim().to_string()).collect();
            
            // 对每个目标执行POC检测
            for target in targets {
                let mut poc_config = PocConfig {
                    target: target.clone(),
                    timeout: config.timeout,
                    headers: config.headers.clone(),
                    verify: config.verify,
                    exploit: config.exploit,
                };
                
                // 根据verify和exploit标志执行相应模式
                if config.verify {
                    info!("执行验证模式...");
                    if let Ok(poc) = poc_manager.get_poc(&poc_name).await {
                        match poc.verify(&poc_config).await {
                            Ok(result) => result_table.add_result(result),
                            Err(e) => result_table.add_result(PocResult {
                                success: false,
                                name: poc_name.clone(),
                                target: poc_config.target.clone(),
                                details: Some(format!("检测失败: {}", e)),
                            }),
                        }
                    }
                }
                
                if config.exploit {
                    info!("执行利用模式...");
                    if let Ok(poc) = poc_manager.get_poc(&poc_name).await {
                        match poc.exploit(&poc_config).await {
                            Ok(result) => result_table.add_result(result),
                            Err(e) => result_table.add_result(PocResult {
                                success: false,
                                name: poc_name.clone(),
                                target: poc_config.target,
                                details: Some(format!("利用失败: {}", e)),
                            }),
                        }
                    }
                }
            }
            
            spinner.finish_with_message("POC检测完成！");
            
            // 显示检测结果表格
            result_table.display();
        }
    }
}
