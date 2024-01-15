use crate::ports::database_port::DatabasePort;
use sled::{Db, IVec};
use std::error::Error;

/// A struct that serves as an adapter for the `DatabasePort` trait using the Sled embedded database.
pub struct SledDatabaseAdapter {
    db: Db, // The sled database instance.
}

/// Implement the `SledDatabaseAdapter` struct
impl SledDatabaseAdapter {
    /// Opens a new database or creates it if it doesn't exist, acting as an adapter.
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let db = sled::open(path)?;
        Ok(SledDatabaseAdapter { db })
    }
}

/// Implement the `DatabasePort` trait for the `SledDatabaseAdapter` struct.
/// This allows the adapter to be used as a port in the application, and it
/// also provides a concrete implementation of the `DatabasePort` interface.
impl DatabasePort for SledDatabaseAdapter {
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
