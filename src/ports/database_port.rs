use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait DatabasePort: Send + Sync {
    async fn insert(&self, key: &[u8], value: &str) -> Result<Option<String>, Box<dyn Error + Send + Sync>>;
    async fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error + Send + Sync>>;
    async fn remove(&self, key: &str) -> Result<Option<String>, Box<dyn Error + Send + Sync>>;
}
