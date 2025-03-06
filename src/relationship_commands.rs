//! Tauri command handlers for relationship operations
//!
//! This module contains the command handlers for relationship-related operations in the Tauri application.
//! These commands are exposed to the frontend and allow it to interact with the part relationship management system.

use std::sync::Mutex;
use tauri::{command, State};
use serde::{Serialize, Deserialize};
use crate::database::relationship::{RelationshipManager, Relationship, RelationshipType};
use crate::database::connection_manager::ConnectionManager;

/// Relationship information for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipDto {
    /// Relationship ID
    pub relationship_id: i64,
    /// Parent part ID
    pub parent_id: i64,
    /// Child part ID
    pub child_id: i64,
    /// Type of the relationship (Assembly, Reference, etc.)
    pub relationship_type: String,
    /// Quantity of child parts in the relationship
    pub quantity: i64,
    /// Units for the quantity
    pub unit: Option<String>,
    /// Description of the relationship
    pub description: Option<String>,
}

/// Relationship creation data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipCreationData {
    /// Parent part ID
    pub parent_id: i64,
    /// Child part ID
    pub child_id: i64,
    /// Type of the relationship (Assembly, Reference, etc.)
    pub relationship_type: String,
    /// Quantity of child parts in the relationship
    pub quantity: i64,
    /// Units for the quantity
    pub unit: Option<String>,
    /// Description of the relationship
    pub description: Option<String>,
}

/// Relationship state for the application
pub struct RelationshipState {
    /// Connection manager for the database
    pub connection_manager: ConnectionManager,
    /// Relationship manager for relationship operations
    pub relationship_manager: Mutex<RelationshipManager<'static>>,
}

impl From<Relationship> for RelationshipDto {
    fn from(relationship: Relationship) -> Self {
        Self {
            relationship_id: relationship.relationship_id.unwrap_or_default(),
            parent_id: relationship.parent_id,
            child_id: relationship.child_id,
            relationship_type: relationship.relationship_type.to_str().to_string(),
            quantity: relationship.quantity,
            unit: relationship.unit,
            description: relationship.description,
        }
    }
}

/// Initialize the relationship state
pub fn init_relationship_state(connection_manager: ConnectionManager) -> RelationshipState {
    // Create a relationship manager with 'static lifetime using a leak (safe in this context)
    let static_connection_manager: &'static ConnectionManager = Box::leak(Box::new(connection_manager.clone()));
    let relationship_manager = RelationshipManager::new(static_connection_manager);
    
    RelationshipState {
        connection_manager,
        relationship_manager: Mutex::new(relationship_manager),
    }
}

/// Get a relationship by ID
#[command]
pub async fn get_relationship(
    relationship_id: i64,
    relationship_state: State<'_, RelationshipState>,
) -> Result<RelationshipDto, String> {
    let relationship_manager = relationship_state.relationship_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the relationship
    let relationship = relationship_manager.get_relationship(relationship_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(RelationshipDto::from(relationship))
}

/// Get all parent relationships for a part
#[command]
pub async fn get_parent_relationships(
    child_id: i64,
    relationship_state: State<'_, RelationshipState>,
) -> Result<Vec<RelationshipDto>, String> {
    let relationship_manager = relationship_state.relationship_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all parent relationships for the part
    let relationships = relationship_manager.get_parent_relationships(child_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let relationship_dtos = relationships.into_iter()
        .map(RelationshipDto::from)
        .collect();
    
    Ok(relationship_dtos)
}

/// Get all child relationships for a part
#[command]
pub async fn get_child_relationships(
    parent_id: i64,
    relationship_state: State<'_, RelationshipState>,
) -> Result<Vec<RelationshipDto>, String> {
    let relationship_manager = relationship_state.relationship_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all child relationships for the part
    let relationships = relationship_manager.get_child_relationships(parent_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let relationship_dtos = relationships.into_iter()
        .map(RelationshipDto::from)
        .collect();
    
    Ok(relationship_dtos)
}

/// Create a new relationship
#[command]
pub async fn create_relationship(
    relationship_data: RelationshipCreationData,
    relationship_state: State<'_, RelationshipState>,
) -> Result<RelationshipDto, String> {
    let relationship_manager = relationship_state.relationship_manager.lock().map_err(|e| e.to_string())?;
    
    // Convert relationship type string to RelationshipType
    let relationship_type = match relationship_data.relationship_type.as_str() {
        "Assembly" => RelationshipType::Assembly,
        "Reference" => RelationshipType::Reference,
        "Alternate" => RelationshipType::Alternate,
        _ => return Err(format!("Invalid relationship type: {}", relationship_data.relationship_type)),
    };
    
    // Create a new relationship
    let relationship = Relationship::new(
        relationship_data.parent_id,
        relationship_data.child_id,
        relationship_type,
        relationship_data.quantity,
        relationship_data.unit,
        relationship_data.description,
    );
    
    // Save the relationship
    let relationship_id = relationship_manager.create_relationship(&relationship)
        .map_err(|e| e.to_string())?;
    
    // Get the created relationship
    let created_relationship = relationship_manager.get_relationship(relationship_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(RelationshipDto::from(created_relationship))
}

/// Update a relationship
#[command]
pub async fn update_relationship(
    relationship_id: i64,
    relationship_data: RelationshipCreationData,
    relationship_state: State<'_, RelationshipState>,
) -> Result<RelationshipDto, String> {
    let relationship_manager = relationship_state.relationship_manager.lock().map_err(|e| e.to_string())?;
    
    // Convert relationship type string to RelationshipType
    let relationship_type = match relationship_data.relationship_type.as_str() {
        "Assembly" => RelationshipType::Assembly,
        "Reference" => RelationshipType::Reference,
        "Alternate" => RelationshipType::Alternate,
        _ => return Err(format!("Invalid relationship type: {}", relationship_data.relationship_type)),
    };
    
    // Create updated relationship
    let mut relationship = Relationship::new(
        relationship_data.parent_id,
        relationship_data.child_id,
        relationship_type,
        relationship_data.quantity,
        relationship_data.unit,
        relationship_data.description,
    );
    relationship.relationship_id = Some(relationship_id);
    
    // Update the relationship
    relationship_manager.update_relationship(&relationship)
        .map_err(|e| e.to_string())?;
    
    // Get the updated relationship
    let updated_relationship = relationship_manager.get_relationship(relationship_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(RelationshipDto::from(updated_relationship))
}

/// Delete a relationship
#[command]
pub async fn delete_relationship(
    relationship_id: i64,
    relationship_state: State<'_, RelationshipState>,
) -> Result<(), String> {
    let relationship_manager = relationship_state.relationship_manager.lock().map_err(|e| e.to_string())?;
    
    // Delete the relationship
    relationship_manager.delete_relationship(relationship_id)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}