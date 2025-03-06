//! Tauri command handlers for workspace operations
//!
//! This module contains the command handlers for workspace-related operations in the Tauri application.
//! These commands are exposed to the frontend and allow it to interact with the workspace management system.

use std::sync::Mutex;
use tauri::{command, State};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Workspace data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    /// Workspace ID
    pub id: String,
    /// Workspace name
    pub name: String,
    /// Workspace description
    pub description: String,
    /// Creation timestamp
    pub created_at: String,
    /// Last update timestamp
    pub updated_at: String,
    /// Parts in this workspace
    pub parts: Vec<WorkspacePart>,
}

/// Part reference in a workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspacePart {
    /// Part ID
    pub id: i64,
    /// Part number
    pub part_number: String,
    /// Part name
    pub name: String,
    /// Part status
    pub status: String,
}

/// Workspace creation data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceCreationData {
    /// Workspace name
    pub name: String,
    /// Workspace description
    pub description: String,
}

/// Workspace update data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceUpdateData {
    /// Workspace name (optional)
    pub name: Option<String>,
    /// Workspace description (optional)
    pub description: Option<String>,
}

/// Workspace state for in-memory storage
/// Note: In a real implementation, this would use the database
pub struct WorkspaceState {
    /// Map of workspace ID to workspace data
    pub workspaces: Mutex<HashMap<String, Workspace>>,
    /// Next workspace ID counter
    pub next_id: Mutex<u64>,
}

impl WorkspaceState {
    /// Create a new workspace state
    pub fn new() -> Self {
        Self {
            workspaces: Mutex::new(HashMap::new()),
            next_id: Mutex::new(1),
        }
    }

    /// Get the next workspace ID
    fn get_next_id(&self) -> String {
        let mut id = self.next_id.lock().unwrap();
        let current = *id;
        *id += 1;
        format!("ws-{}", current)
    }
}

/// Get all workspaces
#[command]
pub async fn get_workspaces(
    workspace_state: State<'_, WorkspaceState>,
) -> Result<Vec<Workspace>, String> {
    let workspaces = workspace_state.workspaces.lock().map_err(|e| e.to_string())?;
    Ok(workspaces.values().cloned().collect())
}

/// Get a specific workspace by ID
#[command]
pub async fn get_workspace(
    workspace_id: String,
    workspace_state: State<'_, WorkspaceState>,
) -> Result<Workspace, String> {
    let workspaces = workspace_state.workspaces.lock().map_err(|e| e.to_string())?;
    
    workspaces.get(&workspace_id)
        .cloned()
        .ok_or_else(|| format!("Workspace not found: {}", workspace_id))
}

/// Create a new workspace
#[command]
pub async fn create_workspace(
    workspace_data: WorkspaceCreationData,
    workspace_state: State<'_, WorkspaceState>,
) -> Result<Workspace, String> {
    let id = workspace_state.get_next_id();
    let now = Utc::now().to_rfc3339();
    
    let workspace = Workspace {
        id,
        name: workspace_data.name,
        description: workspace_data.description,
        created_at: now.clone(),
        updated_at: now,
        parts: Vec::new(),
    };
    
    let mut workspaces = workspace_state.workspaces.lock().map_err(|e| e.to_string())?;
    let workspace_clone = workspace.clone();
    workspaces.insert(workspace.id.clone(), workspace);
    
    Ok(workspace_clone)
}

/// Update an existing workspace
#[command]
pub async fn update_workspace(
    workspace_id: String,
    workspace_data: WorkspaceUpdateData,
    workspace_state: State<'_, WorkspaceState>,
) -> Result<Workspace, String> {
    let mut workspaces = workspace_state.workspaces.lock().map_err(|e| e.to_string())?;
    
    let workspace = workspaces.get_mut(&workspace_id)
        .ok_or_else(|| format!("Workspace not found: {}", workspace_id))?;
    
    if let Some(name) = workspace_data.name {
        workspace.name = name;
    }
    
    if let Some(description) = workspace_data.description {
        workspace.description = description;
    }
    
    workspace.updated_at = Utc::now().to_rfc3339();
    
    Ok(workspace.clone())
}

/// Delete a workspace
#[command]
pub async fn delete_workspace(
    workspace_id: String,
    workspace_state: State<'_, WorkspaceState>,
) -> Result<(), String> {
    let mut workspaces = workspace_state.workspaces.lock().map_err(|e| e.to_string())?;
    
    if workspaces.remove(&workspace_id).is_none() {
        return Err(format!("Workspace not found: {}", workspace_id));
    }
    
    Ok(())
}

/// Add a part to a workspace
#[command]
pub async fn add_part_to_workspace(
    workspace_id: String,
    part_id: i64,
    workspace_state: State<'_, WorkspaceState>,
    // In a real implementation, we would use the part_state to get the part information
) -> Result<(), String> {
    let mut workspaces = workspace_state.workspaces.lock().map_err(|e| e.to_string())?;
    
    let workspace = workspaces.get_mut(&workspace_id)
        .ok_or_else(|| format!("Workspace not found: {}", workspace_id))?;
    
    // Check if the part is already in the workspace
    if workspace.parts.iter().any(|p| p.id == part_id) {
        return Err(format!("Part {} is already in workspace {}", part_id, workspace_id));
    }
    
    // In a real implementation, we would get the part information from the database
    // For now, we'll create a dummy part
    let part = WorkspacePart {
        id: part_id,
        part_number: format!("PART-{}", part_id),
        name: format!("Part {}", part_id),
        status: "Draft".to_string(),
    };
    
    workspace.parts.push(part);
    workspace.updated_at = Utc::now().to_rfc3339();
    
    Ok(())
}

/// Remove a part from a workspace
#[command]
pub async fn remove_part_from_workspace(
    workspace_id: String,
    part_id: i64,
    workspace_state: State<'_, WorkspaceState>,
) -> Result<(), String> {
    let mut workspaces = workspace_state.workspaces.lock().map_err(|e| e.to_string())?;
    
    let workspace = workspaces.get_mut(&workspace_id)
        .ok_or_else(|| format!("Workspace not found: {}", workspace_id))?;
    
    // Find the index of the part in the workspace
    let part_index = workspace.parts.iter().position(|p| p.id == part_id)
        .ok_or_else(|| format!("Part {} not found in workspace {}", part_id, workspace_id))?;
    
    // Remove the part from the workspace
    workspace.parts.remove(part_index);
    workspace.updated_at = Utc::now().to_rfc3339();
    
    Ok(())
}

/// Initialize the workspace state
pub fn init_workspace_state() -> WorkspaceState {
    WorkspaceState::new()
}