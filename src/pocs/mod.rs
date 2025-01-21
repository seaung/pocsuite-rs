//! POC module for pocsuite-rs

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::core::AsyncPoc;

pub mod example;
pub mod redis;
pub mod manager;

pub use manager::{PocManager, PocInfo};

/// POC Registry for managing all available POC plugins
#[derive(Default)]
pub struct PocRegistry {
    pocs: Arc<RwLock<HashMap<String, Arc<Box<dyn AsyncPoc + Send + Sync>>>>>,
}

impl PocRegistry {
    pub fn new() -> Self {
        Self {
            pocs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a new POC plugin
    pub async fn register<P: AsyncPoc + Send + Sync + 'static>(&self, poc: P) {
        let mut pocs = self.pocs.write().await;
        pocs.insert(poc.get_name(), Arc::new(Box::new(poc)));
    }

    /// Get a POC plugin by name
    pub async fn get(&self, name: &str) -> Option<Arc<Box<dyn AsyncPoc + Send + Sync>>> {
        let pocs = self.pocs.read().await;
        pocs.get(name).cloned()
    }

    /// List all available POC plugins
    pub async fn list(&self) -> Vec<String> {
        let pocs = self.pocs.read().await;
        pocs.keys().cloned().collect()
    }
}

// Re-export example POC
pub use example::ExamplePoc;