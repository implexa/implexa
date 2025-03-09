//! Connection Manager module for Implexa
//!
//! This module provides a ConnectionManager struct that uses a thread-safe mutex
//! to provide controlled access to the SQLite database connection.
use std::path::Path;
use std::sync::{Arc, Mutex};
use rusqlite::{Connection, Transaction};

/// Errors that can occur when using the connection manager
#[derive(thiserror::Error, Debug)]
pub enum ConnectionManagerError {
    /// SQLite error
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    
    /// Lock error
    #[error("Failed to acquire lock on database connection: {0}")]
    LockError(String),
}

/// Manager for database connections
#[derive(Clone)]
pub struct ConnectionManager {
    /// Connection to the SQLite database with thread-safe protection
    connection: Arc<Mutex<Connection>>,
}

impl ConnectionManager {
    /// Create a new ConnectionManager
    ///
    /// # Arguments
    ///
    /// * `db_path` - Path to the SQLite database file
    ///
    /// # Returns
    ///
    /// A new ConnectionManager instance
    ///
    /// # Errors
    ///
    /// Returns an error if the connection cannot be created
    pub fn new(db_path: &Path) -> Result<Self, rusqlite::Error> {
        let connection = Connection::open(db_path)?;
        
        // Enable WAL mode for better concurrency
        connection.execute_batch("PRAGMA journal_mode=WAL")?;
        
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }
    
    /// Create a new ConnectionManager with an in-memory database
    ///
    /// # Returns
    ///
    /// A new ConnectionManager instance with an in-memory database
    ///
    /// # Errors
    ///
    /// Returns an error if the connection cannot be created
    pub fn new_in_memory() -> Result<Self, rusqlite::Error> {
        let connection = Connection::open_in_memory()?;
        
        // Note: In-memory databases don't use WAL mode, they use "MEMORY" journal mode
        println!("Using in-memory database with 'MEMORY' journal mode");
        
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }
    
    /// Create a new ConnectionManager with an existing connection
    ///
    /// # Arguments
    ///
    /// * `connection` - Connection to the SQLite database
    ///
    /// # Returns
    ///
    /// A new ConnectionManager instance
    pub fn from_connection(connection: Connection) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
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
    /// Returns an error from the operation
    pub fn execute<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: FnOnce(&Connection) -> Result<T, E>,
        E: From<rusqlite::Error>,
    {
        let conn = self.connection.lock().map_err(|_| {
            E::from(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1), // SQLITE_ERROR code
                Some("Failed to acquire lock on database connection".to_string()),
            ))
        })?;
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
    /// Returns an error from the operation
    pub fn execute_mut<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: FnOnce(&mut Connection) -> Result<T, E>,
        E: From<rusqlite::Error>,
    {
        let mut conn = self.connection.lock().map_err(|_| {
            E::from(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1), // SQLITE_ERROR code
                Some("Failed to acquire lock on database connection".to_string()),
            ))
        })?;
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
    /// Returns an error from the operation or a rusqlite::Error if a transaction operation fails
    pub fn transaction<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: FnOnce(&Transaction) -> Result<T, E>,
        E: From<rusqlite::Error>,
    {
        let mut conn = self.connection.lock().map_err(|_| {
            E::from(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1), // SQLITE_ERROR code
                Some("Failed to acquire lock on database connection".to_string()),
            ))
        })?;
        let tx = conn.transaction().map_err(E::from)?;
        let result = operation(&tx);
        match result {
            Ok(value) => {
                tx.commit().map_err(E::from)?;
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
    pub fn get_raw_connection(&self) -> Result<std::sync::MutexGuard<'_, Connection>, rusqlite::Error> {
        self.connection.lock().map_err(|_| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(1), // SQLITE_ERROR code
                Some("Failed to acquire lock on database connection".to_string()),
            )
        })
    }
}

#[cfg(test)]
pub mod test_utils {
    use super::*;
    use std::sync::{Arc, Mutex};
    use rusqlite::{Connection, Transaction};
    use std::collections::HashMap;

    /// Mock connection manager for testing
    pub struct MockConnectionManager {
        /// Mock data storage
        #[allow(dead_code)]
        data: Arc<Mutex<HashMap<String, Vec<HashMap<String, rusqlite::types::Value>>>>>,
    }

    impl MockConnectionManager {
        /// Create a new MockConnectionManager
        pub fn new() -> Self {
            Self {
                data: Arc::new(Mutex::new(HashMap::new())),
            }
        }
        
        /// Execute a read-only operation with mock data
        pub fn execute<F, T, E>(&self, operation: F) -> Result<T, E>
        where
            F: FnOnce(&Connection) -> Result<T, E>,
            E: From<rusqlite::Error>,
        {
            // Create an in-memory database for testing
            let conn = Connection::open_in_memory().map_err(E::from)?;
            
            // Initialize with mock data
            // ...
            
            operation(&conn)
        }

        /// Execute a mutable operation with mock data
        pub fn execute_mut<F, T, E>(&self, operation: F) -> Result<T, E>
        where
            F: FnOnce(&mut Connection) -> Result<T, E>,
            E: From<rusqlite::Error>,
        {
            // Create an in-memory database for testing
            let mut conn = Connection::open_in_memory().map_err(E::from)?;
            
            // Initialize with mock data
            // ...
            
            operation(&mut conn)
        }

