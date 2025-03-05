# Database Connection Management Refactoring Guide

**Navigation:** [productContext](./productContext.md) | [activeContext](./activeContext.md) | [progress](./progress.md) | [decisionLog](./decisionLog.md) | [Memory Bank Index](./memory-bank-index.md)

## Overview

This document provides a comprehensive guide for refactoring the database connection management in the Implexa project. The current approach has several issues related to mutability, transaction handling, and testing. This guide outlines a system-level solution that addresses these issues while maintaining consistency with the project's architecture and coding standards.

## Current Architecture and Issues

### Current Implementation

The current database connection management approach uses direct references to `rusqlite::Connection` in manager structs:

```rust
// PartManager uses mutable connection
pub struct PartManager<'a> {
    connection: &'a mut Connection,
}

// RevisionManager uses immutable connection
pub struct RevisionManager<'a> {
    connection: &'a Connection,
}

// RelationshipManager uses immutable connection
pub struct RelationshipManager<'a> {
    connection: &'a Connection,
}

// PartManagementManager uses mutable connection and other managers
pub struct PartManagementManager<'a> {
    connection: &'a mut Connection,
    git_manager: &'a GitBackendManager,
    current_user: User,
}
```

### Issues Identified

1. **Inconsistent Mutability Requirements**:
   - `PartManager` requires `&'a mut Connection`
   - `RevisionManager` and `RelationshipManager` use `&'a Connection` (immutable)
   - This inconsistency causes type mismatches when passing transactions to methods

2. **Multiple Mutable Borrow Problems**:
   - In test code and `PartManagementManager`, multiple managers need to use the same connection
   - Rust's ownership rules prevent multiple mutable borrows of the same connection

3. **Transaction Handling Complexity**:
   - Methods like `get_next_part_id` in `PartManager` create transactions internally
   - This makes it difficult to compose operations within a single transaction

4. **Testing Challenges**:
   - Difficult to mock database connections for unit testing
   - Hard to isolate components due to shared connection dependencies

5. **Type Mismatches**:
   - Mismatches between `&Transaction` and `&mut Connection` when passing transactions to methods

## Proposed Solution

### 1. Connection Manager with Interior Mutability

Create a new `ConnectionManager` struct that uses interior mutability to provide controlled access to the database connection:

```rust
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
```

### 2. Update DatabaseManager to Use ConnectionManager

Modify the `DatabaseManager` to use the new `ConnectionManager`:

```rust
pub struct DatabaseManager {
    connection_manager: ConnectionManager,
    // Other fields...
}

impl DatabaseManager {
    pub fn new(db_path: &Path) -> DatabaseResult<Self> {
        let connection = Connection::open(db_path)?;
        Ok(Self {
            connection_manager: ConnectionManager::new(connection),
            // Initialize other fields...
        })
    }

    pub fn connection_manager(&self) -> &ConnectionManager {
        &self.connection_manager
    }

    // Remove the existing connection() method that returns &mut Connection
    // and replace with the above connection_manager() method
}
```

### 3. Refactor Manager Structs to Use ConnectionManager

Update all manager structs to use the `ConnectionManager` instead of direct connection references:

#### PartManager

```rust
pub struct PartManager<'a> {
    connection_manager: &'a ConnectionManager,
}

impl<'a> PartManager<'a> {
    pub fn new(connection_manager: &'a ConnectionManager) -> Self {
        Self { connection_manager }
    }

    // For compatibility with existing code that uses transactions directly
    pub fn new_with_transaction(transaction: &'a Transaction) -> Self {
        // This is a temporary solution during migration
        // It creates a PartManager that works with a specific transaction
        unimplemented!("Implement this method if needed for backward compatibility")
    }

    pub fn get_next_part_id(&self) -> DatabaseResult<i64> {
        self.connection_manager.transaction(|tx| {
            // Get the current next_value
            let next_id: i64 = tx.query_row(
                "SELECT next_value FROM PartSequence WHERE id = 1",
                [],
                |row| row.get(0),
            )?;
            
            // Increment the next_value
            tx.execute(
                "UPDATE PartSequence SET next_value = next_value + 1 WHERE id = 1",
                [],
            )?;
            
            Ok(next_id)
        }).map_err(DatabaseError::from)
    }

    pub fn create_part(&self, part: &Part) -> DatabaseResult<()> {
        self.connection_manager.execute_mut(|conn| {
            // Convert SystemTime to seconds since UNIX_EPOCH for SQLite
            let created_secs = part.created_date
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            
            let modified_secs = part.modified_date
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
                
            conn.execute(
                "INSERT INTO Parts (part_id, category, subcategory, name, description, created_date, modified_date)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    part.part_id,
                    part.category,
                    part.subcategory,
                    part.name,
                    part.description,
                    created_secs,
                    modified_secs,
                ],
            )?;
            Ok(())
        }).map_err(DatabaseError::from)
    }

    // Add a method for creating a part within an existing transaction
    pub fn create_part_in_transaction(&self, part: &Part, tx: &Transaction) -> DatabaseResult<()> {
        // Convert SystemTime to seconds since UNIX_EPOCH for SQLite
        let created_secs = part.created_date
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
        
        let modified_secs = part.modified_date
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
            
        tx.execute(
            "INSERT INTO Parts (part_id, category, subcategory, name, description, created_date, modified_date)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                part.part_id,
                part.category,
                part.subcategory,
                part.name,
                part.description,
                created_secs,
                modified_secs,
            ],
        )?;
        Ok(())
    }

    // Refactor other methods similarly...
}
```

