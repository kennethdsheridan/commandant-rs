use async_trait::async_trait;
use std::error::Error;

/// `DatabasePort` Trait
///
/// Defines an interface for interacting with a database. This trait abstracts the functionality
/// for performing basic CRUD operations, allowing different implementations to tailor database
/// interactions.
#[async_trait]
pub trait DatabasePort: Send + Sync {
    async fn insert(&self, key: &str, value: &str) -> Result<Option<String>, Box<dyn Error>>;
    async fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>>;
    async fn remove(&self, key: &str) -> Result<Option<String>, Box<dyn Error>>;
}
