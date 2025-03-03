//! Implexa: A hardware-focused PLM/PDM solution
//!
//! Implexa is a Product Lifecycle Management (PLM) and Product Data Management (PDM)
//! solution that leverages Git for version control while remaining CAD-agnostic.
//! It bridges the gap between software engineering practices and hardware design workflows,
//! enabling efficient management of design files across multiple CAD platforms.

pub mod git_backend;
pub mod database;

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

/// Re-export the Database types and managers for easier access
pub use database::{
    // Core database types
    DatabaseManager,
    DatabaseError,
    DatabaseResult,
    
    // Part types and manager
    Part,
    PartManager,
    
    // Revision types and manager
    Revision,
    RevisionStatus,
    RevisionManager,
    
    // Relationship types and manager
    Relationship,
    RelationshipType,
    RelationshipManager,
    
    // Property types and manager
    Property,
    PropertyType,
    PropertyManager,
    
    // Manufacturer part types and manager
    ManufacturerPart,
    ManufacturerPartStatus,
    ManufacturerPartManager,
    
    // Approval types and manager
    Approval,
    ApprovalStatus,
    ApprovalManager,
    
    // File types and manager
    File,
    FileType,
    FileManager,
    
    // Workflow types and managers
    Workflow,
    WorkflowState,
    WorkflowTransition,
    WorkflowManager,
    
    // Part Management types and manager
    PartManagementManager,
    PartManagementError,
    PartManagementResult,
    User,
    UserRole,
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

    #[test]
    fn test_database_initialization() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager
        let mut db_manager = DatabaseManager::new(&db_path).unwrap();

        // Initialize the schema
        let result = db_manager.initialize_schema();
        assert!(result.is_ok());

        // Check that the schema version is 1
        let version = db_manager.get_schema_version().unwrap();
        assert_eq!(version, 1);
    }

    #[test]
    fn test_part_creation() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let mut db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create a part manager
        let part_manager = PartManager::new(db_manager.connection());

        // Create a new part
        let part = Part::new(
            "ELE-RES-001".to_string(),
            "Electronic".to_string(),
            "Resistor".to_string(),
            "10K Resistor".to_string(),
            Some("1/4W 10K Ohm Resistor".to_string()),
        );

        // Save the part to the database
        let result = part_manager.create_part(&part);
        assert!(result.is_ok());

        // Retrieve the part from the database
        let retrieved_part = part_manager.get_part("ELE-RES-001").unwrap();

        // Check that the retrieved part matches the original
        assert_eq!(retrieved_part.part_id, part.part_id);
        assert_eq!(retrieved_part.category, part.category);
        assert_eq!(retrieved_part.subcategory, part.subcategory);
        assert_eq!(retrieved_part.name, part.name);
        assert_eq!(retrieved_part.description, part.description);
    }
}
