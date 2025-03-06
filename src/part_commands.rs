//! Tauri command handlers for part operations
//!
//! This module contains the command handlers for part-related operations in the Tauri application.
//! These commands are exposed to the frontend and allow it to interact with the part management system.

use std::sync::Mutex;
use tauri::{command, State};
use serde::{Serialize, Deserialize};
use crate::database::part_management::PartManagementManager;
use crate::database::part::Part;
use crate::database::connection_manager::ConnectionManager;

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
    pub part_manager: Mutex<PartManagementManager>,
}

impl From<Part> for PartDto {
    fn from(part: Part) -> Self {
        Self {
            id: part.id,
            part_number: part.part_number,
            name: part.name,
            description: part.description,
            status: part.status.to_string(),
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
    let part = Part {
        id: 0, // Will be set by the database
        part_number: String::new(), // Will be generated
        name: part_data.name,
        description: part_data.description,
        status: "Draft".to_string(), // Initial status is Draft
        category: part_data.category,
        subcategory: part_data.subcategory,
    };
    
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
    
    // Update the part
    part.name = part_data.name;
    part.description = part_data.description;
    part.category = part_data.category;
    part.subcategory = part_data.subcategory;
    
    // Save the updated part
    part_manager.update_part(&part)
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
    let mut part_manager = db_state.part_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the current part
    let mut part = part_manager.get_part(part_id)
        .map_err(|e| e.to_string())?;
    
    // Update the status
    part.status = status;
    
    // Save the updated part
    part_manager.update_part(&part)
        .map_err(|e| e.to_string())?;
    
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
    let connection_manager = ConnectionManager::new();
    let part_manager = PartManagementManager::new(connection_manager.clone());
    
    DatabaseState {
        connection_manager,
        part_manager: Mutex::new(part_manager),
    }
}