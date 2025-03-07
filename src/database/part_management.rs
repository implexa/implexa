//! Part Management module for Implexa
//!
//! This module provides functionality for managing parts throughout their lifecycle,
//! including creation, status transitions, and workflow enforcement.

use rusqlite::{Connection, params, Result as SqliteResult};
use std::time::SystemTime;
use crate::database::schema::{DatabaseError, DatabaseResult};
use crate::database::part::{Part, PartManager};
use crate::database::revision::{Revision, RevisionStatus, RevisionManager};
use crate::database::approval::{Approval, ApprovalStatus, ApprovalManager};
use crate::database::workflow::{WorkflowManager};
use crate::git_backend::{GitBackendManager, GitBackendError};
use crate::database::connection_manager::ConnectionManager;
use std::path::Path;

/// Error types specific to part management
#[derive(Debug, thiserror::Error)]
pub enum PartManagementError {
    /// Database error
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
    
    /// Git backend error
    #[error("Git backend error: {0}")]
    GitBackendError(#[from] GitBackendError),
    
    /// SQLite error
    #[error("SQLite error: {0}")]
    SqliteError(#[from] rusqlite::Error),
    
    /// Invalid state transition
    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),
    
    /// Approval required
    #[error("Approval required: {0}")]
    ApprovalRequired(String),
    
    /// Permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    
    /// Other errors
    #[error("Part management error: {0}")]
    Other(String),
}

/// Result type for part management operations
pub type PartManagementResult<T> = Result<T, PartManagementError>;

/// Represents a user role in the system
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    /// Designer role - can create, edit, and submit parts for review
    Designer,
    /// Viewer role - can view and comment on parts
    Viewer,
    /// Admin role - has full control over the system
    Admin,
}

/// Represents a user in the system
#[derive(Debug, Clone)]
pub struct User {
    /// Username
    pub username: String,
    /// User role
    pub role: UserRole,
}

impl User {
    /// Create a new user
    ///
    /// # Arguments
    ///
    /// * `username` - Username
    /// * `role` - User role
    ///
    /// # Returns
    ///
    /// A new User instance
    pub fn new(username: String, role: UserRole) -> Self {
        Self { username, role }
    }
    
    /// Check if the user can create parts
    ///
    /// # Returns
    ///
    /// true if the user can create parts, false otherwise
    pub fn can_create_parts(&self) -> bool {
        matches!(self.role, UserRole::Designer | UserRole::Admin)
    }
    
    /// Check if the user can edit a part
    ///
    /// # Arguments
    ///
    /// * `part_author` - Username of the part author
    ///
    /// # Returns
    ///
    /// true if the user can edit the part, false otherwise
    pub fn can_edit_part(&self, part_author: &str) -> bool {
        match self.role {
            UserRole::Designer => self.username == part_author,
            UserRole::Admin => true,
            _ => false,
        }
    }
    
    /// Check if the user can approve a part
    ///
    /// # Arguments
    ///
    /// * `part_author` - Username of the part author
    ///
    /// # Returns
    ///
    /// true if the user can approve the part, false otherwise
    pub fn can_approve_part(&self, part_author: &str) -> bool {
        match self.role {
            UserRole::Designer => self.username != part_author,
            UserRole::Admin => true,
            _ => false,
        }
    }
}

/// Manager for part management operations
pub struct PartManagementManager<'a> {
    /// Connection manager for the SQLite database
    connection_manager: &'a ConnectionManager,
    /// Git backend manager
    git_manager: &'a GitBackendManager,
    /// Current user
    current_user: User,
}

