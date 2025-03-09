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

/// Create a new repository
#[command]
pub async fn create_repository(
    path: String,
    template_type: String,
    git_state: State<'_, GitBackendState>,
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
    let _template_type = match template_type.as_str() {
        "minimal" => TemplateType::Minimal,
        "extended" => TemplateType::Extended,
        _ => TemplateType::Standard,
    };
    
    // Get repository info
    let info = repo_manager.get_info().map_err(|e| e.to_string())?;
    
    Ok(RepositoryDto::from(info))
}

/// Open an existing repository
#[command]
pub async fn open_repository(
    path: String,
    git_state: State<'_, GitBackendState>,
) -> Result<RepositoryDto, String> {
    let manager = git_state.manager.lock().map_err(|e| e.to_string())?;
    
    // Open the repository
    let repo = manager
        .open_repository(Path::new(&path))
        .map_err(|e| e.to_string())?;
    
    // Get repository info
    let repo_manager = manager.repository_manager(&repo);
    let info = repo_manager.get_info().map_err(|e| e.to_string())?;
    
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
) -> Result<RepositoryDto, String> {
    let manager = git_state.manager.lock().map_err(|e| e.to_string())?;
    
    // Open the repository
    let repo = manager
        .open_repository(Path::new(&path))
        .map_err(|e| e.to_string())?;
    
    // Get repository info
    let repo_manager = manager.repository_manager(&repo);
    let info = repo_manager.get_info().map_err(|e| e.to_string())?;
    
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