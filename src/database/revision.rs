//! Revision module for Implexa
//!
//! This module provides functionality for managing part revisions in the database.

use rusqlite::{Connection, Transaction, params, Row, Result as SqliteResult, OptionalExtension};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::database::schema::{DatabaseResult, DatabaseError};
use crate::database::connection_manager::ConnectionManager;

/// Status of a revision
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RevisionStatus {
    /// Draft status - initial state, can be edited
    Draft,
    /// In Review status - under review, limited edits
    InReview,
    /// Released status - approved and released, no edits
    Released,
    /// Obsolete status - no longer active
    Obsolete,
}

impl RevisionStatus {
    /// Convert a string to a RevisionStatus
    ///
    /// # Arguments
    ///
    /// * `status` - The status string
    ///
    /// # Returns
    ///
    /// The corresponding RevisionStatus
    pub fn from_str(status: &str) -> Option<Self> {
        match status {
            "Draft" => Some(Self::Draft),
            "In Review" => Some(Self::InReview),
            "Released" => Some(Self::Released),
            "Obsolete" => Some(Self::Obsolete),
            _ => None,
        }
    }

    /// Convert a RevisionStatus to a string
    ///
    /// # Returns
    ///
    /// The string representation of the status
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::InReview => "In Review",
            Self::Released => "Released",
            Self::Obsolete => "Obsolete",
        }
    }
}

/// Represents a revision of a part
#[derive(Debug, Clone)]
pub struct Revision {
    /// Unique identifier for the revision
    pub revision_id: Option<i64>,
    /// ID of the part this revision belongs to
    pub part_id: String,
    /// Version of the revision
    pub version: String,
    /// Status of the revision
    pub status: RevisionStatus,
    /// Date the revision was created
    pub created_date: SystemTime,
    /// User who created the revision
    pub created_by: String,
    /// Git commit hash associated with this revision
    pub commit_hash: Option<String>,
}

impl Revision {
    /// Create a new revision
    ///
    /// # Arguments
    ///
    /// * `part_id` - ID of the part this revision belongs to
    /// * `version` - Version of the revision
    /// * `status` - Status of the revision
    /// * `created_by` - User who created the revision
    /// * `commit_hash` - Git commit hash associated with this revision
    ///
    /// # Returns
    ///
    /// A new Revision instance
    pub fn new(
        part_id: String,
        version: String,
        status: RevisionStatus,
        created_by: String,
        commit_hash: Option<String>,
    ) -> Self {
        Self {
            revision_id: None,
            part_id,
            version,
            status,
            created_date: SystemTime::now(),
            created_by,
            commit_hash,
        }
    }
}

/// Manager for revision operations
pub struct RevisionManager<'a> {
    /// Connection manager for the SQLite database
    connection_manager: &'a ConnectionManager,
}

impl<'a> RevisionManager<'a> {
    /// Create a new RevisionManager
    ///
    /// # Arguments
    ///
    /// * `connection_manager` - Connection manager for the SQLite database
    ///
    /// # Returns
    ///
    /// A new RevisionManager instance
    pub fn new(connection_manager: &'a ConnectionManager) -> Self {
        Self { connection_manager }
    }
    
    /// Create a new RevisionManager with a transaction
    ///
    /// # Arguments
    ///
    /// * `transaction` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// A new RevisionManager instance
    ///
    /// # Note
    ///
    /// This is a temporary method for backward compatibility during migration
    pub fn new_with_transaction(_transaction: &'a Transaction) -> Self {
        unimplemented!("This method is a placeholder for backward compatibility during migration")
    }