impl<'a> PartManagementManager<'a> {
    /// Create a new PartManagementManager
    ///
    /// # Arguments
    ///
    /// * `connection_manager` - Connection manager for the SQLite database
    /// * `git_manager` - Git backend manager
    /// * `current_user` - Current user
    ///
    /// # Returns
    ///
    /// A new PartManagementManager instance
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
    /// Create a new part with initial metadata and set up Git branch
    ///
    /// # Arguments
    ///
    /// * `category` - Category of the part
    /// * `subcategory` - Subcategory of the part
    /// * `name` - Name of the part
    /// * `description` - Description of the part
    /// * `repo_path` - Path to the Git repository
    ///
    /// # Returns
    ///
    /// The newly created part and its revision ID
    ///
    /// # Errors
    ///
    /// Returns a PartManagementError if the part could not be created
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
        self.connection_manager.transaction::<_, _, PartManagementError>(|tx| {
            // Create part managers
            let part_manager = PartManager::new(self.connection_manager);
            let revision_manager = RevisionManager::new(self.connection_manager);
            
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
            let display_part_number = part.display_part_number_in_transaction(tx);
            
            // Create a new revision in Draft state
            let revision = Revision::new(
                part.part_id,
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
        })
    }
    
    /// Submit a part for review
    ///
    /// # Arguments
    ///
    /// * `revision_id` - ID of the revision to submit for review
    /// * `repo_path` - Path to the Git repository
    /// * `reviewers` - List of usernames to request review from
    ///
    /// # Returns
    ///
    /// Ok(()) if the part was successfully submitted for review
    ///
    /// # Errors
    ///
    /// Returns a PartManagementError if the part could not be submitted for review
    pub fn submit_for_review(
        &self,
        revision_id: i64,
        repo_path: &Path,
        reviewers: Vec<String>,
    ) -> PartManagementResult<()> {
        self.connection_manager.transaction::<_, _, PartManagementError>(|tx| {
            // Create managers
            let revision_manager = RevisionManager::new(self.connection_manager);
            let approval_manager = ApprovalManager::new(self.connection_manager);
            
            // Get the revision
            let revision = revision_manager.get_revision_in_transaction(revision_id, tx)?;
            
            // Check if the user has permission to submit the part for review
            if !self.current_user.can_edit_part(&revision.created_by) {
                return Err(PartManagementError::PermissionDenied(
                    "User does not have permission to submit this part for review".to_string(),
                ));
            }
            
            // Check if the revision is in Draft state
            if revision.status != RevisionStatus::Draft {
                return Err(PartManagementError::InvalidStateTransition(
                    format!("Cannot submit revision for review: current status is {}", revision.status.to_str()),
                ));
            }
            
            // Get the part
            let part_manager = PartManager::new(self.connection_manager);
            let part = part_manager.get_part_in_transaction(revision.part_id, tx)?;
            
            // Generate the display part number
            let display_part_number = part.display_part_number_in_transaction(tx);
            
            // Create a review branch
            let feature_branch = format!("part/{}/draft", display_part_number);
            let review_branch = format!("part/{}/review", display_part_number);
            
            // Create and checkout the review branch
            // Open the repository first
            let repo = self.git_manager.open_repository(repo_path)?;
            self.git_manager.create_branch(&repo, &review_branch)?;
            self.git_manager.checkout_branch(&repo, &review_branch)?;
            
            // Update the revision status to In Review
            revision_manager.update_status_in_transaction(revision_id, RevisionStatus::InReview, tx)?;
            
            // Create approval requests for each reviewer
            for reviewer in reviewers {
                let approval = Approval::new(
                    revision_id,
                    reviewer,
                    ApprovalStatus::Pending,
                    None,
                );
                approval_manager.create_approval_in_transaction(&approval, tx)?;
            }
            
            Ok(())
        })
    }
    
