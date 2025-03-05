//! Approval module for Implexa
//!
//! This module provides functionality for managing approvals of revisions in the database.

use rusqlite::{Connection, params, Row, Result as SqliteResult};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::database::schema::DatabaseResult;

/// Status of an approval
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalStatus {
    /// Pending status - waiting for approval
    Pending,
    /// Approved status - approved by the approver
    Approved,
    /// Rejected status - rejected by the approver
    Rejected,
}

impl ApprovalStatus {
    /// Convert a string to an ApprovalStatus
    ///
    /// # Arguments
    ///
    /// * `status` - The status string
    ///
    /// # Returns
    ///
    /// The corresponding ApprovalStatus
    pub fn from_str(status: &str) -> Option<Self> {
        match status {
            "Pending" => Some(Self::Pending),
            "Approved" => Some(Self::Approved),
            "Rejected" => Some(Self::Rejected),
            _ => None,
        }
    }

    /// Convert an ApprovalStatus to a string
    ///
    /// # Returns
    ///
    /// The string representation of the status
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Pending => "Pending",
            Self::Approved => "Approved",
            Self::Rejected => "Rejected",
        }
    }
}

/// Represents an approval of a revision
#[derive(Debug, Clone)]
pub struct Approval {
    /// Unique identifier for the approval
    pub approval_id: Option<i64>,
    /// ID of the revision this approval is for
    pub revision_id: i64,
    /// User who is approving the revision
    pub approver: String,
    /// Status of the approval
    pub status: ApprovalStatus,
    /// Date the approval was made
    pub date: Option<SystemTime>,
    /// Comments from the approver
    pub comments: Option<String>,
}

impl Approval {
    /// Create a new approval
    ///
    /// # Arguments
    ///
    /// * `revision_id` - ID of the revision this approval is for
    /// * `approver` - User who is approving the revision
    /// * `status` - Status of the approval
    /// * `comments` - Comments from the approver
    ///
    /// # Returns
    ///
    /// A new Approval instance
    pub fn new(
        revision_id: i64,
        approver: String,
        status: ApprovalStatus,
        comments: Option<String>,
    ) -> Self {
        let date = if status == ApprovalStatus::Pending {
            None
        } else {
            Some(SystemTime::now())
        };

        Self {
            approval_id: None,
            revision_id,
            approver,
            status,
            date,
            comments,
        }
    }
}

/// Manager for approval operations
pub struct ApprovalManager<'a> {
    /// Connection to the SQLite database
    connection: &'a Connection,
}