    /// Create a new revision in the database
    ///
    /// # Arguments
    ///
    /// * `revision` - The revision to create
    ///
    /// # Returns
    ///
    /// The ID of the newly created revision
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the revision could not be created
    pub fn create_revision(&self, revision: &Revision) -> DatabaseResult<i64> {
        self.connection_manager.execute_mut(|conn| {
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
    
    /// Create a new revision in the database within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `revision` - The revision to create
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// The ID of the newly created revision
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the revision could not be created
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

    /// Get a revision by its ID
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision to get
    ///
    /// # Returns
    ///
    /// The revision with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the revision could not be found
    pub fn get_revision(&self, revision_id: i64) -> DatabaseResult<Revision> {
        self.connection_manager.execute(|conn| {
            let revision = conn.query_row(
                "SELECT revision_id, part_id, version, status, created_date, created_by, commit_hash
                 FROM Revisions
                 WHERE revision_id = ?1",
                params![revision_id],
                |row| self.row_to_revision(row),
            )?;
            Ok(revision)
        }).map_err(DatabaseError::from)
    }

    /// Get a revision by its ID within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision to get
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// The revision with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the revision could not be found
    pub fn get_revision_in_transaction(&self, revision_id: i64, tx: &Transaction) -> DatabaseResult<Revision> {
        let revision = tx.query_row(
            "SELECT revision_id, part_id, version, status, created_date, created_by, commit_hash
             FROM Revisions
             WHERE revision_id = ?1",
            params![revision_id],
            |row| self.row_to_revision(row),
        )?;
        Ok(revision)
    }

    /// Get all revisions for a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part
    ///
    /// # Returns
    ///
    /// A vector of revisions for the specified part
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the revisions could not be retrieved
    pub fn get_revisions_for_part(&self, part_id: &str) -> DatabaseResult<Vec<Revision>> {
        self.connection_manager.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT revision_id, part_id, version, status, created_date, created_by, commit_hash
                 FROM Revisions
                 WHERE part_id = ?1
                 ORDER BY created_date DESC",
            )?;
            let revisions_iter = stmt.query_map(params![part_id], |row| self.row_to_revision(row))?;
            let mut revisions = Vec::new();
            for revision_result in revisions_iter {
                revisions.push(revision_result?);
            }
            Ok(revisions)
        }).map_err(DatabaseError::from)
    }

    /// Get the latest revision for a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part
    ///
    /// # Returns
    ///
    /// The latest revision for the specified part
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the revision could not be retrieved
    pub fn get_latest_revision(&self, part_id: &str) -> DatabaseResult<Revision> {
        self.connection_manager.execute(|conn| {
            let revision = conn.query_row(
                "SELECT revision_id, part_id, version, status, created_date, created_by, commit_hash
                 FROM Revisions
                 WHERE part_id = ?1
                 ORDER BY created_date DESC
                 LIMIT 1",
                params![part_id],
                |row| self.row_to_revision(row),
            )?;
            Ok(revision)
        }).map_err(DatabaseError::from)
    }

    /// Get the latest revision for a part within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// The latest revision for the specified part
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the revision could not be retrieved
    pub fn get_latest_revision_in_transaction(&self, part_id: &str, tx: &Transaction) -> DatabaseResult<Revision> {
        let revision = tx.query_row(
            "SELECT revision_id, part_id, version, status, created_date, created_by, commit_hash
             FROM Revisions
             WHERE part_id = ?1
             ORDER BY created_date DESC
             LIMIT 1",
            params![part_id],
            |row| self.row_to_revision(row),
        )?;
        Ok(revision)
    }

    /// Update the status of a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision
    /// * `status` - The new status
    ///
    /// # Returns
    ///
    /// Ok(()) if the status was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the status could not be updated
    pub fn update_status(&self, revision_id: i64, status: RevisionStatus) -> DatabaseResult<()> {
        self.connection_manager.execute_mut(|conn| {
            conn.execute(
                "UPDATE Revisions
                 SET status = ?2
                 WHERE revision_id = ?1",
                params![revision_id, status.to_str()],
            )?;
            Ok(())
        }).map_err(DatabaseError::from)
    }

    /// Update the status of a revision within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision
    /// * `status` - The new status
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// Ok(()) if the status was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the status could not be updated
    pub fn update_status_in_transaction(&self, revision_id: i64, status: RevisionStatus, tx: &Transaction) -> DatabaseResult<()> {
        tx.execute(
            "UPDATE Revisions
             SET status = ?2
             WHERE revision_id = ?1",
            params![revision_id, status.to_str()],
        )?;
        Ok(())
    }

    /// Update the commit hash of a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision
    /// * `commit_hash` - The new commit hash
    ///
    /// # Returns
    ///
    /// Ok(()) if the commit hash was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the commit hash could not be updated
    pub fn update_commit_hash(&self, revision_id: i64, commit_hash: &str) -> DatabaseResult<()> {
        self.connection_manager.execute_mut(|conn| {
            conn.execute(
                "UPDATE Revisions
                 SET commit_hash = ?2
                 WHERE revision_id = ?1",
                params![revision_id, commit_hash],
            )?;
            Ok(())
        }).map_err(DatabaseError::from)
    }

    /// Update the commit hash of a revision within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision
    /// * `commit_hash` - The new commit hash
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// Ok(()) if the commit hash was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the commit hash could not be updated
    pub fn update_commit_hash_in_transaction(&self, revision_id: i64, commit_hash: &str, tx: &Transaction) -> DatabaseResult<()> {
        tx.execute(
            "UPDATE Revisions
             SET commit_hash = ?2
             WHERE revision_id = ?1",
            params![revision_id, commit_hash],
        )?;
        Ok(())
    }