    /// Approve a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - ID of the revision to approve
    /// * `comments` - Comments from the approver
    ///
    /// # Returns
    ///
    /// Ok(()) if the revision was successfully approved
    ///
    /// # Errors
    ///
    /// Returns a PartManagementError if the revision could not be approved
    pub fn approve_revision(
        &self,
        revision_id: i64,
        comments: Option<String>,
    ) -> PartManagementResult<()> {
        self.connection_manager.transaction::<_, _, PartManagementError>(|tx| {
            // Create managers
            let revision_manager = RevisionManager::new(self.connection_manager);
            let approval_manager = ApprovalManager::new(self.connection_manager);
            
            // Get the revision
            let revision = revision_manager.get_revision_in_transaction(revision_id, tx)?;
            
            // Check if the user has permission to approve the revision
            if !self.current_user.can_approve_part(&revision.created_by) {
                return Err(PartManagementError::PermissionDenied(
                    "User does not have permission to approve this revision".to_string(),
                ));
            }
            
            // Check if the revision is in In Review state
            if revision.status != RevisionStatus::InReview {
                return Err(PartManagementError::InvalidStateTransition(
                    format!("Cannot approve revision: current status is {}", revision.status.to_str()),
                ));
            }
            
            // Get the approval for this user
            let approval = match approval_manager.get_approval_for_revision_and_approver_in_transaction(
                revision_id,
                &self.current_user.username,
                tx
            ) {
                Ok(approval) => approval,
                Err(_) => {
                    // Create a new approval if one doesn't exist
                    let approval = Approval::new(
                        revision_id,
                        self.current_user.username.clone(),
                        ApprovalStatus::Pending,
                        None,
                    );
                    let approval_id = approval_manager.create_approval_in_transaction(&approval, tx)?;
                    approval_manager.get_approval_in_transaction(approval_id, tx)?
                }
            };
            
            // Update the approval status
            approval_manager.update_status_in_transaction(
                approval.approval_id.unwrap(),
                ApprovalStatus::Approved,
                comments.as_deref(),
                tx
            )?;
            
            Ok(())
        })
    }
    
    /// Reject a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - ID of the revision to reject
    /// * `comments` - Comments from the reviewer
    ///
    /// # Returns
    ///
    /// Ok(()) if the revision was successfully rejected
    ///
    /// # Errors
    ///
    /// Returns a PartManagementError if the revision could not be rejected
    pub fn reject_revision(
        &self,
        revision_id: i64,
        comments: Option<String>,
    ) -> PartManagementResult<()> {
        self.connection_manager.transaction::<_, _, PartManagementError>(|tx| {
            // Create managers
            let revision_manager = RevisionManager::new(self.connection_manager);
            let approval_manager = ApprovalManager::new(self.connection_manager);
            
            // Get the revision
            let revision = revision_manager.get_revision_in_transaction(revision_id, tx)?;
            
            // Check if the user has permission to reject the revision
            if !self.current_user.can_approve_part(&revision.created_by) {
                return Err(PartManagementError::PermissionDenied(
                    "User does not have permission to reject this revision".to_string(),
                ));
            }
            
            // Check if the revision is in In Review state
            if revision.status != RevisionStatus::InReview {
                return Err(PartManagementError::InvalidStateTransition(
                    format!("Cannot reject revision: current status is {}", revision.status.to_str()),
                ));
            }
            
            // Get the approval for this user
            let approval = match approval_manager.get_approval_for_revision_and_approver_in_transaction(
                revision_id,
                &self.current_user.username,
                tx
            ) {
                Ok(approval) => approval,
                Err(_) => {
                    // Create a new approval if one doesn't exist
                    let approval = Approval::new(
                        revision_id,
                        self.current_user.username.clone(),
                        ApprovalStatus::Pending,
                        None,
                    );
                    let approval_id = approval_manager.create_approval_in_transaction(&approval, tx)?;
                    approval_manager.get_approval_in_transaction(approval_id, tx)?
                }
            };
            
            // Update the approval status
            approval_manager.update_status_in_transaction(
                approval.approval_id.unwrap(),
                ApprovalStatus::Rejected,
                comments.as_deref(),
                tx
            )?;
            
            // Update the revision status back to Draft
            revision_manager.update_status_in_transaction(revision_id, RevisionStatus::Draft, tx)?;
            
            Ok(())
        })
    }
    
