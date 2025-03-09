//! Tauri command handlers for part operations
//!
//! This module contains the command handlers for part-related operations in the Tauri application.
//! These commands are exposed to the frontend and allow it to interact with the part management system.

use std::sync::Mutex;
use tauri::{command, State};
use serde::{Serialize, Deserialize};
use implexa::database::part_management::PartManagementManager;
use implexa::database::part::Part;
use implexa::database::connection_manager::ConnectionManager;

/// Part information for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartDto {
    /// Part ID
    pub id: i64,
    /// Part number (display format)
    pub part_number: String,
    /// Part name
    pub name: String,
    /// Part description
    pub description: Option<String>,
    /// Part status
    pub status: String,
    /// Part category
    pub category: String,
    /// Part subcategory
    pub subcategory: String,
}

/// Part creation data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartCreateDto {
    /// Part name
    pub name: String,
    /// Part description
    pub description: Option<String>,
    /// Part category
    pub category: String,
    /// Part subcategory
    pub subcategory: String,
}

/// Database state for part operations
pub struct DatabaseState {
    /// Connection manager
    pub connection_manager: ConnectionManager,
    /// Part management manager
    pub part_manager: Mutex<PartManagementManager<'static>>,
}

impl From<Part> for PartDto {
    fn from(part: Part) -> Self {
        // Get the default status
        let status = "Draft".to_string(); // This would typically come from the revision
        
        // Create the DTO
        Self {
            id: part.part_id,
            part_number: format!("{}-{}-{}",
                part.category.chars().take(2).collect::<String>().to_uppercase(),
                part.subcategory.chars().take(3).collect::<String>().to_uppercase(),
                part.part_id),
            name: part.name,
            description: part.description,
            status,
            category: part.category,
            subcategory: part.subcategory,
        }
    }
}

/// Get all parts
#[command]
pub async fn get_parts(
    db_state: State<'_, DatabaseState>,
) -> Result<Vec<PartDto>, String> {
    let part_manager = db_state.part_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all parts
    let parts = part_manager.get_all_parts()
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let part_dtos = parts.into_iter()
        .map(PartDto::from)
        .collect();
    
    Ok(part_dtos)
}

/// Get a specific part by ID
#[command]
pub async fn get_part(
    part_id: i64,
    db_state: State<'_, DatabaseState>,
) -> Result<PartDto, String> {
    let part_manager = db_state.part_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the part
    let part = part_manager.get_part(part_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(PartDto::from(part))
}

/// Create a new part
#[command]
pub async fn create_part(
    part_data: PartCreateDto,
    db_state: State<'_, DatabaseState>,
) -> Result<PartDto, String> {
    let mut part_manager = db_state.part_manager.lock().map_err(|e| e.to_string())?;
    
    // Create a new part
    let part = Part::new(
        0, // part_id will be set by the database
        part_data.category,
        part_data.subcategory,
        part_data.name,
        part_data.description
    );
    
    // Save the part
    let created_part = part_manager.create_part(&part)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(PartDto::from(created_part))
}

/// Update an existing part
#[command]
pub async fn update_part(
    part_id: i64,
    part_data: PartCreateDto,
    db_state: State<'_, DatabaseState>,
) -> Result<PartDto, String> {
    let mut part_manager = db_state.part_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the current part
    let mut part = part_manager.get_part(part_id)
        .map_err(|e| e.to_string())?;
    
    // We need to create a new part since Part's fields are not directly mutable
    let updated_part = Part::new(
        part.part_id,
        part_data.category,
        part_data.subcategory,
        part_data.name,
        part_data.description
    );
    
    // Save the updated part
    part_manager.update_part(&updated_part)
        .map_err(|e| e.to_string())?;
    
    // Get the updated part
    let updated_part = part_manager.get_part(part_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(PartDto::from(updated_part))
}

/// Change the status of a part
#[command]
pub async fn change_part_status(
    part_id: i64,
    status: String,
    db_state: State<'_, DatabaseState>,
) -> Result<(), String> {
    let part_manager = db_state.part_manager.lock().map_err(|e| e.to_string())?;
    
    // Status is actually tracked in the Revision table, not on the Part itself.
    // This would normally be handled by getting the latest revision for the part
    // and updating its status. For now, we'll just log that this needs to be implemented.
    println!("Changing status for part {} to {} - implementation needed", part_id, status);
    
    // Return success for now
    Ok(())
}

/// Delete a part
#[command]
pub async fn delete_part(
    part_id: i64,
    db_state: State<'_, DatabaseState>,
) -> Result<(), String> {
    let mut part_manager = db_state.part_manager.lock().map_err(|e| e.to_string())?;
    
    // Delete the part
    part_manager.delete_part(part_id)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Initialize the database state
pub fn init_database_state() -> DatabaseState {
    // Get or create a database file path
    let app_data_dir = std::env::current_dir().expect("Failed to get current directory");
    let db_path = app_data_dir.join("implexa.db");
    
    // Create the connection manager
    let connection_manager = ConnectionManager::new(&db_path)
        .expect("Failed to create database connection manager");
        
    // Initialize the git backend manager
    use implexa::git_backend::{GitBackendManager, GitBackendConfig, AuthConfig};
    let git_config = GitBackendConfig::default();
    let auth_config = AuthConfig::default();
    let git_manager = GitBackendManager::new(git_config, auth_config)
        .expect("Failed to create git backend manager");
    
    // Create a default user for system operations
    use implexa::database::part_management::{User, UserRole};
    let system_user = User::new("system".to_string(), UserRole::Admin);
    
    // Create the part manager with 'static lifetime
    let static_connection_manager: &'static ConnectionManager = Box::leak(Box::new(connection_manager.clone()));
    let static_git_manager: &'static GitBackendManager = Box::leak(Box::new(git_manager));
    let part_manager = PartManagementManager::new(
        static_connection_manager,
        static_git_manager,
        system_user
    );
    
    DatabaseState {
        connection_manager,
        part_manager: Mutex::new(part_manager),
    }
}