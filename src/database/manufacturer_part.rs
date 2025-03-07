//! Manufacturer Part module for Implexa
//!
//! This module provides functionality for managing manufacturer parts in the database.

use rusqlite::{Transaction, params, Row, Result as SqliteResult};
use crate::database::schema::{DatabaseError, DatabaseResult};
use crate::database::connection_manager::ConnectionManager;

/// Status of a manufacturer part
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ManufacturerPartStatus {
    /// Active status - can be used
    Active,
    /// Preferred status - recommended for use
    Preferred,
    /// Alternate status - can be used as an alternative
    Alternate,
    /// Obsolete status - should not be used for new designs
    Obsolete,
}

impl ManufacturerPartStatus {
    /// Convert a string to a ManufacturerPartStatus
    ///
    /// # Arguments
    ///
    /// * `status` - The status string
    ///
    /// # Returns
    ///
    /// The corresponding ManufacturerPartStatus
    pub fn from_str(status: &str) -> Option<Self> {
        match status {
            "Active" => Some(Self::Active),
            "Preferred" => Some(Self::Preferred),
            "Alternate" => Some(Self::Alternate),
            "Obsolete" => Some(Self::Obsolete),
            _ => None,
        }
    }

    /// Convert a ManufacturerPartStatus to a string
    ///
    /// # Returns
    ///
    /// The string representation of the status
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Active => "Active",
            Self::Preferred => "Preferred",
            Self::Alternate => "Alternate",
            Self::Obsolete => "Obsolete",
        }
    }
}

/// Represents a manufacturer part
#[derive(Debug, Clone)]
pub struct ManufacturerPart {
    /// Unique identifier for the manufacturer part
    pub mpn_id: Option<i64>,
    /// ID of the part this manufacturer part is associated with
    pub part_id: i64,
    /// Manufacturer name
    pub manufacturer: String,
    /// Manufacturer part number
    pub mpn: String,
    /// Description of the manufacturer part
    pub description: Option<String>,
    /// Status of the manufacturer part
    pub status: ManufacturerPartStatus,
}

impl ManufacturerPart {
    /// Create a new manufacturer part
    ///
    /// # Arguments
    ///
    /// * `part_id` - ID of the part this manufacturer part is associated with
    /// * `manufacturer` - Manufacturer name
    /// * `mpn` - Manufacturer part number
    /// * `description` - Description of the manufacturer part
    /// * `status` - Status of the manufacturer part
    ///
    /// # Returns
    ///
    /// A new ManufacturerPart instance
    pub fn new(
        part_id: i64,
        manufacturer: String,
        mpn: String,
        description: Option<String>,
        status: ManufacturerPartStatus,
    ) -> Self {
        Self {
            mpn_id: None,
            part_id,
            manufacturer,
            mpn,
            description,
            status,
        }
    }
}

/// Manager for manufacturer part operations
pub struct ManufacturerPartManager<'a> {
    /// Connection manager for the SQLite database
    connection_manager: &'a ConnectionManager,
}

impl<'a> ManufacturerPartManager<'a> {
    /// Create a new ManufacturerPartManager
    ///
    /// # Arguments
    ///
    /// * `connection_manager` - Connection manager for the SQLite database
    ///
    /// # Returns
    ///
    /// A new ManufacturerPartManager instance
    pub fn new(connection_manager: &'a ConnectionManager) -> Self {
        Self { connection_manager }
    }
    
    /// Create a new ManufacturerPartManager with a transaction
    ///
    /// # Arguments
    ///
    /// * `transaction` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// A new ManufacturerPartManager instance
    ///
    /// # Note
    ///
    /// This is a temporary method for backward compatibility during migration
    pub fn new_with_transaction(_transaction: &'a Transaction) -> Self {
        unimplemented!("This method is a placeholder for backward compatibility during migration")
    }