    /// Release a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - ID of the revision to release
    /// * `repo_path` - Path to the Git repository
    ///
    /// # Returns
    ///
    /// Ok(()) if the revision was successfully released
    ///
    /// # Errors
    ///
    /// Returns a PartManagementError if the revision could not be released
    pub fn release_revision(
        &self,
        revision_id: i64,
        repo_path: &Path,
    ) -> PartManagementResult<()> {
        self.connection_manager.transaction::<_, _, PartManagementError>(|tx| {
            // Create managers
            let revision_manager = RevisionManager::new(self.connection_manager);
            let approval_manager = ApprovalManager::new(self.connection_manager);
            let part_manager = PartManager::new(self.connection_manager);
            
            // Get the revision
            let revision = revision_manager.get_revision_in_transaction(revision_id, tx)?;
            
            // Check if the user has permission to release the revision
            if !self.current_user.can_approve_part(&revision.created_by) {
                return Err(PartManagementError::PermissionDenied(
                    "User does not have permission to release this revision".to_string(),
                ));
            }
            
            // Check if the revision is in In Review state
            if revision.status != RevisionStatus::InReview {
                return Err(PartManagementError::InvalidStateTransition(
                    format!("Cannot release revision: current status is {}", revision.status.to_str()),
                ));
            }
            
            // Check if the revision is fully approved
            if !approval_manager.is_revision_approved_in_transaction(revision_id, tx)? {
                return Err(PartManagementError::ApprovalRequired(
                    "Revision must be fully approved before release".to_string(),
                ));
            }
            
            // Get the part
            let part = part_manager.get_part_in_transaction(revision.part_id, tx)?;
            
            // Generate the display part number
            let display_part_number = part.display_part_number_in_transaction(tx);
            
            // Checkout the main branch
            // Open the repository first
            let repo = self.git_manager.open_repository(repo_path)?;
            self.git_manager.checkout_branch(&repo, "main")?;
            
            // Merge the review branch into main
            let review_branch = format!("part/{}/review", display_part_number);
            self.git_manager.merge_branch(&repo, &review_branch)?;
            
            // Update the revision status to Released
            revision_manager.update_status_in_transaction(revision_id, RevisionStatus::Released, tx)?;
            
            Ok(())
        })
    }
    
    /// Mark a revision as obsolete
    ///
    /// # Arguments
    ///
    /// * `revision_id` - ID of the revision to mark as obsolete
    ///
    /// # Returns
    ///
    /// Ok(()) if the revision was successfully marked as obsolete
    ///
    /// # Errors
    ///
    /// Returns a PartManagementError if the revision could not be marked as obsolete
    pub fn mark_as_obsolete(
        &self,
        revision_id: i64,
    ) -> PartManagementResult<()> {
        self.connection_manager.transaction::<_, _, PartManagementError>(|tx| {
            // Create managers
            let revision_manager = RevisionManager::new(self.connection_manager);
            
            // Get the revision
            let revision = revision_manager.get_revision_in_transaction(revision_id, tx)?;
            
            // Check if the user has permission to mark the revision as obsolete
            if !self.current_user.can_approve_part(&revision.created_by) {
                return Err(PartManagementError::PermissionDenied(
                    "User does not have permission to mark this revision as obsolete".to_string(),
                ));
            }
            
            // Check if the revision is in Released state
            if revision.status != RevisionStatus::Released {
                return Err(PartManagementError::InvalidStateTransition(
                    format!("Cannot mark revision as obsolete: current status is {}", revision.status.to_str()),
                ));
            }
            
            // Update the revision status to Obsolete
            revision_manager.update_status_in_transaction(revision_id, RevisionStatus::Obsolete, tx)?;
            
            Ok(())
        })
    }
    
