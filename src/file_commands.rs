//! Tauri command handlers for file operations
//!
//! This module contains the command handlers for file-related operations in the Tauri application.
//! These commands are exposed to the frontend and allow it to interact with the file management system.

use std::sync::Mutex;
use std::path::PathBuf;
use tauri::{command, State};
use serde::{Serialize, Deserialize};
use implexa::database::file::{FileManager, File, FileType};
use implexa::database::connection_manager::ConnectionManager;

/// File information for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDto {
    /// File ID
    pub file_id: i64,
    /// Part ID this file is associated with (if applicable)
    pub part_id: Option<i64>,
    /// Revision ID this file is associated with (if applicable)
    pub revision_id: Option<i64>,
    /// Path to the file
    pub path: String,
    /// Type of the file (Design, Documentation, Manufacturing, Test, Image, Model3D, SourceCode, Other)
    pub file_type: String,
    /// Description of the file
    pub description: Option<String>,
}

/// File creation data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCreationData {
    /// Part ID this file is associated with (if applicable)
    pub part_id: Option<i64>,
    /// Revision ID this file is associated with (if applicable)
    pub revision_id: Option<i64>,
    /// Path to the file
    pub path: String,
    /// Type of the file (Design, Documentation, Manufacturing, Test, Image, Model3D, SourceCode, Other)
    pub file_type: String,
    /// Description of the file
    pub description: Option<String>,
}

/// File state for the application
pub struct FileState {
    /// Connection manager for the database
    pub connection_manager: ConnectionManager,
    /// File manager for file operations
    pub file_manager: Mutex<FileManager<'static>>,
}

impl From<File> for FileDto {
    fn from(file: File) -> Self {
        Self {
            file_id: file.file_id.unwrap_or_default(),
            part_id: file.part_id,
            revision_id: file.revision_id,
            path: file.path.to_string_lossy().to_string(),
            file_type: file.file_type.to_str(),
            description: file.description,
        }
    }
}

/// Initialize the file state
pub fn init_file_state(connection_manager: ConnectionManager) -> FileState {
    // Create a file manager with 'static lifetime using a leak (safe in this context)
    let static_connection_manager: &'static ConnectionManager = Box::leak(Box::new(connection_manager.clone()));
    let file_manager = FileManager::new(static_connection_manager);
    
    FileState {
        connection_manager,
        file_manager: Mutex::new(file_manager),
    }
}

/// Get a file by ID
#[command]
pub async fn get_file(
    file_id: i64,
    file_state: State<'_, FileState>,
) -> Result<FileDto, String> {
    let file_manager = file_state.file_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the file
    let file = file_manager.get_file(file_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(FileDto::from(file))
}

/// Get all files for a part
#[command]
pub async fn get_part_files(
    part_id: String,
    file_state: State<'_, FileState>,
) -> Result<Vec<FileDto>, String> {
    let file_manager = file_state.file_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all files for the part
    let files = file_manager.get_part_files(&part_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let file_dtos = files.into_iter()
        .map(FileDto::from)
        .collect();
    
    Ok(file_dtos)
}

/// Get all files for a revision
#[command]
pub async fn get_revision_files(
    revision_id: i64,
    file_state: State<'_, FileState>,
) -> Result<Vec<FileDto>, String> {
    let file_manager = file_state.file_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all files for the revision
    let files = file_manager.get_revision_files(revision_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let file_dtos = files.into_iter()
        .map(FileDto::from)
        .collect();
    
    Ok(file_dtos)
}

/// Get files by type for a part
#[command]
pub async fn get_part_files_by_type(
    part_id: String,
    file_type: String,
    file_state: State<'_, FileState>,
) -> Result<Vec<FileDto>, String> {
    let file_manager = file_state.file_manager.lock().map_err(|e| e.to_string())?;
    
    // Convert file type string to FileType
    let file_type_enum = FileType::from_str(&file_type);
    
    // Get files by type for the part
    let files = file_manager.get_part_files_by_type(&part_id, &file_type_enum)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let file_dtos = files.into_iter()
        .map(FileDto::from)
        .collect();
    
    Ok(file_dtos)
}

/// Get files by type for a revision
#[command]
pub async fn get_revision_files_by_type(
    revision_id: i64,
    file_type: String,
    file_state: State<'_, FileState>,
) -> Result<Vec<FileDto>, String> {
    let file_manager = file_state.file_manager.lock().map_err(|e| e.to_string())?;
    
    // Convert file type string to FileType
    let file_type_enum = FileType::from_str(&file_type);
    
    // Get files by type for the revision
    let files = file_manager.get_revision_files_by_type(revision_id, &file_type_enum)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let file_dtos = files.into_iter()
        .map(FileDto::from)
        .collect();
    
    Ok(file_dtos)
}

/// Create a new file
#[command]
pub async fn create_file(
    file_data: FileCreationData,
    file_state: State<'_, FileState>,
) -> Result<FileDto, String> {
    let file_manager = file_state.file_manager.lock().map_err(|e| e.to_string())?;
    
    // Convert file type string to FileType
    let file_type = FileType::from_str(&file_data.file_type);
    
    // Create a new file
    let file = if let Some(part_id) = file_data.part_id {
        File::new_part_file(
            part_id,
            PathBuf::from(file_data.path),
            file_type,
            file_data.description,
        )
    } else if let Some(revision_id) = file_data.revision_id {
        File::new_revision_file(
            revision_id,
            PathBuf::from(file_data.path),
            file_type,
            file_data.description,
        )
    } else {
        return Err("Either part_id or revision_id must be provided".to_string());
    };
    
    // Save the file
    let file_id = file_manager.create_file(&file)
        .map_err(|e| e.to_string())?;
    
    // Get the created file
    let created_file = file_manager.get_file(file_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(FileDto::from(created_file))
}

/// Update a file
#[command]
pub async fn update_file(
    file_id: i64,
    file_data: FileCreationData,
    file_state: State<'_, FileState>,
) -> Result<FileDto, String> {
    let file_manager = file_state.file_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the existing file
    let mut file = file_manager.get_file(file_id)
        .map_err(|e| e.to_string())?;
    
    // Convert file type string to FileType
    let file_type = FileType::from_str(&file_data.file_type);
    
    // Update the file
    file.part_id = file_data.part_id;
    file.revision_id = file_data.revision_id;
    file.path = PathBuf::from(file_data.path);
    file.file_type = file_type;
    file.description = file_data.description;
    
    // Save the updated file
    file_manager.update_file(&file)
        .map_err(|e| e.to_string())?;
    
    // Get the updated file
    let updated_file = file_manager.get_file(file_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(FileDto::from(updated_file))
}

/// Delete a file
#[command]
pub async fn delete_file(
    file_id: i64,
    file_state: State<'_, FileState>,
) -> Result<(), String> {
    let file_manager = file_state.file_manager.lock().map_err(|e| e.to_string())?;
    
    // Delete the file
    file_manager.delete_file(file_id)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}