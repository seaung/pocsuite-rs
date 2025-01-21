use colored::*;
use std::fmt;
use crate::core::PocResult;

pub struct ResultTable {
    results: Vec<PocResult>,
}

impl ResultTable {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_result(&mut self, result: PocResult) {
        self.results.push(result);
    }

    pub fn display(&self) {
        println!("{}", "漏洞检测结果".bold());
        println!("{}", "=".repeat(80));
        println!("{:<20} {:<30} {:<10} {}", "漏洞名称", "目标URL", "状态", "详情");
        println!("{}", "-".repeat(80));

        for result in &self.results {
            let status = if result.success {
                "成功".green()
            } else {
                "失败".red()
            };

            let details = result.details.as_deref().unwrap_or("-");
            println!(
                "{:<20} {:<30} {:<10} {}",
                result.name,
                result.target,
                status,
                details
            );
        }
        println!("{}", "=".repeat(80));
    }
}

impl Default for ResultTable {
    fn default() -> Self {
        Self::new()
    }
}