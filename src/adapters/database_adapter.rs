use crate::ports::database_port::DatabasePort;
use crate::ports::log_port::LoggerPort;
use sled::{Db, IVec};
use std::error::Error;
use std::sync::Arc;

/// A struct that serves as an adapter for the `DatabasePort` trait using the Sled embedded database.
pub struct DatabaseAdapter {
    db: Db, // The sled database instance.
}

impl DatabaseAdapter {
    /// Opens a new database or creates it if it doesn't exist, acting as an adapter.
    pub fn new(path: &str, logger: Arc<dyn LoggerPort>) -> Result<Self, Box<dyn Error>> {
        let db = sled::open(path)?;
        logger.log_info(&format!("Database opened at path: {}", path));
        Ok(DatabaseAdapter { db })
    }
}

/// Implement the `DatabasePort` trait for the `DatabaseAdapter` struct.
/// This allows the adapter to be used as a port in the application, and it
/// also provides a concrete implementation of the `DatabasePort` interface.
impl DatabasePort for DatabaseAdapter {
    /// Inserts a key-value pair
    fn insert(&self, key: &[u8], value: &[u8]) -> Result<Option<IVec>, Box<dyn Error>> {
        let previous_value = self.db.insert(key, value)?;
        self.db.flush()?; // Ensure that changes are written to disk through the adapter.
        Ok(previous_value)
    }

    /// Retrieves a value
    fn get(&self, key: &[u8]) -> Result<Option<IVec>, Box<dyn Error>> {
        let value = self.db.get(key)?;
        Ok(value)
    }

    /// Removes a key-value pair
    fn remove(&self, key: &[u8]) -> Result<Option<IVec>, Box<dyn Error>> {
        let previous_value = self.db.remove(key)?;
        self.db.flush()?; // Ensure that changes are written to disk through the adapter.
        Ok(previous_value)
    }
}
