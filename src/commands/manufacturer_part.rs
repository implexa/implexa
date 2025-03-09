//! Tauri command handlers for manufacturer part operations
//!
//! This module contains the command handlers for manufacturer part operations in the Tauri application.
//! These commands are exposed to the frontend and allow it to interact with the manufacturer part management system.

use std::sync::Mutex;
use tauri::{command, State};
use serde::{Serialize, Deserialize};
use crate::database::manufacturer_part::{ManufacturerPartManager, ManufacturerPart, ManufacturerPartStatus};
use crate::database::connection_manager::ConnectionManager;

/// Manufacturer part information for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturerPartDto {
    /// Manufacturer part ID
    pub mpn_id: i64,
    /// Part ID this manufacturer part is associated with
    pub part_id: i64,
    /// Manufacturer name
    pub manufacturer: String,
    /// Manufacturer part number
    pub mpn: String,
    /// Description of the manufacturer part
    pub description: Option<String>,
    /// Status of the manufacturer part (Active, Preferred, Alternate, Obsolete)
    pub status: String,
}

/// Manufacturer part creation data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturerPartCreationData {
    /// Part ID this manufacturer part is associated with
    pub part_id: i64,
    /// Manufacturer name
    pub manufacturer: String,
    /// Manufacturer part number
    pub mpn: String,
    /// Description of the manufacturer part
    pub description: Option<String>,
    /// Status of the manufacturer part (Active, Preferred, Alternate, Obsolete)
    pub status: String,
}

/// Manufacturer part state for the application
pub struct ManufacturerPartState {
    /// Connection manager for the database
    pub connection_manager: ConnectionManager,
    /// Manufacturer part manager for manufacturer part operations
    pub manufacturer_part_manager: Mutex<ManufacturerPartManager<'static>>,
}

impl From<ManufacturerPart> for ManufacturerPartDto {
    fn from(mpn: ManufacturerPart) -> Self {
        Self {
            mpn_id: mpn.mpn_id.unwrap_or_default(),
            part_id: mpn.part_id,
            manufacturer: mpn.manufacturer,
            mpn: mpn.mpn,
            description: mpn.description,
            status: mpn.status.to_str().to_string(),
        }
    }
}

/// Initialize the manufacturer part state
pub fn init_manufacturer_part_state(connection_manager: ConnectionManager) -> ManufacturerPartState {
    // Create a manufacturer part manager with 'static lifetime using a leak (safe in this context)
    let static_connection_manager: &'static ConnectionManager = Box::leak(Box::new(connection_manager.clone()));
    let manufacturer_part_manager = ManufacturerPartManager::new(static_connection_manager);
    
    ManufacturerPartState {
        connection_manager,
        manufacturer_part_manager: Mutex::new(manufacturer_part_manager),
    }
}