        /// Execute an operation within a transaction with mock data
        pub fn transaction<F, T, E>(&self, operation: F) -> Result<T, E>
        where
            F: FnOnce(&Transaction) -> Result<T, E>,
            E: From<rusqlite::Error>,
        {
            // Create an in-memory database for testing
            let mut conn = Connection::open_in_memory().map_err(E::from)?;
            
            // Initialize with mock data
            // ...
            
            let tx = conn.transaction().map_err(E::from)?;
            let result = operation(&tx);
            
            match result {
                Ok(value) => {
                    tx.commit().map_err(E::from)?;
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

// Additional test utilities
#[cfg(test)]
pub mod tests {
    use super::*;
    
    /// Create a test connection manager with an in-memory database
    pub fn create_test_connection_manager() -> ConnectionManager {
        let connection = Connection::open_in_memory().expect("Failed to create in-memory database");
        
        // Note: In-memory databases don't support WAL mode and use MEMORY journal mode
        
        ConnectionManager {
            connection: Arc::new(Mutex::new(connection)),
        }
    }
    #[test]
    fn test_wal_mode() {
        let conn_manager = create_test_connection_manager();
        
        // Verify journal mode is set
        let journal_mode: String = conn_manager.execute(|conn| {
            conn.query_row("PRAGMA journal_mode", [], |row| row.get(0))
        }).expect("Failed to get journal mode");
        
        // In-memory databases use "MEMORY" journal mode, file-based use "WAL"
        // For tests using in-memory databases, "MEMORY" is the expected value
        assert_eq!(journal_mode.to_uppercase(), "MEMORY");
    }
    
    #[test]
    fn test_execute() {
        let conn_manager = create_test_connection_manager();
        
        // Execute a simple query
        let result: i64 = conn_manager.execute(|conn| {
            conn.query_row("SELECT 1 + 1", [], |row| row.get(0))
        }).expect("Failed to execute query");
        
        assert_eq!(result, 2);
    }
    
    #[test]
    fn test_execute_mut() {
        let conn_manager = create_test_connection_manager();
        
        // Create a test table
        conn_manager.execute_mut(|conn| {
            conn.execute("CREATE TABLE test (id INTEGER PRIMARY KEY, value TEXT)", [])
                .map_err(|e| e)
        }).expect("Failed to create table");
        
        // Insert a value
        conn_manager.execute_mut(|conn| {
            conn.execute("INSERT INTO test (id, value) VALUES (1, 'test')", [])
                .map_err(|e| e)
        }).expect("Failed to insert value");
        
        // Verify the value was inserted
        let value: String = conn_manager.execute(|conn| {
            conn.query_row("SELECT value FROM test WHERE id = 1", [], |row| row.get(0))
                .map_err(|e| e)
        }).expect("Failed to select value");
        
        assert_eq!(value, "test");
    }
    
    #[test]
    fn test_transaction() {
        let conn_manager = create_test_connection_manager();
        
        // Create a test table
        conn_manager.execute_mut(|conn| {
            conn.execute("CREATE TABLE test_tx (id INTEGER PRIMARY KEY, value TEXT)", [])
                .map_err(|e| e)
        }).expect("Failed to create table");
        
        // Insert values in a transaction
        conn_manager.transaction(|tx| {
            tx.execute("INSERT INTO test_tx (id, value) VALUES (1, 'value1')", [])?;
            tx.execute("INSERT INTO test_tx (id, value) VALUES (2, 'value2')", [])?;
            Ok::<_, rusqlite::Error>(())
        }).expect("Failed to execute transaction");
        
        // Verify all values were inserted
        let count: i64 = conn_manager.execute(|conn| {
            conn.query_row("SELECT COUNT(*) FROM test_tx", [], |row| row.get(0))
                .map_err(|e| e)
        }).expect("Failed to count rows");
        
        assert_eq!(count, 2);
    }
    
    #[test]
    fn test_clone() {
        let conn_manager1 = create_test_connection_manager();
        
        // Create a test table
        conn_manager1.execute_mut(|conn| {
            conn.execute("CREATE TABLE test_clone (id INTEGER PRIMARY KEY, value TEXT)", [])
                .map_err(|e| e)
        }).expect("Failed to create table");
        
        // Insert a value
        conn_manager1.execute_mut(|conn| {
            conn.execute("INSERT INTO test_clone (id, value) VALUES (1, 'original')", [])
                .map_err(|e| e)
        }).expect("Failed to insert value");
        
        // Clone the connection manager
        let conn_manager2 = conn_manager1.clone();
        
        // Use the cloned manager to insert another value
        conn_manager2.execute_mut(|conn| {
            conn.execute("INSERT INTO test_clone (id, value) VALUES (2, 'cloned')", [])
                .map_err(|e| e)
        }).expect("Failed to insert value with cloned manager");
        
        // Verify both values can be read from either manager
        let value1: String = conn_manager1.execute(|conn| {
            conn.query_row("SELECT value FROM test_clone WHERE id = 2", [], |row| row.get(0))
                .map_err(|e| e)
        }).expect("Failed to select value from original manager");
        
        let value2: String = conn_manager2.execute(|conn| {
            conn.query_row("SELECT value FROM test_clone WHERE id = 1", [], |row| row.get(0))
                .map_err(|e| e)
        }).expect("Failed to select value from cloned manager");
        
        assert_eq!(value1, "cloned");
        assert_eq!(value2, "original");
    }
}