#### RevisionManager

```rust
pub struct RevisionManager<'a> {
    connection_manager: &'a ConnectionManager,
}

impl<'a> RevisionManager<'a> {
    pub fn new(connection_manager: &'a ConnectionManager) -> Self {
        Self { connection_manager }
    }

    // For compatibility with existing code that uses transactions directly
    pub fn new_with_transaction(transaction: &'a Transaction) -> Self {
        // This is a temporary solution during migration
        unimplemented!("Implement this method if needed for backward compatibility")
    }

    pub fn create_revision(&self, revision: &Revision) -> DatabaseResult<i64> {
        self.connection_manager.execute(|conn| {
            // Convert SystemTime to seconds since UNIX_EPOCH for SQLite
            let created_secs = revision.created_date
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
                
            conn.execute(
                "INSERT INTO Revisions (part_id, version, status, created_date, created_by, commit_hash)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    revision.part_id,
                    revision.version,
                    revision.status.to_str(),
                    created_secs,
                    revision.created_by,
                    revision.commit_hash,
                ],
            )?;
            Ok(conn.last_insert_rowid())
        }).map_err(DatabaseError::from)
    }

    // Add a method for creating a revision within an existing transaction
    pub fn create_revision_in_transaction(&self, revision: &Revision, tx: &Transaction) -> DatabaseResult<i64> {
        // Convert SystemTime to seconds since UNIX_EPOCH for SQLite
        let created_secs = revision.created_date
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;
            
        tx.execute(
            "INSERT INTO Revisions (part_id, version, status, created_date, created_by, commit_hash)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                revision.part_id,
                revision.version,
                revision.status.to_str(),
                created_secs,
                revision.created_by,
                revision.commit_hash,
            ],
        )?;
        Ok(tx.last_insert_rowid())
    }

    // Refactor other methods similarly...
}
```

#### RelationshipManager

```rust
pub struct RelationshipManager<'a> {
    connection_manager: &'a ConnectionManager,
}

impl<'a> RelationshipManager<'a> {
    pub fn new(connection_manager: &'a ConnectionManager) -> Self {
        Self { connection_manager }
    }

    // Refactor methods similarly to the examples above...
}
```

#### PartManagementManager

```rust
pub struct PartManagementManager<'a> {
    connection_manager: &'a ConnectionManager,
    git_manager: &'a GitBackendManager,
    current_user: User,
}

impl<'a> PartManagementManager<'a> {
    pub fn new(
        connection_manager: &'a ConnectionManager,
        git_manager: &'a GitBackendManager,
        current_user: User,
    ) -> Self {
        Self {
            connection_manager,
            git_manager,
            current_user,
        }
    }

    pub fn create_part(
        &self,
        category: String,
        subcategory: String,
        name: String,
        description: Option<String>,
        repo_path: &Path,
    ) -> PartManagementResult<(Part, i64)> {
        // Check if the user has permission to create parts
        if !self.current_user.can_create_parts() {
            return Err(PartManagementError::PermissionDenied(
                "User does not have permission to create parts".to_string(),
            ));
        }
        
        // Use a transaction for the entire operation
        self.connection_manager.transaction(|tx| {
            // Create part managers for this transaction
            let part_manager = PartManager::new_with_transaction(tx);
            let revision_manager = RevisionManager::new_with_transaction(tx);
            
            // Create the part
            let part_id = part_manager.get_next_part_id_in_transaction(tx)?;
            let part = Part::new(
                part_id,
                category,
                subcategory,
                name,
                description,
            );
            
            part_manager.create_part_in_transaction(&part, tx)?;
            
            // Generate the display part number
            let display_part_number = part.display_part_number_in_transaction(tx)?;
            
            // Create a new revision in Draft state
            let revision = Revision::new(
                part.part_id.to_string(),
                "1".to_string(),
                RevisionStatus::Draft,
                self.current_user.username.clone(),
                None, // No commit hash yet
            );
            
            // Save the revision to the database
            let revision_id = revision_manager.create_revision_in_transaction(&revision, tx)?;
            
            // Create a feature branch for the part
            let branch_name = format!("part/{}/draft", display_part_number);
            // Open the repository first
            let repo = self.git_manager.open_repository(repo_path)?;
            self.git_manager.create_branch(&repo, &branch_name)?;
            self.git_manager.checkout_branch(&repo, &branch_name)?;
            
            Ok((part, revision_id))
        }).map_err(|e| e.into())
    }

    // Refactor other methods similarly...
}
```

