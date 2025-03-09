//! Tauri command handlers for approval operations
//!
//! This module contains the command handlers for approval-related operations in the Tauri application.
//! These commands are exposed to the frontend and allow it to interact with the approval system.

use std::sync::Mutex;
use tauri::{command, State};
use serde::{Serialize, Deserialize};
use crate::database::approval::{ApprovalManager, Approval, ApprovalStatus};
use crate::database::connection_manager::ConnectionManager;
use std::time::UNIX_EPOCH;
use chrono;

/// Approval data structure for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalDto {
    /// Approval ID
    pub approval_id: i64,
    /// Revision ID this approval is for
    pub revision_id: i64,
    /// Approver username
    pub approver: String,
    /// Status of the approval (Pending, Approved, Rejected)
    pub status: String,
    /// Date of the approval (ISO 8601 format)
    pub date: Option<String>,
    /// Comments from the approver
    pub comments: Option<String>,
}

/// Approval creation data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalCreationData {
    /// Revision ID this approval is for
    pub revision_id: i64,
    /// Approver username
    pub approver: String,
    /// Comments from the approver (optional)
    pub comments: Option<String>,
}

/// Approval update data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalUpdateData {
    /// New status (Approved, Rejected)
    pub status: String,
    /// Comments from the approver (optional)
    pub comments: Option<String>,
}

/// Approval state for the Tauri application
pub struct ApprovalState {
    /// Connection manager for the database
    pub connection_manager: ConnectionManager,
    /// Approval manager for approval operations
    pub approval_manager: Mutex<ApprovalManager<'static>>,
}

impl From<Approval> for ApprovalDto {
    fn from(approval: Approval) -> Self {
        // Convert the SystemTime to ISO 8601 format if it exists
        let date_string = approval.date.map(|date| {
            date.duration_since(UNIX_EPOCH)
                .map(|d| {
                    // Convert to milliseconds and format as ISO 8601
                    let secs = d.as_secs();
                    let millis = d.subsec_millis();
                    chrono::DateTime::<chrono::Utc>::from_timestamp(secs as i64, millis * 1_000_000)
                        .map(|dt| dt.to_rfc3339())
                        .unwrap_or_else(|| String::from("Invalid date"))
                })
                .unwrap_or_else(|_| String::from("Invalid date"))
        });

        Self {
            approval_id: approval.approval_id.unwrap_or_default(),
            revision_id: approval.revision_id,
            approver: approval.approver,
            status: approval.status.to_str().to_string(),
            date: date_string,
            comments: approval.comments,
        }
    }
}

/// Initialize the approval state
pub fn init_approval_state(connection_manager: ConnectionManager) -> ApprovalState {
    // Create an approval manager with 'static lifetime using a leak (safe in this context)
    let static_connection_manager: &'static ConnectionManager = Box::leak(Box::new(connection_manager.clone()));
    let approval_manager = ApprovalManager::new(static_connection_manager);
    
    ApprovalState {
        connection_manager,
        approval_manager: Mutex::new(approval_manager),
    }
}

