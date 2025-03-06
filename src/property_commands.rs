//! Tauri command handlers for property operations
//!
//! This module contains the command handlers for property-related operations in the Tauri application.
//! These commands are exposed to the frontend and allow it to interact with the property management system.

use std::sync::Mutex;
use tauri::{command, State};
use serde::{Serialize, Deserialize};
use crate::database::property::{PropertyManager, Property, PropertyType};
use crate::database::connection_manager::ConnectionManager;

/// Property information for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyDto {
    /// Property ID
    pub property_id: i64,
    /// Part ID this property is associated with (if applicable)
    pub part_id: Option<i64>,
    /// Revision ID this property is associated with (if applicable)
    pub revision_id: Option<i64>,
    /// Key of the property
    pub key: String,
    /// Value of the property
    pub value: Option<String>,
    /// Type of the property value (string, integer, float, boolean, date, url, json)
    pub property_type: String,
}

/// Property creation data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyCreationData {
    /// Part ID this property is associated with (if applicable)
    pub part_id: Option<i64>,
    /// Revision ID this property is associated with (if applicable)
    pub revision_id: Option<i64>,
    /// Key of the property
    pub key: String,
    /// Value of the property
    pub value: Option<String>,
    /// Type of the property value (string, integer, float, boolean, date, url, json)
    pub property_type: String,
}

/// Property state for the application
pub struct PropertyState {
    /// Connection manager for the database
    pub connection_manager: ConnectionManager,
    /// Property manager for property operations
    pub property_manager: Mutex<PropertyManager<'static>>,
}

impl From<Property> for PropertyDto {
    fn from(property: Property) -> Self {
        Self {
            property_id: property.property_id.unwrap_or_default(),
            part_id: property.part_id,
            revision_id: property.revision_id,
            key: property.key,
            value: property.value,
            property_type: property.property_type.to_str().to_string(),
        }
    }
}

/// Initialize the property state
pub fn init_property_state(connection_manager: ConnectionManager) -> PropertyState {
    // Create a property manager with 'static lifetime using a leak (safe in this context)
    let static_connection_manager: &'static ConnectionManager = Box::leak(Box::new(connection_manager.clone()));
    let property_manager = PropertyManager::new(static_connection_manager);
    
    PropertyState {
        connection_manager,
        property_manager: Mutex::new(property_manager),
    }
}

/// Get a property by ID
#[command]
pub async fn get_property(
    property_id: i64,
    property_state: State<'_, PropertyState>,
) -> Result<PropertyDto, String> {
    let property_manager = property_state.property_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the property
    let property = property_manager.get_property(property_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(PropertyDto::from(property))
}

/// Get all properties for a part
#[command]
pub async fn get_part_properties(
    part_id: String,
    property_state: State<'_, PropertyState>,
) -> Result<Vec<PropertyDto>, String> {
    let property_manager = property_state.property_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all properties for the part
    let properties = property_manager.get_part_properties(&part_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let property_dtos = properties.into_iter()
        .map(PropertyDto::from)
        .collect();
    
    Ok(property_dtos)
}

/// Get all properties for a revision
#[command]
pub async fn get_revision_properties(
    revision_id: i64,
    property_state: State<'_, PropertyState>,
) -> Result<Vec<PropertyDto>, String> {
    let property_manager = property_state.property_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all properties for the revision
    let properties = property_manager.get_revision_properties(revision_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let property_dtos = properties.into_iter()
        .map(PropertyDto::from)
        .collect();
    
    Ok(property_dtos)
}

/// Get a specific property for a part
#[command]
pub async fn get_part_property(
    part_id: String,
    key: String,
    property_state: State<'_, PropertyState>,
) -> Result<PropertyDto, String> {
    let property_manager = property_state.property_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the property
    let property = property_manager.get_part_property(&part_id, &key)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(PropertyDto::from(property))
}

/// Get a specific property for a revision
#[command]
pub async fn get_revision_property(
    revision_id: i64,
    key: String,
    property_state: State<'_, PropertyState>,
) -> Result<PropertyDto, String> {
    let property_manager = property_state.property_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the property
    let property = property_manager.get_revision_property(revision_id, &key)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(PropertyDto::from(property))
}

/// Create a new property
#[command]
pub async fn create_property(
    property_data: PropertyCreationData,
    property_state: State<'_, PropertyState>,
) -> Result<PropertyDto, String> {
    let property_manager = property_state.property_manager.lock().map_err(|e| e.to_string())?;
    
    // Convert property type string to PropertyType
    let property_type = PropertyType::from_str(&property_data.property_type)
        .ok_or_else(|| format!("Invalid property type: {}", property_data.property_type))?;
    
    // Create a new property
    let property = if let Some(part_id) = property_data.part_id {
        Property::new_part_property(
            part_id,
            property_data.key,
            property_data.value,
            property_type,
        )
    } else if let Some(revision_id) = property_data.revision_id {
        Property::new_revision_property(
            revision_id,
            property_data.key,
            property_data.value,
            property_type,
        )
    } else {
        return Err("Either part_id or revision_id must be provided".to_string());
    };
    
    // Save the property
    let property_id = property_manager.create_property(&property)
        .map_err(|e| e.to_string())?;
    
    // Get the created property
    let created_property = property_manager.get_property(property_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(PropertyDto::from(created_property))
}

/// Update a property
#[command]
pub async fn update_property(
    property_id: i64,
    property_data: PropertyCreationData,
    property_state: State<'_, PropertyState>,
) -> Result<PropertyDto, String> {
    let property_manager = property_state.property_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the existing property
    let mut property = property_manager.get_property(property_id)
        .map_err(|e| e.to_string())?;
    
    // Convert property type string to PropertyType
    let property_type = PropertyType::from_str(&property_data.property_type)
        .ok_or_else(|| format!("Invalid property type: {}", property_data.property_type))?;
    
    // Update the property
    property.part_id = property_data.part_id;
    property.revision_id = property_data.revision_id;
    property.key = property_data.key;
    property.value = property_data.value;
    property.property_type = property_type;
    
    // Save the updated property
    property_manager.update_property(&property)
        .map_err(|e| e.to_string())?;
    
    // Get the updated property
    let updated_property = property_manager.get_property(property_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(PropertyDto::from(updated_property))
}

/// Delete a property
#[command]
pub async fn delete_property(
    property_id: i64,
    property_state: State<'_, PropertyState>,
) -> Result<(), String> {
    let property_manager = property_state.property_manager.lock().map_err(|e| e.to_string())?;
    
    // Delete the property
    property_manager.delete_property(property_id)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}