### 4. Update Tests

Update the test code to use the new `ConnectionManager`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::DatabaseManager;
    use tempfile::tempdir;

    #[test]
    fn test_part_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create a part manager using the connection manager
        let part_manager = PartManager::new(db_manager.connection_manager());

        // Get the next part ID
        let part_id = part_manager.get_next_part_id().unwrap();

        // Create a new part
        let part = Part::new(
            part_id,
            "Electronic".to_string(),
            "Resistor".to_string(),
            "10K Resistor".to_string(),
            Some("1/4W 10K Ohm Resistor".to_string()),
        );

        // Save the part to the database
        part_manager.create_part(&part).unwrap();

        // Retrieve the part from the database
        let retrieved_part = part_manager.get_part(part_id).unwrap();

        // Check that the retrieved part matches the original
        assert_eq!(retrieved_part.part_id, part.part_id);
        assert_eq!(retrieved_part.category, part.category);
        assert_eq!(retrieved_part.subcategory, part.subcategory);
        assert_eq!(retrieved_part.name, part.name);
        assert_eq!(retrieved_part.description, part.description);
    }
}
```

### 5. Add Support for Mocking in Tests

Create a mock connection manager for testing:

```rust
#[cfg(test)]
pub mod test_utils {
    use super::*;
    use std::cell::RefCell;
    use rusqlite::{Connection, Transaction};
    use rusqlite::hooks::Action;
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
```

## Implementation Strategy

Follow these steps to implement the database connection management refactoring:

1. **Create the ConnectionManager**:
   - Add a new file `src/database/connection_manager.rs`
   - Implement the `ConnectionManager` struct with interior mutability
   - Add methods for executing operations and managing transactions

2. **Update DatabaseManager**:
   - Modify `src/database/schema.rs` to use the `ConnectionManager`
   - Replace the `connection()` method with `connection_manager()`
   - Update initialization code to create a `ConnectionManager`

3. **Refactor Managers One at a Time**:
   - Start with `PartManager` in `src/database/part.rs`
   - Update to use `ConnectionManager` instead of direct connection
   - Add transaction-specific methods for compatibility
   - Repeat for other managers (`RevisionManager`, `RelationshipManager`, etc.)

4. **Update PartManagementManager**:
   - Modify `src/database/part_management.rs` to use `ConnectionManager`
   - Update methods to use transactions consistently

5. **Update Tests**:
   - Modify test code to use the new `ConnectionManager`
   - Implement mock connection manager for testing

6. **Verify and Fix Issues**:
   - Run tests to verify the refactoring works correctly
   - Fix any issues that arise during testing

## Benefits of This Approach

1. **Eliminates Multiple Mutable Borrow Issues**: By using interior mutability, we avoid the multiple mutable borrow problem
2. **Consistent API Across Managers**: All managers use the same `ConnectionManager` interface
3. **Simplified Transaction Management**: Transactions are managed at the `ConnectionManager` level
4. **Improved Testability**: Easier to mock the database connection for unit tests
5. **Type Safety**: Eliminates type mismatches between `&Transaction` and `&mut Connection`
6. **Composability**: Makes it easier to compose operations across different managers

## Alignment with Project Standards

This solution aligns with the project's coding standards:
- Uses Rust's type system for safety
- Follows the error handling approach with custom error types
- Maintains clear separation of concerns
- Improves testability
- Follows the modular architecture pattern

## Related Files

- [Product Context](./productContext.md) - Project overview and high-level design
- [Decision Log](./decisionLog.md) - Key architectural decisions
- [Database Schema Design](./database-schema-design.md) - SQLite database schema design
- [Coding Standards](./coding-standards.md) - Code style and practices
- [Unit Testing Approach](./unit-testing-approach.md) - Testing philosophy and practices

## Related Decisions

- [DEC-018](./decisionLog.md#dec-018---database-connection-management-refactoring) - Database Connection Management Refactoring