/// Get an approval by its ID
#[command]
pub async fn get_approval(
    approval_id: i64,
    approval_state: State<'_, ApprovalState>,
) -> Result<ApprovalDto, String> {
    let approval_manager = approval_state.approval_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the approval
    let approval = approval_manager.get_approval(approval_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(ApprovalDto::from(approval))
}

/// Get all approvals for a revision
#[command]
pub async fn get_approvals_for_revision(
    revision_id: i64,
    approval_state: State<'_, ApprovalState>,
) -> Result<Vec<ApprovalDto>, String> {
    let approval_manager = approval_state.approval_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all approvals for the revision
    let approvals = approval_manager.get_approvals_for_revision(revision_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let approval_dtos = approvals.into_iter()
        .map(ApprovalDto::from)
        .collect();
    
    Ok(approval_dtos)
}

/// Get an approval for a specific revision and approver
#[command]
pub async fn get_approval_for_revision_and_approver(
    revision_id: i64,
    approver: String,
    approval_state: State<'_, ApprovalState>,
) -> Result<ApprovalDto, String> {
    let approval_manager = approval_state.approval_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the approval
    let approval = approval_manager.get_approval_for_revision_and_approver(revision_id, &approver)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(ApprovalDto::from(approval))
}

/// Create a new approval
#[command]
pub async fn create_approval(
    approval_data: ApprovalCreationData,
    approval_state: State<'_, ApprovalState>,
) -> Result<ApprovalDto, String> {
    let approval_manager = approval_state.approval_manager.lock().map_err(|e| e.to_string())?;
    
    // Create a new approval
    let approval = Approval::new(
        approval_data.revision_id,
        approval_data.approver,
        ApprovalStatus::Pending,
        approval_data.comments,
    );
    
    // Save the approval
    let approval_id = approval_manager.create_approval(&approval)
        .map_err(|e| e.to_string())?;
    
    // Get the created approval
    let created_approval = approval_manager.get_approval(approval_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(ApprovalDto::from(created_approval))
}

/// Update the status of an approval
#[command]
pub async fn update_approval_status(
    approval_id: i64,
    update_data: ApprovalUpdateData,
    approval_state: State<'_, ApprovalState>,
) -> Result<ApprovalDto, String> {
    let approval_manager = approval_state.approval_manager.lock().map_err(|e| e.to_string())?;
    
    // Convert the status string to ApprovalStatus
    let status = match update_data.status.as_str() {
        "Approved" => ApprovalStatus::Approved,
        "Rejected" => ApprovalStatus::Rejected,
        "Pending" => ApprovalStatus::Pending,
        _ => return Err(format!("Invalid approval status: {}", update_data.status)),
    };
    
    // Update the approval status
    approval_manager.update_status(approval_id, status, update_data.comments.as_deref())
        .map_err(|e| e.to_string())?;
    
    // Get the updated approval
    let updated_approval = approval_manager.get_approval(approval_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(ApprovalDto::from(updated_approval))
}

/// Delete an approval
#[command]
pub async fn delete_approval(
    approval_id: i64,
    approval_state: State<'_, ApprovalState>,
) -> Result<(), String> {
    let approval_manager = approval_state.approval_manager.lock().map_err(|e| e.to_string())?;
    
    // Delete the approval
    approval_manager.delete_approval(approval_id)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Check if a revision is fully approved
#[command]
pub async fn is_revision_approved(
    revision_id: i64,
    approval_state: State<'_, ApprovalState>,
) -> Result<bool, String> {
    let approval_manager = approval_state.approval_manager.lock().map_err(|e| e.to_string())?;
    
    // Check if the revision is fully approved
    let is_approved = approval_manager.is_revision_approved(revision_id)
        .map_err(|e| e.to_string())?;
    
    Ok(is_approved)
}

/// Submit a revision for approval
#[command]
pub async fn submit_for_approval(
    revision_id: i64,
    approvers: Vec<String>,
    approval_state: State<'_, ApprovalState>,
) -> Result<Vec<ApprovalDto>, String> {
    let approval_manager = approval_state.approval_manager.lock().map_err(|e| e.to_string())?;
    let mut approval_dtos = Vec::new();
    
    // Create an approval for each approver
    for approver in approvers {
        let approval = Approval::new(
            revision_id,
            approver,
            ApprovalStatus::Pending,
            None,
        );
        
        let approval_id = approval_manager.create_approval(&approval)
            .map_err(|e| e.to_string())?;
        
        let created_approval = approval_manager.get_approval(approval_id)
            .map_err(|e| e.to_string())?;
        
        approval_dtos.push(ApprovalDto::from(created_approval));
    }
    
    Ok(approval_dtos)
}

/// Approve a revision (convenience function that approves all pending approvals for the current user)
#[command]
pub async fn approve_revision(
    revision_id: i64,
    approver: String,
    comments: Option<String>,
    approval_state: State<'_, ApprovalState>,
) -> Result<ApprovalDto, String> {
    let approval_manager = approval_state.approval_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the approval for this revision and approver
    let approval = approval_manager.get_approval_for_revision_and_approver(revision_id, &approver)
        .map_err(|e| e.to_string())?;
    
    // Update the status to Approved
    approval_manager.update_status(
        approval.approval_id.unwrap(),
        ApprovalStatus::Approved,
        comments.as_deref()
    ).map_err(|e| e.to_string())?;
    
    // Get the updated approval
    let updated_approval = approval_manager.get_approval(approval.approval_id.unwrap())
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(ApprovalDto::from(updated_approval))
}

/// Reject a revision (convenience function that rejects all pending approvals for the current user)
#[command]
pub async fn reject_revision(
    revision_id: i64,
    approver: String,
    comments: Option<String>,
    approval_state: State<'_, ApprovalState>,
) -> Result<ApprovalDto, String> {
    let approval_manager = approval_state.approval_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the approval for this revision and approver
    let approval = approval_manager.get_approval_for_revision_and_approver(revision_id, &approver)
        .map_err(|e| e.to_string())?;
    
    // Update the status to Rejected
    approval_manager.update_status(
        approval.approval_id.unwrap(),
        ApprovalStatus::Rejected,
        comments.as_deref()
    ).map_err(|e| e.to_string())?;
    
    // Get the updated approval
    let updated_approval = approval_manager.get_approval(approval.approval_id.unwrap())
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(ApprovalDto::from(updated_approval))
}