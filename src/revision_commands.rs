//! Tauri command handlers for revision operations
//!
//! This module contains the command handlers for revision-related operations in the Tauri application.
//! These commands are exposed to the frontend and allow it to interact with the revision management system.

use std::sync::Mutex;
use tauri::{command, State};
use serde::{Serialize, Deserialize};
use implexa::database::revision::{RevisionManager, Revision, RevisionStatus};
use implexa::database::connection_manager::ConnectionManager;

/// Revision information for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionDto {
    /// Revision ID
    pub revision_id: i64,
    /// Part ID this revision is associated with
    pub part_id: i64,
    /// Revision version
    pub version: String,
    /// Status of the revision (Draft, InReview, Released, Obsolete)
    pub status: String,
    /// User who created the revision
    pub created_by: String,
    /// Git commit hash associated with this revision
    pub commit_hash: Option<String>,
}

/// Revision creation data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevisionCreationData {
    /// Part ID this revision is associated with
    pub part_id: i64,
    /// Revision version
    pub version: String,
    /// Status of the revision (Draft, InReview, Released, Obsolete)
    pub status: String,
    /// User who created the revision
    pub created_by: String,
    /// Git commit hash associated with this revision
    pub commit_hash: Option<String>,
}

/// Revision state for the application
pub struct RevisionState {
    /// Connection manager for the database
    pub connection_manager: ConnectionManager,
    /// Revision manager for revision operations
    pub revision_manager: Mutex<RevisionManager<'static>>,
}

impl From<Revision> for RevisionDto {
    fn from(revision: Revision) -> Self {
        Self {
            revision_id: revision.revision_id.unwrap_or_default(),
            part_id: revision.part_id,
            version: revision.version,
            status: revision.status.to_str().to_string(),
            created_by: revision.created_by,
            commit_hash: revision.commit_hash,
        }
    }
}

/// Initialize the revision state
pub fn init_revision_state(connection_manager: ConnectionManager) -> RevisionState {
    // Create a revision manager with 'static lifetime using a leak (safe in this context)
    let static_connection_manager: &'static ConnectionManager = Box::leak(Box::new(connection_manager.clone()));
    let revision_manager = RevisionManager::new(static_connection_manager);
    
    RevisionState {
        connection_manager,
        revision_manager: Mutex::new(revision_manager),
    }
}

/// Get a revision by ID
#[command]
pub async fn get_revision(
    revision_id: i64,
    revision_state: State<'_, RevisionState>,
) -> Result<RevisionDto, String> {
    let revision_manager = revision_state.revision_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the revision
    let revision = revision_manager.get_revision(revision_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(RevisionDto::from(revision))
}

/// Get all revisions for a part
#[command]
pub async fn get_part_revisions(
    part_id: i64,
    revision_state: State<'_, RevisionState>,
) -> Result<Vec<RevisionDto>, String> {
    let revision_manager = revision_state.revision_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all revisions for the part
    let revisions = revision_manager.get_part_revisions(part_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let revision_dtos = revisions.into_iter()
        .map(RevisionDto::from)
        .collect();
    
    Ok(revision_dtos)
}

/// Get the latest revision for a part
#[command]
pub async fn get_latest_revision(
    part_id: i64,
    revision_state: State<'_, RevisionState>,
) -> Result<RevisionDto, String> {
    let revision_manager = revision_state.revision_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the latest revision for the part
    let revision = revision_manager.get_latest_revision(part_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(RevisionDto::from(revision))
}

/// Get the latest released revision for a part
#[command]
pub async fn get_latest_released_revision(
    part_id: i64,
    revision_state: State<'_, RevisionState>,
) -> Result<RevisionDto, String> {
    let revision_manager = revision_state.revision_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the latest released revision for the part
    let revision = revision_manager.get_latest_released_revision(part_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(RevisionDto::from(revision))
}

/// Create a new revision
#[command]
pub async fn create_revision(
    revision_data: RevisionCreationData,
    revision_state: State<'_, RevisionState>,
) -> Result<RevisionDto, String> {
    let revision_manager = revision_state.revision_manager.lock().map_err(|e| e.to_string())?;
    
    // Convert status string to RevisionStatus
    let status = match revision_data.status.as_str() {
        "Draft" => RevisionStatus::Draft,
        "InReview" => RevisionStatus::InReview,
        "Released" => RevisionStatus::Released,
        "Obsolete" => RevisionStatus::Obsolete,
        _ => return Err(format!("Invalid revision status: {}", revision_data.status)),
    };
    
    // Create a new revision
    let revision = Revision::new(
        revision_data.part_id,
        revision_data.version,
        status,
        revision_data.created_by,
        revision_data.commit_hash,
    );
    
    // Save the revision
    let revision_id = revision_manager.create_revision(&revision)
        .map_err(|e| e.to_string())?;
    
    // Get the created revision
    let created_revision = revision_manager.get_revision(revision_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(RevisionDto::from(created_revision))
}

/// Update a revision
#[command]
pub async fn update_revision(
    revision_id: i64,
    revision_data: RevisionCreationData,
    revision_state: State<'_, RevisionState>,
) -> Result<RevisionDto, String> {
    let revision_manager = revision_state.revision_manager.lock().map_err(|e| e.to_string())?;
    
    // Convert status string to RevisionStatus
    let status = match revision_data.status.as_str() {
        "Draft" => RevisionStatus::Draft,
        "InReview" => RevisionStatus::InReview,
        "Released" => RevisionStatus::Released,
        "Obsolete" => RevisionStatus::Obsolete,
        _ => return Err(format!("Invalid revision status: {}", revision_data.status)),
    };
    
    // Get the existing revision
    let mut revision = revision_manager.get_revision(revision_id)
        .map_err(|e| e.to_string())?;
    
    // Update the revision
    revision.part_id = revision_data.part_id;
    revision.version = revision_data.version;
    revision.status = status;
    revision.created_by = revision_data.created_by;
    revision.commit_hash = revision_data.commit_hash;
    
    // Save the updated revision
    revision_manager.update_revision(&revision)
        .map_err(|e| e.to_string())?;
    
    // Get the updated revision
    let updated_revision = revision_manager.get_revision(revision_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(RevisionDto::from(updated_revision))
}

/// Update revision status
#[command]
pub async fn update_revision_status(
    revision_id: i64,
    status: String,
    revision_state: State<'_, RevisionState>,
) -> Result<RevisionDto, String> {
    let revision_manager = revision_state.revision_manager.lock().map_err(|e| e.to_string())?;
    
    // Convert status string to RevisionStatus
    let status_enum = match status.as_str() {
        "Draft" => RevisionStatus::Draft,
        "InReview" => RevisionStatus::InReview,
        "Released" => RevisionStatus::Released,
        "Obsolete" => RevisionStatus::Obsolete,
        _ => return Err(format!("Invalid revision status: {}", status)),
    };
    
    // Update the revision status
    revision_manager.update_revision_status(revision_id, status_enum)
        .map_err(|e| e.to_string())?;
    
    // Get the updated revision
    let updated_revision = revision_manager.get_revision(revision_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(RevisionDto::from(updated_revision))
}

/// Delete a revision
#[command]
pub async fn delete_revision(
    revision_id: i64,
    revision_state: State<'_, RevisionState>,
) -> Result<(), String> {
    let revision_manager = revision_state.revision_manager.lock().map_err(|e| e.to_string())?;
    
    // Delete the revision
    revision_manager.delete_revision(revision_id)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}