    /// Create a new revision of a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - ID of the part to revise
    /// * `repo_path` - Path to the Git repository
    ///
    /// # Returns
    ///
    /// The newly created revision ID
    ///
    /// # Errors
    ///
    /// Returns a PartManagementError if the revision could not be created
    pub fn create_revision(
        &self,
        part_id: i64,
        repo_path: &Path,
    ) -> PartManagementResult<i64> {
        self.connection_manager.transaction::<_, _, PartManagementError>(|tx| {
            // Create managers
            let part_manager = PartManager::new(self.connection_manager);
            let revision_manager = RevisionManager::new(self.connection_manager);
            
            // Get the part
            let part = part_manager.get_part_in_transaction(part_id, tx)?;
            
            // Check if the user has permission to create a revision
            if !self.current_user.can_create_parts() {
                return Err(PartManagementError::PermissionDenied(
                    "User does not have permission to create revisions".to_string(),
                ));
            }
            
            // Get the latest revision
            let latest_revision = revision_manager.get_latest_revision_in_transaction(part_id, tx)?;
            
            // Check if the latest revision is in Released state
            if latest_revision.status != RevisionStatus::Released {
                return Err(PartManagementError::InvalidStateTransition(
                    format!("Cannot create new revision: latest revision status is {}", latest_revision.status.to_str()),
                ));
            }
            
            // Get the next version number
            let next_version = revision_manager.get_next_version_in_transaction(part_id, tx)?;
            
            // Generate the display part number
            let display_part_number = part.display_part_number_in_transaction(tx);
            
            // Create a new feature branch from main - include version number to make it unique
            let branch_name = format!("part/{}/v{}/draft", display_part_number, next_version);
            // Open the repository first
            let repo = self.git_manager.open_repository(repo_path)?;
            self.git_manager.checkout_branch(&repo, "main")?;
            self.git_manager.create_branch(&repo, &branch_name)?;
            self.git_manager.checkout_branch(&repo, &branch_name)?;
            
            // Create a new revision in Draft state
            let revision = Revision::new(
                part_id,
                next_version,
                RevisionStatus::Draft,
                self.current_user.username.clone(),
                None, // No commit hash yet
            );
            
            // Save the revision to the database
            let revision_id = revision_manager.create_revision_in_transaction(&revision, tx)?;
            
            Ok(revision_id)
        })
    }
    
    /// Update the commit hash for a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - ID of the revision
    /// * `commit_hash` - Git commit hash
    ///
    /// # Returns
    ///
    /// Ok(()) if the commit hash was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a PartManagementError if the commit hash could not be updated
    pub fn update_commit_hash(
        &self,
        revision_id: i64,
        commit_hash: &str,
    ) -> PartManagementResult<()> {
        self.connection_manager.transaction::<_, _, PartManagementError>(|tx| {
            // Create a revision manager
            let revision_manager = RevisionManager::new(self.connection_manager);
            
            // Get the revision
            let revision = revision_manager.get_revision_in_transaction(revision_id, tx)?;
            
            // Check if the user has permission to update the commit hash
            if !self.current_user.can_edit_part(&revision.created_by) {
                return Err(PartManagementError::PermissionDenied(
                    "User does not have permission to update this revision".to_string(),
                ));
            }
            
            // Update the commit hash
            revision_manager.update_commit_hash_in_transaction(revision_id, commit_hash, tx)?;
            
            Ok(())
        })
    }
    
