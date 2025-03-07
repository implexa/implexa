//! Part module for Implexa
//!
//! This module provides functionality for managing parts in the database.

use rusqlite::{Transaction, params, Row, Result as SqliteResult};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::database::schema::{DatabaseResult, DatabaseError};
use crate::database::connection_manager::ConnectionManager;

/// Represents a part in the system
#[derive(Debug, Clone)]
pub struct Part {
    /// Unique identifier for the part (sequential number)
    pub part_id: i64,
    /// Category of the part
    pub category: String,
    /// Subcategory of the part
    pub subcategory: String,
    /// Name of the part
    pub name: String,
    /// Description of the part
    pub description: Option<String>,
    /// Date the part was created
    pub created_date: SystemTime,
    /// Date the part was last modified
    pub modified_date: SystemTime,
}

impl Part {
    /// Create a new part
    ///
    /// # Arguments
    ///
    /// * `part_id` - Unique identifier for the part (sequential number)
    /// * `category` - Category of the part
    /// * `subcategory` - Subcategory of the part
    /// * `name` - Name of the part
    /// * `description` - Description of the part
    ///
    /// # Returns
    ///
    /// A new Part instance
    pub fn new(
        part_id: i64,
        category: String,
        subcategory: String,
        name: String,
        description: Option<String>,
    ) -> Self {
        let now = SystemTime::now();
        Self {
            part_id,
            category,
            subcategory,
            name,
            description,
            created_date: now,
            modified_date: now,
        }
    }

    /// Generate a display part number based on category, subcategory, and a sequential number
    ///
    /// # Arguments
    ///
    /// * `connection` - Database connection to look up category/subcategory codes
    /// * `category` - Category of the part
    /// * `subcategory` - Subcategory of the part
    /// * `sequence` - Sequential number for the part
    ///
    /// # Returns
    ///
    /// A display part number in the format "CAT-SUB-SEQUENCE"
    pub fn generate_display_part_number(
        connection: &rusqlite::Connection,
        category: &str,
        subcategory: &str,
        sequence: i64
    ) -> String {
        // Try to look up the category code in the database
        let category_code: String = match connection.query_row(
            "SELECT code FROM Categories WHERE name = ?1",
            params![category],
            |row| row.get(0),
        ) {
            Ok(code) => code,
            Err(_) => category.chars().take(2).collect::<String>().to_uppercase(),
        };
        
        // Try to look up the subcategory code in the database
        let subcategory_code: String = match connection.query_row(
            "SELECT s.code FROM Subcategories s
             JOIN Categories c ON s.category_id = c.category_id
             WHERE c.name = ?1 AND s.name = ?2",
            params![category, subcategory],
            |row| row.get(0),
        ) {
            Ok(code) => code,
            Err(_) => subcategory.chars().take(3).collect::<String>().to_uppercase(),
        };
        
        format!("{}-{}-{}", category_code, subcategory_code, sequence)
    }

    /// Generate a display part number using a ConnectionManager
    ///
    /// # Arguments
    ///
    /// * `conn_mgr` - Connection manager
    /// * `category` - Category of the part
    /// * `subcategory` - Subcategory of the part
    /// * `sequence` - Sequential number for the part
    ///
    /// # Returns
    ///
    /// A display part number in the format "CAT-SUB-SEQUENCE"
    pub fn generate_display_part_number_with_manager(
        conn_mgr: &ConnectionManager,
        category: &str,
        subcategory: &str,
        sequence: i64
    ) -> String {
        conn_mgr.execute::<_, String, rusqlite::Error>(|conn| {
            Ok(Self::generate_display_part_number(conn, category, subcategory, sequence))
        }).unwrap_or_else(|_: rusqlite::Error| {
            format!("{}-{}-{}",
                category.chars().take(2).collect::<String>().to_uppercase(),
                subcategory.chars().take(3).collect::<String>().to_uppercase(),
                sequence
            )
        })
    }
/// Get the display part number for this part
///
/// # Arguments
///
/// * `connection` - Database connection to look up category/subcategory codes
///
/// # Returns
///
/// The display part number in the format "CAT-SUB-SEQUENCE"
pub fn display_part_number(&self, conn_mgr: &ConnectionManager) -> String {
    Self::generate_display_part_number_with_manager(conn_mgr, &self.category, &self.subcategory, self.part_id)
}

/// Get a display part number for use with a transaction
///
/// # Arguments
///
/// * `tx` - Database transaction
///
/// # Returns
///
/// The display part number in the format "CAT-SUB-SEQUENCE"
pub fn display_part_number_in_transaction(&self, tx: &Transaction) -> String {
    Self::generate_display_part_number(tx, &self.category, &self.subcategory, self.part_id)
}
}

/// Manager for part operations
pub struct PartManager<'a> {
    /// Connection manager for the SQLite database
    connection_manager: &'a ConnectionManager,
}

impl<'a> PartManager<'a> {
    /// Create a new PartManager
    ///
    /// # Arguments
    ///
    /// * `connection_manager` - Connection manager for the SQLite database
    ///
    /// # Returns
    ///
    /// A new PartManager instance
    pub fn new(connection_manager: &'a ConnectionManager) -> Self {
        Self { connection_manager }
    }
    