impl<'a> ApprovalManager<'a> {
    /// Create a new ApprovalManager
    ///
    /// # Arguments
    ///
    /// * `connection` - Connection to the SQLite database
    ///
    /// # Returns
    ///
    /// A new ApprovalManager instance
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    /// Create a new approval in the database
    ///
    /// # Arguments
    ///
    /// * `approval` - The approval to create
    ///
    /// # Returns
    ///
    /// The ID of the newly created approval
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the approval could not be created
    pub fn create_approval(&self, approval: &Approval) -> DatabaseResult<i64> {
        // Convert SystemTime to seconds since UNIX_EPOCH for SQLite
        let date_secs = approval.date.map(|d| {
            d.duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64
        });
        
        self.connection.execute(
            "INSERT INTO Approvals (revision_id, approver, status, date, comments)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                approval.revision_id,
                approval.approver,
                approval.status.to_str(),
                date_secs,
                approval.comments,
            ],
        )?;
        Ok(self.connection.last_insert_rowid())
    }

    /// Get an approval by its ID
    ///
    /// # Arguments
    ///
    /// * `approval_id` - The ID of the approval to get
    ///
    /// # Returns
    ///
    /// The approval with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the approval could not be found
    pub fn get_approval(&self, approval_id: i64) -> DatabaseResult<Approval> {
        let approval = self.connection.query_row(
            "SELECT approval_id, revision_id, approver, status, date, comments
             FROM Approvals
             WHERE approval_id = ?1",
            params![approval_id],
            |row| self.row_to_approval(row),
        )?;
        Ok(approval)
    }

    /// Get all approvals for a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision
    ///
    /// # Returns
    ///
    /// A vector of approvals for the specified revision
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the approvals could not be retrieved
    pub fn get_approvals_for_revision(&self, revision_id: i64) -> DatabaseResult<Vec<Approval>> {
        let mut stmt = self.connection.prepare(
            "SELECT approval_id, revision_id, approver, status, date, comments
             FROM Approvals
             WHERE revision_id = ?1
             ORDER BY approver",
        )?;
        let approvals_iter = stmt.query_map(params![revision_id], |row| self.row_to_approval(row))?;
        let mut approvals = Vec::new();
        for approval_result in approvals_iter {
            approvals.push(approval_result?);
        }
        Ok(approvals)
    }

    /// Get an approval for a specific revision and approver
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision
    /// * `approver` - The approver
    ///
    /// # Returns
    ///
    /// The approval for the specified revision and approver
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the approval could not be found
    pub fn get_approval_for_revision_and_approver(&self, revision_id: i64, approver: &str) -> DatabaseResult<Approval> {
        let approval = self.connection.query_row(
            "SELECT approval_id, revision_id, approver, status, date, comments
             FROM Approvals
             WHERE revision_id = ?1 AND approver = ?2",
            params![revision_id, approver],
            |row| self.row_to_approval(row),
        )?;
        Ok(approval)
    }

    /// Update the status of an approval
    ///
    /// # Arguments
    ///
    /// * `approval_id` - The ID of the approval
    /// * `status` - The new status
    /// * `comments` - Comments from the approver
    ///
    /// # Returns
    ///
    /// Ok(()) if the status was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the status could not be updated
    pub fn update_status(&self, approval_id: i64, status: ApprovalStatus, comments: Option<&str>) -> DatabaseResult<()> {
        let date = if status == ApprovalStatus::Pending {
            None
        } else {
            Some(SystemTime::now())
        };

        // Convert SystemTime to seconds since UNIX_EPOCH for SQLite
        let date_secs = date.map(|d| {
            d.duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64
        });

        self.connection.execute(
            "UPDATE Approvals
             SET status = ?2, date = ?3, comments = ?4
             WHERE approval_id = ?1",
            params![
                approval_id,
                status.to_str(),
                date_secs,
                comments,
            ],
        )?;
        Ok(())
    }

    /// Delete an approval
    ///
    /// # Arguments
    ///
    /// * `approval_id` - The ID of the approval to delete
    ///
    /// # Returns
    ///
    /// Ok(()) if the approval was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the approval could not be deleted
    pub fn delete_approval(&self, approval_id: i64) -> DatabaseResult<()> {
        self.connection.execute(
            "DELETE FROM Approvals WHERE approval_id = ?1",
            params![approval_id],
        )?;
        Ok(())
    }

    /// Check if a revision is fully approved
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision
    ///
    /// # Returns
    ///
    /// true if all approvals for the revision are approved, false otherwise
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the approval status could not be determined
    pub fn is_revision_approved(&self, revision_id: i64) -> DatabaseResult<bool> {
        let approvals = self.get_approvals_for_revision(revision_id)?;
        
        if approvals.is_empty() {
            return Ok(false);
        }

        for approval in &approvals {
            if approval.status != ApprovalStatus::Approved {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Convert a database row to an Approval
    ///
    /// # Arguments
    ///
    /// * `row` - The database row
    ///
    /// # Returns
    ///
    /// An Approval instance
    ///
    /// # Errors
    ///
    /// Returns a SqliteError if the row could not be converted
    fn row_to_approval(&self, row: &Row) -> SqliteResult<Approval> {
        let status_str: String = row.get(3)?;
        let status = ApprovalStatus::from_str(&status_str)
            .unwrap_or(ApprovalStatus::Pending);
            
        // Convert SQLite timestamp (seconds since UNIX_EPOCH) to SystemTime
        let date_secs: Option<i64> = row.get(4)?;
        let date = date_secs.map(|secs| {
            UNIX_EPOCH + std::time::Duration::from_secs(secs as u64)
        });

        Ok(Approval {
            approval_id: Some(row.get(0)?),
            revision_id: row.get(1)?,
            approver: row.get(2)?,
            status,
            date,
            comments: row.get(5)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::DatabaseManager;
    use crate::database::part::{Part, PartManager};
    use crate::database::revision::{Revision, RevisionStatus, RevisionManager};
    use tempfile::tempdir;

    #[test]
    fn test_approval_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let mut db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create managers
        let part_manager = PartManager::new(db_manager.connection());
        let revision_manager = RevisionManager::new(db_manager.connection());
        let approval_manager = ApprovalManager::new(db_manager.connection());

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

        // Create a new revision
        let revision = Revision::new(
            "ELE-RES-001".to_string(),
            "1".to_string(),
            RevisionStatus::InReview,
            "test_user".to_string(),
            Some("abc123".to_string()),
        );

        // Save the revision to the database
        let revision_id = revision_manager.create_revision(&revision).unwrap();

        // Create a new approval
        let approval = Approval::new(
            revision_id,
            "approver1".to_string(),
            ApprovalStatus::Pending,
            None,
        );

        // Save the approval to the database
        let approval_id = approval_manager.create_approval(&approval).unwrap();

        // Retrieve the approval from the database
        let retrieved_approval = approval_manager.get_approval(approval_id).unwrap();

        // Check that the retrieved approval matches the original
        assert_eq!(retrieved_approval.revision_id, approval.revision_id);
        assert_eq!(retrieved_approval.approver, approval.approver);
        assert_eq!(retrieved_approval.status.to_str(), approval.status.to_str());
        assert_eq!(retrieved_approval.comments, approval.comments);
    }

    #[test]
    fn test_approval_status_update() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let mut db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create managers
        let part_manager = PartManager::new(db_manager.connection());
        let revision_manager = RevisionManager::new(db_manager.connection());
        let approval_manager = ApprovalManager::new(db_manager.connection());

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

        // Create a new revision
        let revision = Revision::new(
            "ELE-RES-001".to_string(),
            "1".to_string(),
            RevisionStatus::InReview,
            "test_user".to_string(),
            Some("abc123".to_string()),
        );

        // Save the revision to the database
        let revision_id = revision_manager.create_revision(&revision).unwrap();

        // Create a new approval
        let approval = Approval::new(
            revision_id,
            "approver1".to_string(),
            ApprovalStatus::Pending,
            None,
        );

        // Save the approval to the database
        let approval_id = approval_manager.create_approval(&approval).unwrap();

        // Update the approval status
        approval_manager.update_status(approval_id, ApprovalStatus::Approved, Some("Looks good!")).unwrap();

        // Retrieve the approval from the database
        let retrieved_approval = approval_manager.get_approval(approval_id).unwrap();

        // Check that the status was updated
        assert_eq!(retrieved_approval.status, ApprovalStatus::Approved);
        assert_eq!(retrieved_approval.comments, Some("Looks good!".to_string()));
        assert!(retrieved_approval.date.is_some());
    }

    #[test]
    fn test_approval_status_conversion() {
        assert_eq!(ApprovalStatus::from_str("Pending"), Some(ApprovalStatus::Pending));
        assert_eq!(ApprovalStatus::from_str("Approved"), Some(ApprovalStatus::Approved));
        assert_eq!(ApprovalStatus::from_str("Rejected"), Some(ApprovalStatus::Rejected));
        assert_eq!(ApprovalStatus::from_str("Invalid"), None);

        assert_eq!(ApprovalStatus::Pending.to_str(), "Pending");
        assert_eq!(ApprovalStatus::Approved.to_str(), "Approved");
        assert_eq!(ApprovalStatus::Rejected.to_str(), "Rejected");
    }
}