    /// Get the next version number for a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part
    ///
    /// # Returns
    ///
    /// The next version number
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the version number could not be determined
    pub fn get_next_version(&self, part_id: &str) -> DatabaseResult<String> {
        self.connection_manager.execute(|conn| {
            let max_version: Option<String> = conn.query_row(
                "SELECT MAX(version)
                 FROM Revisions
                 WHERE part_id = ?1",
                params![part_id],
                |row| row.get(0),
            ).optional()?;

            if let Some(version) = max_version {
                // Parse the version and increment it
                if let Ok(num) = version.parse::<u32>() {
                    return Ok((num + 1).to_string());
                }
            }

            // If no version exists or parsing failed, start with "1"
            Ok("1".to_string())
        }).map_err(DatabaseError::from)
    }

    /// Get the next version number for a part within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// The next version number
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the version number could not be determined
    pub fn get_next_version_in_transaction(&self, part_id: &str, tx: &Transaction) -> DatabaseResult<String> {
        let max_version: Option<String> = tx.query_row(
            "SELECT MAX(version)
             FROM Revisions
             WHERE part_id = ?1",
            params![part_id],
            |row| row.get(0),
        ).optional()?;

        if let Some(version) = max_version {
            // Parse the version and increment it
            if let Ok(num) = version.parse::<u32>() {
                return Ok((num + 1).to_string());
            }
        }

        // If no version exists or parsing failed, start with "1"
        Ok("1".to_string())
    }

    /// Convert a database row to a Revision
    ///
    /// # Arguments
    ///
    /// * `row` - The database row
    ///
    /// # Returns
    ///
    /// A Revision instance
    ///
    /// # Errors
    ///
    /// Returns a SqliteError if the row could not be converted
    fn row_to_revision(&self, row: &Row) -> SqliteResult<Revision> {
        let status_str: String = row.get(3)?;
        let status = RevisionStatus::from_str(&status_str)
            .unwrap_or(RevisionStatus::Draft);
            
        // Convert SQLite timestamp (seconds since UNIX_EPOCH) to SystemTime
        let created_secs: i64 = row.get(4)?;
        let created_date = UNIX_EPOCH + std::time::Duration::from_secs(created_secs as u64);

        Ok(Revision {
            revision_id: Some(row.get(0)?),
            part_id: row.get(1)?,
            version: row.get(2)?,
            status,
            created_date,
            created_by: row.get(5)?,
            commit_hash: row.get(6)?,
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
    fn test_revision_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create a part manager and a revision manager
        let part_manager = PartManager::new(db_manager.connection_manager());
        let revision_manager = RevisionManager::new(db_manager.connection_manager());

        // Create a new part
        let part = Part::new(
            10001, // Use a numeric part_id instead of a string
            "Electronic".to_string(),
            "Resistor".to_string(),
            "10K Resistor".to_string(),
            Some("1/4W 10K Ohm Resistor".to_string()),
        );

        // Save the part to the database
        part_manager.create_part(&part).unwrap();

        // Create a new revision
        let revision = Revision::new(
            "ELE-RES-001".to_string(),
            "1".to_string(),
            RevisionStatus::Draft,
            "test_user".to_string(),
            Some("abc123".to_string()),
        );

        // Save the revision to the database
        let revision_id = revision_manager.create_revision(&revision).unwrap();

        // Retrieve the revision from the database
        let retrieved_revision = revision_manager.get_revision(revision_id).unwrap();

        // Check that the retrieved revision matches the original
        assert_eq!(retrieved_revision.part_id, revision.part_id);
        assert_eq!(retrieved_revision.version, revision.version);
        assert_eq!(retrieved_revision.status.to_str(), revision.status.to_str());
        assert_eq!(retrieved_revision.created_by, revision.created_by);
        assert_eq!(retrieved_revision.commit_hash, revision.commit_hash);
    }

    #[test]
    fn test_revision_status_conversion() {
        assert_eq!(RevisionStatus::from_str("Draft"), Some(RevisionStatus::Draft));
        assert_eq!(RevisionStatus::from_str("In Review"), Some(RevisionStatus::InReview));
        assert_eq!(RevisionStatus::from_str("Released"), Some(RevisionStatus::Released));
        assert_eq!(RevisionStatus::from_str("Obsolete"), Some(RevisionStatus::Obsolete));
        assert_eq!(RevisionStatus::from_str("Invalid"), None);

        assert_eq!(RevisionStatus::Draft.to_str(), "Draft");
        assert_eq!(RevisionStatus::InReview.to_str(), "In Review");
        assert_eq!(RevisionStatus::Released.to_str(), "Released");
        assert_eq!(RevisionStatus::Obsolete.to_str(), "Obsolete");
    }
}