    /// Create a new manufacturer part in the database
    ///
    /// # Arguments
    ///
    /// * `manufacturer_part` - The manufacturer part to create
    ///
    /// # Returns
    ///
    /// The ID of the newly created manufacturer part
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the manufacturer part could not be created
    pub fn create_manufacturer_part(&self, manufacturer_part: &ManufacturerPart) -> DatabaseResult<i64> {
        self.connection_manager.execute_mut::<_, _, DatabaseError>(|conn| {
            conn.execute(
                "INSERT INTO ManufacturerParts (part_id, manufacturer, mpn, description, status)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    manufacturer_part.part_id,
                    manufacturer_part.manufacturer,
                    manufacturer_part.mpn,
                    manufacturer_part.description,
                    manufacturer_part.status.to_str(),
                ],
            )?;
            Ok::<i64, DatabaseError>(conn.last_insert_rowid())
        }).map_err(DatabaseError::from)
    }
    
    /// Create a new manufacturer part in the database within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `manufacturer_part` - The manufacturer part to create
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// The ID of the newly created manufacturer part
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the manufacturer part could not be created
    pub fn create_manufacturer_part_in_transaction(&self, manufacturer_part: &ManufacturerPart, tx: &Transaction) -> DatabaseResult<i64> {
        tx.execute(
            "INSERT INTO ManufacturerParts (part_id, manufacturer, mpn, description, status)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                manufacturer_part.part_id,
                manufacturer_part.manufacturer,
                manufacturer_part.mpn,
                manufacturer_part.description,
                manufacturer_part.status.to_str(),
            ],
        )?;
        Ok(tx.last_insert_rowid())
    }

    /// Get a manufacturer part by its ID
    ///
    /// # Arguments
    ///
    /// * `mpn_id` - The ID of the manufacturer part to get
    ///
    /// # Returns
    ///
    /// The manufacturer part with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the manufacturer part could not be found
    pub fn get_manufacturer_part(&self, mpn_id: i64) -> DatabaseResult<ManufacturerPart> {
        self.connection_manager.execute::<_, _, DatabaseError>(|conn| {
            let manufacturer_part = conn.query_row(
                "SELECT mpn_id, part_id, manufacturer, mpn, description, status
                 FROM ManufacturerParts
                 WHERE mpn_id = ?1",
                params![mpn_id],
                |row| self.row_to_manufacturer_part(row),
            )?;
            Ok::<ManufacturerPart, DatabaseError>(manufacturer_part)
        }).map_err(DatabaseError::from)
    }

    /// Get all manufacturer parts for a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part
    ///
    /// # Returns
    ///
    /// A vector of manufacturer parts for the specified part
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the manufacturer parts could not be retrieved
    pub fn get_manufacturer_parts_for_part(&self, part_id: &str) -> DatabaseResult<Vec<ManufacturerPart>> {
        self.connection_manager.execute::<_, _, DatabaseError>(|conn| {
            let mut stmt = conn.prepare(
                "SELECT mpn_id, part_id, manufacturer, mpn, description, status
                 FROM ManufacturerParts
                 WHERE part_id = ?1
                 ORDER BY manufacturer, mpn",
            )?;
            let manufacturer_parts_iter = stmt.query_map(params![part_id], |row| self.row_to_manufacturer_part(row))?;
            let mut manufacturer_parts = Vec::new();
            for manufacturer_part_result in manufacturer_parts_iter {
                manufacturer_parts.push(manufacturer_part_result?);
            }
            Ok::<Vec<ManufacturerPart>, DatabaseError>(manufacturer_parts)
        }).map_err(DatabaseError::from)
    }

    /// Get manufacturer parts by manufacturer and MPN
    ///
    /// # Arguments
    ///
    /// * `manufacturer` - The manufacturer name
    /// * `mpn` - The manufacturer part number
    ///
    /// # Returns
    ///
    /// A vector of manufacturer parts matching the specified manufacturer and MPN
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the manufacturer parts could not be retrieved
    pub fn get_manufacturer_parts_by_mpn(&self, manufacturer: &str, mpn: &str) -> DatabaseResult<Vec<ManufacturerPart>> {
        self.connection_manager.execute::<_, _, DatabaseError>(|conn| {
            let mut stmt = conn.prepare(
                "SELECT mpn_id, part_id, manufacturer, mpn, description, status
                 FROM ManufacturerParts
                 WHERE manufacturer = ?1 AND mpn = ?2",
            )?;
            let manufacturer_parts_iter = stmt.query_map(params![manufacturer, mpn], |row| self.row_to_manufacturer_part(row))?;
            let mut manufacturer_parts = Vec::new();
            for manufacturer_part_result in manufacturer_parts_iter {
                manufacturer_parts.push(manufacturer_part_result?);
            }
            Ok::<Vec<ManufacturerPart>, DatabaseError>(manufacturer_parts)
        }).map_err(DatabaseError::from)
    }

    /// Update a manufacturer part
    ///
    /// # Arguments
    ///
    /// * `manufacturer_part` - The manufacturer part to update
    ///
    /// # Returns
    ///
    /// Ok(()) if the manufacturer part was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the manufacturer part could not be updated
    pub fn update_manufacturer_part(&self, manufacturer_part: &ManufacturerPart) -> DatabaseResult<()> {
        let mpn_id = manufacturer_part.mpn_id.ok_or_else(|| {
            DatabaseError::InitializationError("Manufacturer Part ID is required for update".to_string())
        })?;

        self.connection_manager.execute_mut::<_, _, DatabaseError>(|conn| {
            conn.execute(
                "UPDATE ManufacturerParts
                 SET part_id = ?2, manufacturer = ?3, mpn = ?4, description = ?5, status = ?6
                 WHERE mpn_id = ?1",
                params![
                    mpn_id,
                    manufacturer_part.part_id,
                    manufacturer_part.manufacturer,
                    manufacturer_part.mpn,
                    manufacturer_part.description,
                    manufacturer_part.status.to_str(),
                ],
            )?;
            Ok::<(), DatabaseError>(())
        }).map_err(DatabaseError::from)
    }
    
    /// Update a manufacturer part within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `manufacturer_part` - The manufacturer part to update
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// Ok(()) if the manufacturer part was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the manufacturer part could not be updated
    pub fn update_manufacturer_part_in_transaction(&self, manufacturer_part: &ManufacturerPart, tx: &Transaction) -> DatabaseResult<()> {
        let mpn_id = manufacturer_part.mpn_id.ok_or_else(|| {
            DatabaseError::InitializationError("Manufacturer Part ID is required for update".to_string())
        })?;

        tx.execute(
            "UPDATE ManufacturerParts
             SET part_id = ?2, manufacturer = ?3, mpn = ?4, description = ?5, status = ?6
             WHERE mpn_id = ?1",
            params![
                mpn_id,
                manufacturer_part.part_id,
                manufacturer_part.manufacturer,
                manufacturer_part.mpn,
                manufacturer_part.description,
                manufacturer_part.status.to_str(),
            ],
        )?;
        Ok(())
    }

    /// Delete a manufacturer part
    ///
    /// # Arguments
    ///
    /// * `mpn_id` - The ID of the manufacturer part to delete
    ///
    /// # Returns
    ///
    /// Ok(()) if the manufacturer part was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the manufacturer part could not be deleted
    pub fn delete_manufacturer_part(&self, mpn_id: i64) -> DatabaseResult<()> {
        self.connection_manager.execute_mut::<_, _, DatabaseError>(|conn| {
            conn.execute(
                "DELETE FROM ManufacturerParts WHERE mpn_id = ?1",
                params![mpn_id],
            )?;
            Ok::<(), DatabaseError>(())
        }).map_err(DatabaseError::from)
    }
    
    /// Delete a manufacturer part within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `mpn_id` - The ID of the manufacturer part to delete
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// Ok(()) if the manufacturer part was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the manufacturer part could not be deleted
    pub fn delete_manufacturer_part_in_transaction(&self, mpn_id: i64, tx: &Transaction) -> DatabaseResult<()> {
        tx.execute(
            "DELETE FROM ManufacturerParts WHERE mpn_id = ?1",
            params![mpn_id],
        )?;
        Ok(())
    }

    /// Search for manufacturer parts by manufacturer or MPN
    ///
    /// # Arguments
    ///
    /// * `search_term` - The search term
    ///
    /// # Returns
    ///
    /// A vector of manufacturer parts matching the search term
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the search could not be performed
    pub fn search_manufacturer_parts(&self, search_term: &str) -> DatabaseResult<Vec<ManufacturerPart>> {
        self.connection_manager.execute::<_, _, DatabaseError>(|conn| {
            let search_pattern = format!("%{}%", search_term);
            let mut stmt = conn.prepare(
                "SELECT mpn_id, part_id, manufacturer, mpn, description, status
                 FROM ManufacturerParts
                 WHERE manufacturer LIKE ?1 OR mpn LIKE ?1
                 ORDER BY manufacturer, mpn",
            )?;
            let manufacturer_parts_iter = stmt.query_map(params![search_pattern], |row| self.row_to_manufacturer_part(row))?;
            let mut manufacturer_parts = Vec::new();
            for manufacturer_part_result in manufacturer_parts_iter {
                manufacturer_parts.push(manufacturer_part_result?);
            }
            Ok::<Vec<ManufacturerPart>, DatabaseError>(manufacturer_parts)
        }).map_err(DatabaseError::from)
    }

    /// Convert a database row to a ManufacturerPart
    ///
    /// # Arguments
    ///
    /// * `row` - The database row
    ///
    /// # Returns
    ///
    /// A ManufacturerPart instance
    ///
    /// # Errors
    ///
    /// Returns a SqliteError if the row could not be converted
    fn row_to_manufacturer_part(&self, row: &Row) -> SqliteResult<ManufacturerPart> {
        let status_str: String = row.get(5)?;
        let status = ManufacturerPartStatus::from_str(&status_str)
            .unwrap_or(ManufacturerPartStatus::Active);

        Ok(ManufacturerPart {
            mpn_id: Some(row.get(0)?),
            part_id: row.get(1)?,
            manufacturer: row.get(2)?,
            mpn: row.get(3)?,
            description: row.get(4)?,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::DatabaseManager;
    use crate::database::part::{Part, PartManager};
    use tempfile::tempdir;

    #[test]
    fn test_manufacturer_part_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create a part manager and a manufacturer part manager
        let part_manager = PartManager::new(db_manager.connection_manager());
        let manufacturer_part_manager = ManufacturerPartManager::new(db_manager.connection_manager());

        // Create a new part
        let part = Part::new(
            10001,
            "Electronic".to_string(),
            "Resistor".to_string(),
            "10K Resistor".to_string(),
            Some("1/4W 10K Ohm Resistor".to_string()),
        );

        // Save the part to the database
        part_manager.create_part(&part).unwrap();

        // Create a new manufacturer part
        let manufacturer_part = ManufacturerPart::new(
            10001, // Use the same part_id as the part we created above
            "Yageo".to_string(),
            "RC0603FR-0710KL".to_string(),
            Some("10K Ohm ±1% 0.1W, 1/10W Chip Resistor 0603 (1608 Metric)".to_string()),
            ManufacturerPartStatus::Preferred,
        );

        // Save the manufacturer part to the database
        let mpn_id = manufacturer_part_manager.create_manufacturer_part(&manufacturer_part).unwrap();

        // Retrieve the manufacturer part from the database
        let retrieved_manufacturer_part = manufacturer_part_manager.get_manufacturer_part(mpn_id).unwrap();

        // Check that the retrieved manufacturer part matches the original
        assert_eq!(retrieved_manufacturer_part.part_id, manufacturer_part.part_id);
        assert_eq!(retrieved_manufacturer_part.manufacturer, manufacturer_part.manufacturer);
        assert_eq!(retrieved_manufacturer_part.mpn, manufacturer_part.mpn);
        assert_eq!(retrieved_manufacturer_part.description, manufacturer_part.description);
        assert_eq!(retrieved_manufacturer_part.status.to_str(), manufacturer_part.status.to_str());
    }

    #[test]
    fn test_manufacturer_part_status_conversion() {
        assert_eq!(ManufacturerPartStatus::from_str("Active"), Some(ManufacturerPartStatus::Active));
        assert_eq!(ManufacturerPartStatus::from_str("Preferred"), Some(ManufacturerPartStatus::Preferred));
        assert_eq!(ManufacturerPartStatus::from_str("Alternate"), Some(ManufacturerPartStatus::Alternate));
        assert_eq!(ManufacturerPartStatus::from_str("Obsolete"), Some(ManufacturerPartStatus::Obsolete));
        assert_eq!(ManufacturerPartStatus::from_str("Invalid"), None);

        assert_eq!(ManufacturerPartStatus::Active.to_str(), "Active");
        assert_eq!(ManufacturerPartStatus::Preferred.to_str(), "Preferred");
        assert_eq!(ManufacturerPartStatus::Alternate.to_str(), "Alternate");
        assert_eq!(ManufacturerPartStatus::Obsolete.to_str(), "Obsolete");
    }
}