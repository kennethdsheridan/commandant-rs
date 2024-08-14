use std::error::Error;
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use serde_json;
use async_trait::async_trait;
use common::ports::log_port::LoggerPort;
use crate::ports::database_port::DatabasePort;

/// A struct that serves as an adapter for the `DatabasePort` trait using SurrealDB.
pub struct DatabaseAdapter {
    db: Surreal<Db>, // The SurrealDB instance using the local engine.
}

impl DatabaseAdapter {
    /// Opens a new database or creates it if it doesn't exist, acting as an adapter.
    /// This function is asynchronous due to SurrealDB's async nature.
    pub async fn new(path: &str, logger: Arc<dyn LoggerPort>) -> Result<Self, Box<dyn Error>> {
        // Create a new SurrealDB instance with the local engine
        let db = Surreal::new::<Db>(path).await?;
        
        // Use the 'default' namespace and database
        db.use_ns("default").use_db("default").await?;
        
        // Log the successful database connection
        logger.log_info(&format!("Database opened at path: {}", path));
        
        // Return the new DatabaseAdapter instance
        Ok(DatabaseAdapter { db })
    }
}

/// Implement the `DatabasePort` trait for the `DatabaseAdapter` struct.
#[async_trait]
impl DatabasePort for DatabaseAdapter {
    /// Inserts a key-value pair into the database
    async fn insert(&self, key: &str, value: &str) -> Result<Option<String>, Box<dyn Error>> {
        let created: Option<serde_json::Value> = self.db
            .create(("key_value", key))
            .content(serde_json::json!({ "value": value }))
            .await?;
        Ok(created.and_then(|v| v["value"].as_str().map(String::from)))
    }

    /// Retrieves a value from the database given a key
    async fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let record: Option<serde_json::Value> = self.db
            .select(("key_value", key))
            .await?;
        Ok(record.and_then(|v| v["value"].as_str().map(String::from)))
    }

    /// Removes a key-value pair from the database
    async fn remove(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let deleted: Option<serde_json::Value> = self.db
            .delete(("key_value", key))
            .await?;
        Ok(deleted.and_then(|v| v["value"].as_str().map(String::from)))
    }
}
