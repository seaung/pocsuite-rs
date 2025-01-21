use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;
use std::time::Duration;

const BANNERS: [&str; 3] = [
    r#"
    ____                      _ __                        
   / __ \____  _________    (_) /____     _______  _____
  / /_/ / __ \/ ___/ __ \  / / __/ _ \   / ___/ / / / _ \
 / ____/ /_/ / /__/ /_/ / / / /_/  __/  / /  / /_/ /  __/
/_/    \____/\___/\____/_/_/\__/\___/  /_/   \__,_/\___/ 
                                                          
"#,
    r#"
 ▄▄▄·      ▄▄· .▄▄ · ▄• ▄▌▪  ▄▄▄▄▄▄▄▄ .    ▄▄▄  .▄▄ · 
▐█ ▄█▪     ▐█ ▌▪▐█ ▀. █▪██▌██ •██  ▀▄.▀·    ▀▄ █·▐█ ▀. 
 █▀▀█ ▄█▀▄ ██ ▄▄▄▀▀▀█▄█▌▐█▌▐█· ▐█.▪▐▀▀▪▄    ▐▀▀▄ ▄▀▀▀█▄
██▄▪ ▐█▌.▐▌▐███▌▐█▄▪▐█▐█▄█▌▐█▌ ▐█▌·▐█▄▄▌    ▐█•█▌▐█▄▪▐█
·▀▀▀▀ ▀█▄▀▪·▀▀▀  ▀▀▀▀  ▀▀▀ ▀▀▀ ▀▀▀  ▀▀▀     .▀  ▀ ▀▀▀▀ 
"#,
    r#"
██████╗  ██████╗  ██████╗███████╗██╗   ██╗██╗████████╗███████╗
██╔══██╗██╔═══██╗██╔════╝██╔════╝██║   ██║██║╚══██╔══╝██╔════╝
██████╔╝██║   ██║██║     ███████╗██║   ██║██║   ██║   █████╗  
██╔═══╝ ██║   ██║██║     ╚════██║██║   ██║██║   ██║   ██╔══╝  
██║     ╚██████╔╝╚██████╗███████║╚██████╔╝██║   ██║   ███████╗
╚═╝      ╚═════╝  ╚═════╝╚══════╝ ╚═════╝ ╚═╝   ╚═╝   ╚══════╝
"#,
];

pub fn show_banner() {
    let banner = BANNERS.choose(&mut rand::thread_rng()).unwrap();
    println!("{}", banner.bright_cyan());
    println!("{}", "Pocsuite-rs - A Modern Vulnerability Testing Framework".bright_yellow());
    println!("{}", "Version: 0.1.0".bright_green());
    println!();
}

pub fn create_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("=>-"));
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

pub fn create_spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.set_message(msg.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}