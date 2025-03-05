//! Connection Manager module for Implexa
//!
//! This module provides a ConnectionManager struct that uses interior mutability
//! to provide controlled access to the database connection.

use std::cell::RefCell;
use rusqlite::{Connection, Transaction};
use crate::database::schema::{DatabaseError, DatabaseResult};

/// Manager for database connections
pub struct ConnectionManager {
    /// Connection to the SQLite database with interior mutability
    connection: RefCell<Connection>,
}

impl ConnectionManager {
    /// Create a new ConnectionManager
    ///
    /// # Arguments
    ///
    /// * `connection` - Connection to the SQLite database
    ///
    /// # Returns
    ///
    /// A new ConnectionManager instance
    pub fn new(connection: Connection) -> Self {
        Self {
            connection: RefCell::new(connection),
        }
    }

    /// Execute a read-only operation on the database connection
    ///
    /// # Arguments
    ///
    /// * `operation` - Function that performs the operation
    ///
    /// # Returns
    ///
    /// The result of the operation
    ///
    /// # Errors
    ///
    /// Returns a rusqlite::Error if the operation fails
    pub fn execute<F, T>(&self, operation: F) -> Result<T, rusqlite::Error>
    where
        F: FnOnce(&Connection) -> Result<T, rusqlite::Error>,
    {
        let conn = self.connection.borrow();
        operation(&conn)
    }

    /// Execute a mutable operation on the database connection
    ///
    /// # Arguments
    ///
    /// * `operation` - Function that performs the operation
    ///
    /// # Returns
    ///
    /// The result of the operation
    ///
    /// # Errors
    ///
    /// Returns a rusqlite::Error if the operation fails
    pub fn execute_mut<F, T>(&self, operation: F) -> Result<T, rusqlite::Error>
    where
        F: FnOnce(&mut Connection) -> Result<T, rusqlite::Error>,
    {
        let mut conn = self.connection.borrow_mut();
        operation(&mut conn)
    }

    /// Execute an operation within a transaction
    ///
    /// # Arguments
    ///
    /// * `operation` - Function that performs the operation within a transaction
    ///
    /// # Returns
    ///
    /// The result of the operation
    ///
    /// # Errors
    ///
    /// Returns a rusqlite::Error if the operation fails
    pub fn transaction<F, T>(&self, operation: F) -> Result<T, rusqlite::Error>
    where
        F: FnOnce(&Transaction) -> Result<T, rusqlite::Error>,
    {
        let mut conn = self.connection.borrow_mut();
        let tx = conn.transaction()?;
        let result = operation(&tx);
        match result {
            Ok(value) => {
                tx.commit()?;
                Ok(value)
            }
            Err(err) => {
                // Transaction will automatically roll back when dropped
                Err(err)
            }
        }
    }

    /// Get a raw connection for operations that need direct access
    /// 
    /// Note: This should be used sparingly and with caution
    ///
    /// # Returns
    ///
    /// A mutable reference to the connection
    pub fn get_raw_connection(&self) -> std::cell::RefMut<'_, Connection> {
        self.connection.borrow_mut()
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use std::cell::RefCell;
    use rusqlite::{Connection, Transaction};
    use std::collections::HashMap;

    /// Mock connection manager for testing
    pub struct MockConnectionManager {
        /// Mock data storage
        data: RefCell<HashMap<String, Vec<HashMap<String, rusqlite::types::Value>>>>,
    }

    impl MockConnectionManager {
        /// Create a new MockConnectionManager
        pub fn new() -> Self {
            Self {
                data: RefCell::new(HashMap::new()),
            }
        }

        /// Execute a read-only operation with mock data
        pub fn execute<F, T>(&self, operation: F) -> Result<T, rusqlite::Error>
        where
            F: FnOnce(&Connection) -> Result<T, rusqlite::Error>,
        {
            // Create an in-memory database for testing
            let conn = Connection::open_in_memory()?;
            
            // Initialize with mock data
            // ...
            
            operation(&conn)
        }

        /// Execute a mutable operation with mock data
        pub fn execute_mut<F, T>(&self, operation: F) -> Result<T, rusqlite::Error>
        where
            F: FnOnce(&mut Connection) -> Result<T, rusqlite::Error>,
        {
            // Create an in-memory database for testing
            let mut conn = Connection::open_in_memory()?;
            
            // Initialize with mock data
            // ...
            
            operation(&mut conn)
        }

        /// Execute an operation within a transaction with mock data
        pub fn transaction<F, T>(&self, operation: F) -> Result<T, rusqlite::Error>
        where
            F: FnOnce(&Transaction) -> Result<T, rusqlite::Error>,
        {
            // Create an in-memory database for testing
            let mut conn = Connection::open_in_memory()?;
            
            // Initialize with mock data
            // ...
            
            let tx = conn.transaction()?;
            let result = operation(&tx);
            
            match result {
                Ok(value) => {
                    tx.commit()?;
                    Ok(value)
                }
                Err(err) => {
                    // Transaction will automatically roll back when dropped
                    Err(err)
                }
            }
        }
    }
}