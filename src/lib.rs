//! Implexa: A hardware-focused PLM/PDM solution
//!
//! Implexa is a Product Lifecycle Management (PLM) and Product Data Management (PDM)
//! solution that leverages Git for version control while remaining CAD-agnostic.
//! It bridges the gap between software engineering practices and hardware design workflows,
//! enabling efficient management of design files across multiple CAD platforms.

pub mod git_backend;

/// Re-export the Git Backend Manager for easier access
pub use git_backend::{
    GitBackendManager,
    GitBackendConfig,
    GitBackendError,
    AuthConfig,
    Credentials,
    HookType,
    ConflictStrategy,
    MergeResult,
    LfsStatus,
    RepositoryInfo,
    UiStatus,
    UiCommitResult,
    Metadata,
};

/// Version of the Implexa library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize logging for the Implexa library
pub fn init_logging() {
    env_logger::init();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_git_backend_manager_creation() {
        let config = GitBackendConfig::default();
        let auth_config = AuthConfig::default();
        
        let manager = GitBackendManager::new(config, auth_config);
        assert!(manager.is_ok());
    }

    #[test]
    fn test_repository_initialization() {
        let config = GitBackendConfig::default();
        let auth_config = AuthConfig::default();
        
        let manager = GitBackendManager::new(config, auth_config).unwrap();
        
        // Create a temporary directory for the test
        let temp_dir = tempdir().unwrap();
        let repo_path = temp_dir.path();
        
        // Initialize a repository
        let repo = manager.init_repository(repo_path);
        assert!(repo.is_ok());
        
        // Check that the repository was created
        assert!(repo_path.join(".git").exists());
    }
}