    /// Create a new PartManager with a transaction
    ///
    /// # Arguments
    ///
    /// * `transaction` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// A new PartManager instance
    ///
    /// # Note
    ///
    /// This is a temporary method for backward compatibility during migration
    pub fn new_with_transaction(_transaction: &'a Transaction) -> Self {
        unimplemented!("This method is a placeholder for backward compatibility during migration")
    }
    /// Get the next part ID from the sequence
    ///
    /// # Returns
    ///
    /// The next part ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the next part ID could not be retrieved
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
            
            Ok::<i64, rusqlite::Error>(next_id)
        }).map_err(DatabaseError::from)
    }
    
    /// Get the next part ID from the sequence within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// The next part ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the next part ID could not be retrieved
    pub fn get_next_part_id_in_transaction(&self, tx: &Transaction) -> DatabaseResult<i64> {
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
    }

    /// Create a new part in the database with an automatically assigned part_id
    ///
    /// # Arguments
    ///
    /// * `category` - Category of the part
    /// * `subcategory` - Subcategory of the part
    /// * `name` - Name of the part
    /// * `description` - Description of the part
    ///
    /// # Returns
    ///
    /// The newly created part
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the part could not be created
    pub fn create_new_part(
        &self,
        category: String,
        subcategory: String,
        name: String,
        description: Option<String>,
    ) -> DatabaseResult<Part> {
        // Get the next part ID
        let part_id = self.get_next_part_id()?;
        
        // Create the part
        let part = Part::new(
            part_id,
            category,
            subcategory,
            name,
            description,
        );
        
        // Save the part to the database
        self.create_part(&part)?;
        
        Ok(part)
    }

    /// Create a part in the database with a specific part_id
    ///
    /// # Arguments
    ///
    /// * `part` - The part to create
    ///
    /// # Returns
    ///
    /// Ok(()) if the part was successfully created
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the part could not be created
    pub fn create_part(&self, part: &Part) -> DatabaseResult<()> {
        self.connection_manager.execute_mut::<_, _, DatabaseError>(|conn| {
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
            Ok::<(), DatabaseError>(())
        }).map_err(DatabaseError::from)
    }
    
    /// Create a part in the database with a specific part_id within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `part` - The part to create
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// Ok(()) if the part was successfully created
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the part could not be created
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

    /// Get a part by its ID
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part to get
    ///
    /// # Returns
    ///
    /// The part with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the part could not be found
    pub fn get_part(&self, part_id: i64) -> DatabaseResult<Part> {
        self.connection_manager.execute::<_, _, DatabaseError>(|conn| {
            let part = conn.query_row(
                "SELECT part_id, category, subcategory, name, description, created_date, modified_date
                 FROM Parts
                 WHERE part_id = ?1",
                params![part_id],
                |row| self.row_to_part(row),
            )?;
            Ok::<Part, DatabaseError>(part)
        }).map_err(DatabaseError::from)
    }

    /// Get a part by its ID within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part to get
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// The part with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the part could not be found
    pub fn get_part_in_transaction(&self, part_id: i64, tx: &Transaction) -> DatabaseResult<Part> {
        let part = tx.query_row(
            "SELECT part_id, category, subcategory, name, description, created_date, modified_date
             FROM Parts
             WHERE part_id = ?1",
            params![part_id],
            |row| self.row_to_part(row),
        )?;
        Ok(part)
    }

    /// Get all parts
    ///
    /// # Returns
    ///
    /// A vector of all parts
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the parts could not be retrieved
    pub fn get_all_parts(&self) -> DatabaseResult<Vec<Part>> {
        self.connection_manager.execute::<_, _, DatabaseError>(|conn| {
            let mut stmt = conn.prepare(
                "SELECT part_id, category, subcategory, name, description, created_date, modified_date
                 FROM Parts
                 ORDER BY category, subcategory, name",
            )?;
            let parts_iter = stmt.query_map([], |row| self.row_to_part(row))?;
            let mut parts = Vec::new();
            for part_result in parts_iter {
                parts.push(part_result?);
            }
            Ok::<Vec<Part>, DatabaseError>(parts)
        }).map_err(DatabaseError::from)
    }

    /// Get parts by category
    ///
    /// # Arguments
    ///
    /// * `category` - The category to filter by
    ///
    /// # Returns
    ///
    /// A vector of parts in the specified category
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the parts could not be retrieved
    pub fn get_parts_by_category(&self, category: &str) -> DatabaseResult<Vec<Part>> {
        self.connection_manager.execute::<_, _, DatabaseError>(|conn| {
            let mut stmt = conn.prepare(
                "SELECT part_id, category, subcategory, name, description, created_date, modified_date
                 FROM Parts
                 WHERE category = ?1
                 ORDER BY subcategory, name",
            )?;
            let parts_iter = stmt.query_map(params![category], |row| self.row_to_part(row))?;
            let mut parts = Vec::new();
            for part_result in parts_iter {
                parts.push(part_result?);
            }
            Ok::<Vec<Part>, DatabaseError>(parts)
        }).map_err(DatabaseError::from)
    }

    /// Update a part
    ///
    /// # Arguments
    ///
    /// * `part` - The part to update
    ///
    /// # Returns
    ///
    /// Ok(()) if the part was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the part could not be updated
    pub fn update_part(&self, part: &Part) -> DatabaseResult<()> {
        self.connection_manager.execute_mut::<_, _, DatabaseError>(|conn| {
            // Convert SystemTime to seconds since UNIX_EPOCH for SQLite
            let modified_secs = part.modified_date
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
                
            conn.execute(
                "UPDATE Parts
                 SET category = ?2, subcategory = ?3, name = ?4, description = ?5, modified_date = ?6
                 WHERE part_id = ?1",
                params![
                    part.part_id,
                    part.category,
                    part.subcategory,
                    part.name,
                    part.description,
                    modified_secs,
                ],
            )?;
            Ok::<(), DatabaseError>(())
        }).map_err(DatabaseError::from)
    }

    /// Delete a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part to delete
    ///
    /// # Returns
    ///
    /// Ok(()) if the part was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the part could not be deleted
    pub fn delete_part(&self, part_id: i64) -> DatabaseResult<()> {
        self.connection_manager.execute_mut::<_, _, DatabaseError>(|conn| {
            conn.execute(
                "DELETE FROM Parts WHERE part_id = ?1",
                params![part_id],
            )?;
            Ok::<(), DatabaseError>(())
        }).map_err(DatabaseError::from)
    }

    /// Get parts by display part number
    ///
    /// # Arguments
    ///
    /// * `display_part_number` - The display part number to search for
    ///
    /// # Returns
    ///
    /// A vector of parts matching the display part number pattern
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the parts could not be retrieved
    pub fn get_parts_by_display_number(&self, display_part_number: &str) -> DatabaseResult<Vec<Part>> {
        // Parse the display part number to extract category and subcategory codes
        let parts: Vec<&str> = display_part_number.split('-').collect();
        if parts.len() < 2 {
            return Ok(Vec::new()); // Invalid format, return empty vector
        }
        
        let category_code = parts[0];
        let subcategory_code = parts[1];
        
        self.connection_manager.execute::<_, _, DatabaseError>(|conn| {
            // Find parts with matching category and subcategory codes
            let mut stmt = conn.prepare(
                "SELECT p.part_id, p.category, p.subcategory, p.name, p.description, p.created_date, p.modified_date
                 FROM Parts p
                 JOIN Categories c ON UPPER(p.category) = UPPER(c.name)
                 JOIN Subcategories s ON UPPER(p.subcategory) = UPPER(s.name) AND s.category_id = c.category_id
                 WHERE c.code = ?1 AND s.code = ?2",
            )?;
            
            let parts_iter = stmt.query_map(params![category_code, subcategory_code], |row| self.row_to_part(row))?;
            let mut parts = Vec::new();
            for part_result in parts_iter {
                parts.push(part_result?);
            }
            Ok::<Vec<Part>, DatabaseError>(parts)
        }).map_err(DatabaseError::from)
    }

    /// Convert a database row to a Part
    ///
    /// # Arguments
    ///
    /// * `row` - The database row
    ///
    /// # Returns
    ///
    /// A Part instance
    ///
    /// # Errors
    ///
    /// Returns a SqliteError if the row could not be converted
    fn row_to_part(&self, row: &Row) -> SqliteResult<Part> {
        // Convert SQLite timestamps (seconds since UNIX_EPOCH) to SystemTime
        let created_secs: i64 = row.get(5)?;
        let created_date = UNIX_EPOCH + std::time::Duration::from_secs(created_secs as u64);
        
        let modified_secs: i64 = row.get(6)?;
        let modified_date = UNIX_EPOCH + std::time::Duration::from_secs(modified_secs as u64);
        
        Ok(Part {
            part_id: row.get(0)?,
            category: row.get(1)?,
            subcategory: row.get(2)?,
            name: row.get(3)?,
            description: row.get(4)?,
            created_date,
            modified_date,
        })
    }
}

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

        // Create a part manager
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

        // Check the display part number format
        let conn_mgr = db_manager.connection_manager();
        let display_number = part.display_part_number(conn_mgr);
        assert!(display_number.starts_with("EL-RES-"));
        assert!(display_number.contains(&part_id.to_string()));
    }

    #[test]
    fn test_display_part_number_generation() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Test with database-defined categories
        let conn_mgr = db_manager.connection_manager();
        let display_number = Part::generate_display_part_number_with_manager(
            conn_mgr,
            "Electronic",
            "Resistor",
            10001
        );
        assert_eq!(display_number, "EL-RES-10001");

        // Test with custom category/subcategory
        let display_number = Part::generate_display_part_number_with_manager(
            conn_mgr,
            "Custom Category",
            "Custom Subcategory",
            10042
        );
        assert_eq!(display_number, "CU-CUS-10042");
    }
}