//! HTTP module for pocsuite-rs

use reqwest::Client;
use std::time::Duration;
use crate::core::PocError;

#[derive(Debug)]
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new(timeout: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { client }
    }
    
    pub async fn get(&self, url: &str) -> Result<reqwest::Response, PocError> {
        self.client
            .get(url)
            .send()
            .await
            .map_err(PocError::RequestError)
    }
    
    pub async fn post(&self, url: &str, body: &str) -> Result<reqwest::Response, PocError> {
        self.client
            .post(url)
            .body(body.to_string())
            .send()
            .await
            .map_err(PocError::RequestError)
    }
}