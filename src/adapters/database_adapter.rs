use std::fmt;

use std::error::Error;
use std::sync::Arc;
use surrealdb::{sql, Surreal};
use surrealdb::engine::local::Db;
use surrealdb::sql::{thing, Thing};
use serde_json;
use async_trait::async_trait;
use common::ports::log_port::LoggerPort;
use crate::ports::database_port::DatabasePort;


#[derive(Debug)]
struct ParseThingError(String);

impl fmt::Display for ParseThingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseThingError {}

/// A struct that serves as an adapter for the `DatabasePort` trait using SurrealDB.
pub struct DatabaseAdapter {
    db: Surreal<Db>, // The SurrealDB instance using the local engine.
}

impl DatabaseAdapter {
    /// Opens a new database or creates it if it doesn't exist, acting as an adapter.
    /// This function is asynchronous due to SurrealDB's async nature.
    pub async fn new(path: &str, logger: Arc<dyn LoggerPort>) -> Result<Self, Box<dyn Error>> {
        // Create a new SurrealDB instance with the local engine
        let db = Surreal::new::<surrealdb::engine::local::File>(path).await?;
        
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
    async fn insert(&self, key: &[u8], value: &str) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
        let key_str = String::from_utf8_lossy(key).to_string();
        let thing_id = format!("key_value:{}", key_str);
        let thing = thing(&thing_id)
            .map_err(|e| Box::new(ParseThingError(format!("Failed to create Thing: {:?}", e))) as Box<dyn Error + Send + Sync>)?;

        let created: Option<serde_json::Value> = self.db
            .create(thing)
            .content(serde_json::json!({
                "key": key_str,
                "value": value
            }))
            .await?;

        Ok(created.and_then(|v| v["value"].as_str().map(String::from)))
    }


    /// Retrieves a value from the database given a key
    async fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
        let thing: Thing = ("key_value", key).into();
        let record: Option<serde_json::Value> = self.db
            .select(thing)
            .await?;
        Ok(record.and_then(|v| v["value"].as_str().map(String::from)))
    }

    /// Removes a key-value pair from the database
    async fn remove(&self, key: &str) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
        let thing: Thing = ("key_value", key).into();
        let deleted: Option<serde_json::Value> = self.db
            .delete(thing)
            .await?;
        Ok(deleted.and_then(|v| v["value"].as_str().map(String::from)))
    }
}
