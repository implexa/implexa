//! Repository command handlers
//!
//! This module contains the command handlers for Git repository operations.
//! These commands are exposed to the frontend and allow it to interact with the backend.

use std::path::Path;
use std::sync::Mutex;
use tauri::{command, State};
use serde::{Serialize, Deserialize};
use crate::git_backend::directory::TemplateType;
use crate::{GitBackendManager, GitBackendConfig, AuthConfig, RepositoryInfo};

/// Repository information for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryDto {
    /// Repository ID (path hash)
    pub id: String,
    /// Repository path
    pub path: String,
    /// Repository name
    pub name: String,
    /// Current branch
    pub current_branch: String,
    /// Whether the repository has uncommitted changes
    pub has_changes: bool,
    /// Whether LFS is enabled
    pub lfs_enabled: bool,
}

impl From<RepositoryInfo> for RepositoryDto {
    fn from(info: RepositoryInfo) -> Self {
        let path_str = info.path.to_string_lossy().to_string();
        let name = Path::new(&path_str)
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        
        Self {
            id: format!("{:x}", md5::compute(&path_str)),
            path: path_str,
            name,
            current_branch: info.current_branch,
            has_changes: info.has_changes,
            lfs_enabled: info.lfs_enabled,
        }
    }
}

/// Git backend manager state
pub struct GitBackendState {
    /// Git backend manager
    pub manager: Mutex<GitBackendManager>,
}

use crate::commands::parts::DatabaseState;
use crate::database::connection_manager::ConnectionManager;

/// Create a new repository
#[command]
pub async fn create_repository(
    path: String,
    template_type: String,
    git_state: State<'_, GitBackendState>,
    _db_state: State<'_, DatabaseState>,
) -> Result<RepositoryDto, String> {
    let manager = git_state.manager.lock().map_err(|e| e.to_string())?;
    
    // Initialize the repository
    let repo = manager
        .init_repository(Path::new(&path))
        .map_err(|e| e.to_string())?;
    
    // Get the repository manager
    let repo_manager = manager.repository_manager(&repo);
    
    // Set up the PLM structure
    repo_manager.setup_plm_structure().map_err(|e| e.to_string())?;
    
    // Create a part directory with the specified template
    let _template_type_enum = match template_type.as_str() {
        "minimal" => TemplateType::Minimal,
        "extended" => TemplateType::Extended,
        _ => TemplateType::Standard,
    };
    
    // Get repository info
    let info = repo_manager.get_info().map_err(|e| e.to_string())?;
    
    // Create the main repository database in the config directory
    let config_dir = Path::new(&path).join("config");
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    
    let db_path = config_dir.join("repository.db");
    println!("Creating repository database at: {}", db_path.display());
    
    // TODO: We should update the DatabaseState to use this new connection
    // For now, we'll just create the database file
    let _connection_manager = ConnectionManager::new(&db_path)
        .map_err(|e| format!("Failed to create repository database: {}", e))?;
    
    Ok(RepositoryDto::from(info))
}

/// Open an existing repository
#[command]
pub async fn open_repository(
    path: String,
    git_state: State<'_, GitBackendState>,
    _db_state: State<'_, DatabaseState>,
) -> Result<RepositoryDto, String> {
    let manager = git_state.manager.lock().map_err(|e| e.to_string())?;
    
    // Open the repository
    let repo = manager
        .open_repository(Path::new(&path))
        .map_err(|e| e.to_string())?;
    
    // Get repository info
    let repo_manager = manager.repository_manager(&repo);
    let info = repo_manager.get_info().map_err(|e| e.to_string())?;
    
    // Check for the repository database in the config directory
    let config_dir = Path::new(&path).join("config");
    let db_path = config_dir.join("repository.db");
    
    if db_path.exists() {
        println!("Using existing repository database at: {}", db_path.display());
        // TODO: We should update the DatabaseState to use this existing connection
        let _connection_manager = ConnectionManager::new(&db_path)
            .map_err(|e| format!("Failed to open repository database: {}", e))?;
    } else {
        // Create config directory if it doesn't exist
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }
        
        // Create a new repository database
        println!("Creating repository database at: {}", db_path.display());
        let _connection_manager = ConnectionManager::new(&db_path)
            .map_err(|e| format!("Failed to create repository database: {}", e))?;
    }
    
    Ok(RepositoryDto::from(info))
}

/// Close a repository
#[command]
pub async fn close_repository(
    _path: String,
    _git_state: State<'_, GitBackendState>,
) -> Result<(), String> {
    // Nothing to do here, as the repository is closed automatically when it goes out of scope
    Ok(())
}

/// Get repository information
#[command]
pub async fn get_repository_info(
    path: String,
    git_state: State<'_, GitBackendState>,
    _db_state: State<'_, DatabaseState>,
) -> Result<RepositoryDto, String> {
    let manager = git_state.manager.lock().map_err(|e| e.to_string())?;
    
    // Open the repository
    let repo = manager
        .open_repository(Path::new(&path))
        .map_err(|e| e.to_string())?;
    
    // Get repository info
    let repo_manager = manager.repository_manager(&repo);
    let info = repo_manager.get_info().map_err(|e| e.to_string())?;
    
    // Check for database location
    let config_dir = Path::new(&path).join("config");
    let db_path = config_dir.join("repository.db");
    
    if db_path.exists() {
        println!("Found repository database at: {}", db_path.display());
        // TODO: Switch to this database connection
    }
    
    Ok(RepositoryDto::from(info))
}

/// Initialize the Git backend state
pub fn init_git_backend() -> GitBackendState {
    let config = GitBackendConfig::default();
    let auth_config = AuthConfig::default();
    
    let manager = GitBackendManager::new(config, auth_config)
        .expect("Failed to create Git backend manager");
    
    GitBackendState {
        manager: Mutex::new(manager),
    }
}