    /// Get all revisions for a part with their approval status
    ///
    /// # Arguments
    ///
    /// * `part_id` - ID of the part
    ///
    /// # Returns
    ///
    /// A vector of revisions with their approval status
    ///
    /// # Errors
    ///
    /// Returns a PartManagementError if the revisions could not be retrieved
    pub fn get_revisions_with_approvals(
        &self,
        part_id: i64,
    ) -> PartManagementResult<Vec<(Revision, Vec<Approval>)>> {
        self.connection_manager.execute::<_, _, PartManagementError>(|conn| {
            // Create managers
            let revision_manager = RevisionManager::new(self.connection_manager);
            let approval_manager = ApprovalManager::new(self.connection_manager);
            
            // Get all revisions for the part
            let revisions = revision_manager.get_revisions_for_part(part_id)?;
            
            // Get approvals for each revision
            let mut result = Vec::new();
            for revision in revisions {
                let approvals = approval_manager.get_approvals_for_revision(revision.revision_id.unwrap())?;
                result.push((revision, approvals));
            }
            
            Ok(result)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::DatabaseManager;
    use crate::git_backend::{GitBackendConfig, AuthConfig};
    use tempfile::tempdir;
    
    #[test]
    fn test_part_creation_and_workflow() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        // Create a temporary directory for the Git repository
        let repo_dir = tempdir().unwrap();
        let repo_path = repo_dir.path();
        
        // Create a new database manager and initialize the schema
        let db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();
        
        // Create a workflow manager and set up the default part workflow
        let workflow_manager = WorkflowManager::new(db_manager.connection_manager());
        workflow_manager.create_default_part_workflow().unwrap();
        
        // Create a Git backend manager
        let git_config = GitBackendConfig::default();
        let auth_config = AuthConfig::default();
        let git_manager = GitBackendManager::new(git_config, auth_config).unwrap();
        
        // Initialize a Git repository
        let repo = git_manager.init_repository(repo_path).unwrap();
        
        // Create an initial commit so that the main branch exists
        // This is needed for later operations in the test that require a branch
        let sig = git2::Signature::now("Test User", "test@example.com").unwrap();
        let tree_id = {
            let mut index = repo.index().unwrap();
            index.write_tree().unwrap()
        };
        let tree = repo.find_tree(tree_id).unwrap();
        let commit_id = repo.commit(Some("refs/heads/main"), &sig, &sig, "Initial commit", &tree, &[]).unwrap();
        
        // Explicitly set HEAD to point to the main branch
        repo.set_head("refs/heads/main").unwrap();
        
        // Make sure default branch is correctly set
        let _main_branch = repo.find_branch("main", git2::BranchType::Local).unwrap();
        
        // Create a user
        let user = User::new("test_user".to_string(), UserRole::Designer);
        
        // Create a part management manager
        let part_mgmt = PartManagementManager::new(
            db_manager.connection_manager(),
            &git_manager,
            user,
        );
        
        // Create a new part
        let (part, revision_id) = part_mgmt.create_part(
            "Electronic".to_string(),
            "Resistor".to_string(),
            "10K Resistor".to_string(),
            Some("1/4W 10K Ohm Resistor".to_string()),
            repo_path,
        ).unwrap();
        
        // Check that the part was created
        let part_manager = PartManager::new(db_manager.connection_manager());
        let retrieved_part = part_manager.get_part(part.part_id).unwrap();
        assert_eq!(retrieved_part.name, "10K Resistor");
        
        // Check that the revision was created
        let revision_manager = RevisionManager::new(db_manager.connection_manager());
        let revision = revision_manager.get_revision(revision_id).unwrap();
        assert_eq!(revision.status, RevisionStatus::Draft);
        
        // Create a reviewer user
        let reviewer = User::new("reviewer".to_string(), UserRole::Designer);
        
        // Create a part management manager for the reviewer
        let reviewer_mgmt = PartManagementManager::new(
            db_manager.connection_manager(),
            &git_manager,
            reviewer,
        );
        
        // Submit the part for review
        part_mgmt.submit_for_review(
            revision_id,
            repo_path,
            vec!["reviewer".to_string()],
        ).unwrap();
        
        // Check that the revision status was updated
        let revision = revision_manager.get_revision(revision_id).unwrap();
        assert_eq!(revision.status, RevisionStatus::InReview);
        
        // Check that an approval was created
        let approval_manager = ApprovalManager::new(db_manager.connection_manager());
        let approvals = approval_manager.get_approvals_for_revision(revision_id).unwrap();
        assert_eq!(approvals.len(), 1);
        assert_eq!(approvals[0].approver, "reviewer");
        assert_eq!(approvals[0].status, ApprovalStatus::Pending);
        
        // Approve the revision
        reviewer_mgmt.approve_revision(
            revision_id,
            Some("Looks good!".to_string()),
        ).unwrap();
        
        // Check that the approval status was updated
        let approvals = approval_manager.get_approvals_for_revision(revision_id).unwrap();
        assert_eq!(approvals[0].status, ApprovalStatus::Approved);
        
        // Release the revision
        reviewer_mgmt.release_revision(
            revision_id,
            repo_path,
        ).unwrap();
        
        // Check that the revision status was updated
        let revision = revision_manager.get_revision(revision_id).unwrap();
        assert_eq!(revision.status, RevisionStatus::Released);
        
        // Create a new revision
        let new_revision_id = reviewer_mgmt.create_revision(
            part.part_id,
            repo_path,
        ).unwrap();
        
        // Check that the new revision was created
        let new_revision = revision_manager.get_revision(new_revision_id).unwrap();
        assert_eq!(new_revision.status, RevisionStatus::Draft);
        assert_eq!(new_revision.version, "2");
        
        // Mark the original revision as obsolete
        reviewer_mgmt.mark_as_obsolete(revision_id).unwrap();
        
        // Check that the revision status was updated
        let revision = revision_manager.get_revision(revision_id).unwrap();
        assert_eq!(revision.status, RevisionStatus::Obsolete);
    }
}