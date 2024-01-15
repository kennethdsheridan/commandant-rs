use sled::{Db, IVec};
use std::error::Error;

/// `DatabasePort` Trait
///
/// Defines an interface for interacting with a database. This trait abstracts the functionality
/// for performing basic CRUD operations, allowing different implementations to tailor database
/// interactions.
pub trait DatabasePort: Send + Sync {
    fn insert(&self, key: &[u8], value: &[u8]) -> Result<Option<IVec>, Box<dyn Error>>;
    fn get(&self, key: &[u8]) -> Result<Option<IVec>, Box<dyn Error>>;
    fn remove(&self, key: &[u8]) -> Result<Option<IVec>, Box<dyn Error>>;
}

/// A struct that implements the `DatabasePort` trait using the Sled embedded database.
pub struct SledDatabasePort {
    db: Db,
}

impl SledDatabasePort {
    /// Opens a new database or creates it if it doesn't exist.
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let db = sled::open(path)?;
        Ok(SledDatabasePort { db })
    }
}

impl DatabasePort for SledDatabasePort {
    /// Inserts a key-value pair into the database.
    fn insert(&self, key: &[u8], value: &[u8]) -> Result<Option<IVec>, Box<dyn Error>> {
        let previous_value = self.db.insert(key, value)?;
        self.db.flush()?; // Ensure that changes are written to disk.
        Ok(previous_value)
    }

    /// Retrieves a value from the database by key.
    fn get(&self, key: &[u8]) -> Result<Option<IVec>, Box<dyn Error>> {
        let value = self.db.get(key)?;
        Ok(value)
    }

    /// Removes a key-value pair from the database.
    fn remove(&self, key: &[u8]) -> Result<Option<IVec>, Box<dyn Error>> {
        let previous_value = self.db.remove(key)?;
        self.db.flush()?; // Ensure that changes are written to disk.
        Ok(previous_value)
    }
}