/// Get a manufacturer part by ID
#[command]
pub async fn get_manufacturer_part(
    mpn_id: i64,
    manufacturer_part_state: State<'_, ManufacturerPartState>,
) -> Result<ManufacturerPartDto, String> {
    let manufacturer_part_manager = manufacturer_part_state.manufacturer_part_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the manufacturer part
    let mpn = manufacturer_part_manager.get_manufacturer_part(mpn_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(ManufacturerPartDto::from(mpn))
}

/// Get all manufacturer parts for a part
#[command]
pub async fn get_manufacturer_parts_for_part(
    part_id: String,
    manufacturer_part_state: State<'_, ManufacturerPartState>,
) -> Result<Vec<ManufacturerPartDto>, String> {
    let manufacturer_part_manager = manufacturer_part_state.manufacturer_part_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all manufacturer parts for the part
    let mpns = manufacturer_part_manager.get_manufacturer_parts_for_part(&part_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let mpn_dtos = mpns.into_iter()
        .map(ManufacturerPartDto::from)
        .collect();
    
    Ok(mpn_dtos)
}

/// Get manufacturer parts by MPN
#[command]
pub async fn get_manufacturer_parts_by_mpn(
    manufacturer: String,
    mpn: String,
    manufacturer_part_state: State<'_, ManufacturerPartState>,
) -> Result<Vec<ManufacturerPartDto>, String> {
    let manufacturer_part_manager = manufacturer_part_state.manufacturer_part_manager.lock().map_err(|e| e.to_string())?;
    
    // Get manufacturer parts by MPN
    let mpns = manufacturer_part_manager.get_manufacturer_parts_by_mpn(&manufacturer, &mpn)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let mpn_dtos = mpns.into_iter()
        .map(ManufacturerPartDto::from)
        .collect();
    
    Ok(mpn_dtos)
}

/// Create a new manufacturer part
#[command]
pub async fn create_manufacturer_part(
    manufacturer_part_data: ManufacturerPartCreationData,
    manufacturer_part_state: State<'_, ManufacturerPartState>,
) -> Result<ManufacturerPartDto, String> {
    let manufacturer_part_manager = manufacturer_part_state.manufacturer_part_manager.lock().map_err(|e| e.to_string())?;
    
    // Convert status string to ManufacturerPartStatus
    let status = match manufacturer_part_data.status.as_str() {
        "Active" => ManufacturerPartStatus::Active,
        "Preferred" => ManufacturerPartStatus::Preferred,
        "Alternate" => ManufacturerPartStatus::Alternate,
        "Obsolete" => ManufacturerPartStatus::Obsolete,
        _ => return Err(format!("Invalid manufacturer part status: {}", manufacturer_part_data.status)),
    };
    
    // Create a new manufacturer part
    let mpn = ManufacturerPart::new(
        manufacturer_part_data.part_id,
        manufacturer_part_data.manufacturer,
        manufacturer_part_data.mpn,
        manufacturer_part_data.description,
        status,
    );
    
    // Save the manufacturer part
    let mpn_id = manufacturer_part_manager.create_manufacturer_part(&mpn)
        .map_err(|e| e.to_string())?;
    
    // Get the created manufacturer part
    let created_mpn = manufacturer_part_manager.get_manufacturer_part(mpn_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(ManufacturerPartDto::from(created_mpn))
}

/// Update a manufacturer part
#[command]
pub async fn update_manufacturer_part(
    mpn_id: i64,
    manufacturer_part_data: ManufacturerPartCreationData,
    manufacturer_part_state: State<'_, ManufacturerPartState>,
) -> Result<ManufacturerPartDto, String> {
    let manufacturer_part_manager = manufacturer_part_state.manufacturer_part_manager.lock().map_err(|e| e.to_string())?;
    
    // Convert status string to ManufacturerPartStatus
    let status = match manufacturer_part_data.status.as_str() {
        "Active" => ManufacturerPartStatus::Active,
        "Preferred" => ManufacturerPartStatus::Preferred,
        "Alternate" => ManufacturerPartStatus::Alternate,
        "Obsolete" => ManufacturerPartStatus::Obsolete,
        _ => return Err(format!("Invalid manufacturer part status: {}", manufacturer_part_data.status)),
    };
    
    // Create updated manufacturer part
    let mut mpn = ManufacturerPart::new(
        manufacturer_part_data.part_id,
        manufacturer_part_data.manufacturer,
        manufacturer_part_data.mpn,
        manufacturer_part_data.description,
        status,
    );
    mpn.mpn_id = Some(mpn_id);
    
    // Update the manufacturer part
    manufacturer_part_manager.update_manufacturer_part(&mpn)
        .map_err(|e| e.to_string())?;
    
    // Get the updated manufacturer part
    let updated_mpn = manufacturer_part_manager.get_manufacturer_part(mpn_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(ManufacturerPartDto::from(updated_mpn))
}

/// Delete a manufacturer part
#[command]
pub async fn delete_manufacturer_part(
    mpn_id: i64,
    manufacturer_part_state: State<'_, ManufacturerPartState>,
) -> Result<(), String> {
    let manufacturer_part_manager = manufacturer_part_state.manufacturer_part_manager.lock().map_err(|e| e.to_string())?;
    
    // Delete the manufacturer part
    manufacturer_part_manager.delete_manufacturer_part(mpn_id)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Search manufacturer parts
#[command]
pub async fn search_manufacturer_parts(
    search_term: String,
    manufacturer_part_state: State<'_, ManufacturerPartState>,
) -> Result<Vec<ManufacturerPartDto>, String> {
    let manufacturer_part_manager = manufacturer_part_state.manufacturer_part_manager.lock().map_err(|e| e.to_string())?;
    
    // Search manufacturer parts
    let mpns = manufacturer_part_manager.search_manufacturer_parts(&search_term)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let mpn_dtos = mpns.into_iter()
        .map(ManufacturerPartDto::from)
        .collect();
    
    Ok(mpn_dtos)
}