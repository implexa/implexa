//! Git Backend Manager
//!
//! This module provides a high-level abstraction over Git operations while preserving PLM metadata
//! and ensuring data integrity. It handles repository management, version control operations,
//! large file management, metadata preservation, hooks management, conflict resolution, and authentication.

pub mod repository;
pub mod operation;
pub mod lfs;
pub mod hook;
pub mod conflict;
pub mod auth;
pub mod directory;

use std::path::{Path, PathBuf};
use git2::{Repository, Oid, Branch, Commit};
use thiserror::Error;
use std::io;

/// Configuration for the Git Backend Manager
#[derive(Debug, Clone)]
pub struct GitBackendConfig {
    /// Default branch name for new repositories
    pub default_branch: String,
    /// Whether to enable Git-LFS by default
    pub lfs_enabled: bool,
    /// Default patterns to track with Git-LFS
    pub lfs_patterns: Vec<String>,
    /// Path to the Git executable
    pub git_executable: PathBuf,
    /// Path to the Git-LFS executable
    pub git_lfs_executable: PathBuf,
}

impl Default for GitBackendConfig {
    fn default() -> Self {
        Self {
            default_branch: "main".to_string(),
            lfs_enabled: true,
            lfs_patterns: vec![
                "*.bin".to_string(),
                "*.obj".to_string(),
                "*.step".to_string(),
                "*.stl".to_string(),
                "*.pdf".to_string(),
                "*.png".to_string(),
                "*.jpg".to_string(),
                "*.zip".to_string(),
            ],
            git_executable: "git".into(),
            git_lfs_executable: "git-lfs".into(),
        }
    }
}

/// Error type for Git Backend operations
#[derive(Error, Debug)]
pub enum GitBackendError {
    /// Error from the underlying Git library
    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),
    
    /// Error related to repository initialization or configuration
    #[error("Repository error: {0}")]
    RepositoryError(String),
    
    /// Error related to Git operations
    #[error("Operation error: {0}")]
    OperationError(String),
    
    /// Error related to LFS operations
    #[error("LFS error: {0}")]
    LfsError(String),
    
    /// Error related to hooks
    #[error("Hook error: {0}")]
    HookError(String),
    
    /// Error related to conflict resolution
    #[error("Conflict error: {0}")]
    ConflictError(String),
    
    /// Error related to authentication
    #[error("Auth error: {0}")]
    AuthError(String),
    
    /// Error related to file system operations
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    
    /// Other errors
    #[error("Other error: {0}")]
    Other(String),
}

/// Result type for Git Backend operations
pub type Result<T> = std::result::Result<T, GitBackendError>;

/// Information about a repository
#[derive(Debug, Clone)]
pub struct RepositoryInfo {
    /// Path to the repository
    pub path: PathBuf,
    /// Current branch name
    pub current_branch: String,
    /// Whether the repository has uncommitted changes
    pub has_changes: bool,
    /// Whether LFS is enabled for this repository
    pub lfs_enabled: bool,
}

/// Result of a merge operation
#[derive(Debug, Clone)]
pub struct MergeResult {
    /// Whether the merge was successful
    pub success: bool,
    /// Whether there were conflicts during the merge
    pub has_conflicts: bool,
    /// List of conflicted files
    pub conflicted_files: Vec<PathBuf>,
    /// Commit ID of the merge commit (if successful)
    pub commit_id: Option<String>,
}

/// Status of LFS objects in a repository
#[derive(Debug, Clone)]
pub struct LfsStatus {
    /// Whether LFS is enabled for this repository
    pub enabled: bool,
    /// List of patterns tracked by LFS
    pub tracked_patterns: Vec<String>,
    /// Number of LFS objects in the repository
    pub object_count: usize,
    /// Total size of LFS objects in bytes
    pub total_size: u64,
}

/// Type of Git hook
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HookType {
    /// Called before a commit is created
    PreCommit,
    /// Called after a commit is created
    PostCommit,
    /// Called before a merge is performed
    PreMerge,
    /// Called after a merge is performed
    PostMerge,
    /// Called before a push is performed
    PrePush,
    /// Called after a push is performed
    PostPush,
    /// Called when a checkout is performed
    PostCheckout,
    /// Called when a repository is cloned
    PostClone,
    /// Called when a repository is initialized
    PostInit,
    /// Called when a repository is updated
    PostUpdate,
    /// Called when a reference is updated
    PostReferenceUpdate,
}

/// Strategy for resolving conflicts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictStrategy {
    /// Use the version from the current branch
    Ours,
    /// Use the version from the branch being merged
    Theirs,
    /// Use both versions
    Union,
}

/// Credentials for Git authentication
#[derive(Debug, Clone)]
pub enum Credentials {
    /// Username and password
    UserPass {
        username: String,
        password: String,
    },
    /// SSH key
    SshKey {
        username: String,
        public_key: PathBuf,
        private_key: PathBuf,
        passphrase: Option<String>,
    },
    /// Personal access token
    Token {
        username: String,
        token: String,
    },
}

/// Configuration for authentication
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// Whether to use the system credential helper
    pub use_credential_helper: bool,
    /// Path to the SSH key
    pub ssh_key_path: Option<PathBuf>,
    /// Whether to use the SSH agent
    pub use_ssh_agent: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            use_credential_helper: true,
            ssh_key_path: None,
            use_ssh_agent: true,
        }
    }
}

/// Metadata for a commit
#[derive(Debug, Clone)]
pub struct Metadata {
    /// Key-value pairs of metadata
    pub properties: std::collections::HashMap<String, String>,
}

/// Status of a repository for display in the UI
#[derive(Debug, Clone)]
pub struct UiStatus {
    /// Current branch name
    pub current_branch: String,
    /// Whether the repository has uncommitted changes
    pub has_changes: bool,
    /// List of modified files
    pub modified_files: Vec<PathBuf>,
    /// List of untracked files
    pub untracked_files: Vec<PathBuf>,
    /// List of branches
    pub branches: Vec<String>,
    /// List of tags
    pub tags: Vec<String>,
}

/// Result of a UI-friendly commit operation
#[derive(Debug, Clone)]
pub struct UiCommitResult {
    /// Whether the commit was successful
    pub success: bool,
    /// Commit ID
    pub commit_id: Option<String>,
    /// Error message (if any)
    pub error_message: Option<String>,
}

/// Main Git Backend Manager
pub struct GitBackendManager {
    config: GitBackendConfig,
    auth_provider: auth::AuthProvider,
}

impl GitBackendManager {
    /// Creates a new GitBackendManager with the specified configuration
    pub fn new(config: GitBackendConfig, auth_config: AuthConfig) -> Result<Self> {
        let auth_provider = auth::AuthProvider::new(auth_config)?;
        
        Ok(Self {
            config,
            auth_provider,
        })
    }
    
    /// Initializes a new repository at the specified path
    pub fn init_repository(&self, path: &Path) -> Result<Repository> {
        // Create the directory if it doesn't exist
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        
        // Initialize the repository
        let repo = Repository::init(path)
            .map_err(|e| GitBackendError::RepositoryError(format!("Failed to initialize repository: {}", e)))?;
        
        // Configure the repository
        let repo_manager = self.repository_manager(&repo);
        repo_manager.configure(&RepositorySettings {
            default_branch: self.config.default_branch.clone(),
        })?;
        
        // Set up LFS if enabled
        if self.config.lfs_enabled {
            let lfs_manager = self.lfs_manager(&repo);
            lfs_manager.init_lfs()?;
            lfs_manager.track_patterns(&self.config.lfs_patterns.iter().map(|s| s.as_str()).collect::<Vec<_>>())?;
        }
        
        // Set up default hooks
        let hook_manager = self.hook_manager(&repo);
        hook_manager.install_default_plm_hooks()?;
        
        Ok(repo)
    }
    
    /// Clones a repository from the specified URL
    pub fn clone_repository(&self, url: &str, path: &Path) -> Result<Repository> {
        // Create the directory if it doesn't exist
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
        
        // Set up the fetch options with authentication
        let mut fetch_options = git2::FetchOptions::new();
        let mut callbacks = git2::RemoteCallbacks::new();
        
        callbacks.credentials(|url, username_from_url, allowed_types| {
            self.auth_provider.get_git2_credentials(url, username_from_url, allowed_types)
                .map_err(|e| git2::Error::from_str(&e.to_string()))
        });
        
        fetch_options.remote_callbacks(callbacks);
        
        // Clone the repository
        let repo = git2::build::RepoBuilder::new()
            .fetch_options(fetch_options)
            .clone(url, path)
            .map_err(|e| GitBackendError::RepositoryError(format!("Failed to clone repository: {}", e)))?;
        
        // Set up LFS if enabled
        if self.config.lfs_enabled {
            let lfs_manager = self.lfs_manager(&repo);
            lfs_manager.init_lfs()?;
            lfs_manager.pull_objects()?;
        }
        
        Ok(repo)
    }
    
    /// Opens an existing repository at the specified path
    pub fn open_repository(&self, path: &Path) -> Result<Repository> {
        Repository::open(path)
            .map_err(|e| GitBackendError::RepositoryError(format!("Failed to open repository: {}", e)))
    }
    
    /// Gets the repository manager for the specified repository
    pub fn repository_manager<'a>(&'a self, repo: &'a Repository) -> repository::RepositoryManager<'a> {
        repository::RepositoryManager::new(repo, &self.config)
    }
    
    /// Gets the operation handler for the specified repository
    pub fn operation_handler<'a>(&'a self, repo: &'a Repository) -> operation::OperationHandler<'a> {
        operation::OperationHandler::new(repo, &self.config)
    }
    
    /// Gets the LFS manager for the specified repository
    pub fn lfs_manager<'a>(&'a self, repo: &'a Repository) -> lfs::LfsManager<'a> {
        lfs::LfsManager::new(repo, &self.config)
    }
    
    /// Gets the hook manager for the specified repository
    pub fn hook_manager<'a>(&'a self, repo: &'a Repository) -> hook::HookManager<'a> {
        hook::HookManager::new(repo, &self.config)
    }
    
    /// Gets the conflict resolver for the specified repository
    pub fn conflict_resolver<'a>(&'a self, repo: &'a Repository) -> conflict::ConflictResolver<'a> {
        conflict::ConflictResolver::new(repo, &self.config)
    }
    
    /// Gets the auth provider
    pub fn auth_provider(&self) -> &auth::AuthProvider {
        &self.auth_provider
    }
    
    /// Gets the directory template manager for the specified repository
    pub fn directory_template_manager<'a>(&'a self, repo: &'a Repository) -> directory::DirectoryTemplateManager<'a> {
        let repo_path = repo.path().parent().unwrap_or(Path::new(""));
        directory::DirectoryTemplateManager::new(repo_path, &self.config)
    }
    
    /// Creates a new branch with the specified name
    pub fn create_branch<'a>(&self, repo: &'a Repository, name: &str) -> Result<Branch<'a>> {
        // Directly call the operation method on the repository to avoid lifetime issues
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;
        let branch = repo.branch(name, &commit, false)?;
        Ok(branch)
    }
    
    /// Switches to the specified branch
    pub fn checkout_branch(&self, repo: &Repository, name: &str) -> Result<()> {
        self.operation_handler(repo).checkout_branch(name)
    }
    
    /// Merges the specified branch into the current branch
    pub fn merge_branch(&self, repo: &Repository, name: &str) -> Result<MergeResult> {
        self.operation_handler(repo).merge_branch(name)
    }
    
    /// Commits changes with metadata
    pub fn commit_with_metadata(
        &self,
        repo: &Repository,
        message: &str,
        files: &[&Path],
        metadata: &Metadata,
    ) -> Result<Oid> {
        // Convert metadata to a JSON string
        let metadata_json = serde_json::to_string(&metadata.properties)
            .map_err(|e| GitBackendError::Other(format!("Failed to serialize metadata: {}", e)))?;
        
        // Create a commit message with metadata
        let full_message = format!("{}\n\nPLM-Metadata: {}", message, metadata_json);
        
        // Perform the commit
        self.operation_handler(repo).commit(&full_message, files)
    }
    
    /// Gets metadata for the specified commit
    pub fn get_commit_metadata(
        &self,
        repo: &Repository,
        commit: &Commit,
    ) -> Result<Metadata> {
        // Get the commit message
        let message = commit.message()
            .ok_or_else(|| GitBackendError::Other("Commit message is not valid UTF-8".to_string()))?;
        
        // Extract metadata from the commit message
        let mut properties = std::collections::HashMap::new();
        
        if let Some(metadata_start) = message.find("PLM-Metadata: ") {
            let metadata_json = &message[metadata_start + 14..];
            if let Ok(parsed) = serde_json::from_str::<std::collections::HashMap<String, String>>(metadata_json) {
                properties = parsed;
            }
        }
        
        Ok(Metadata { properties })
    }
    
    /// Gets the status of the repository for display in the UI
    pub fn get_status_for_ui(&self, repo: &Repository) -> Result<UiStatus> {
        let repo_manager = self.repository_manager(repo);
        let info = repo_manager.get_info()?;
        
        let statuses = repo.statuses(None)
            .map_err(|e| GitBackendError::GitError(e))?;
        
        let mut modified_files = Vec::new();
        let mut untracked_files = Vec::new();
        
        for entry in statuses.iter() {
            let path = Path::new(entry.path().unwrap_or("")).to_path_buf();
            
            if entry.status().is_wt_modified() || entry.status().is_wt_deleted() || entry.status().is_wt_typechange() {
                modified_files.push(path);
            } else if entry.status().is_wt_new() {
                untracked_files.push(path);
            }
        }
        
        let mut branches = Vec::new();
        repo.branches(None)
            .map_err(|e| GitBackendError::GitError(e))?
            .for_each(|branch_result| {
                if let Ok((branch, _)) = branch_result {
                    if let Ok(name) = branch.name() {
                        if let Some(name) = name {
                            branches.push(name.to_string());
                        }
                    }
                }
            });
        
        let mut tags = Vec::new();
        repo.tag_names(None)
            .map_err(|e| GitBackendError::GitError(e))?
            .iter()
            .for_each(|tag| {
                if let Some(tag) = tag {
                    tags.push(tag.to_string());
                }
            });
        
        Ok(UiStatus {
            current_branch: info.current_branch,
            has_changes: info.has_changes,
            modified_files,
            untracked_files,
            branches,
            tags,
        })
    }
    
    /// Performs a UI-friendly commit operation
    pub fn ui_commit(
        &self,
        repo: &Repository,
        message: &str,
        files: &[&Path],
    ) -> Result<UiCommitResult> {
        match self.operation_handler(repo).commit(message, files) {
            Ok(oid) => Ok(UiCommitResult {
                success: true,
                commit_id: Some(oid.to_string()),
                error_message: None,
            }),
            Err(e) => Ok(UiCommitResult {
                success: false,
                commit_id: None,
                error_message: Some(e.to_string()),
            }),
        }
    }
    
    /// Logs an operation for debugging and audit purposes
    fn log_operation(&self, operation: &str, details: &str) {
        log::info!("Git operation: {} - {}", operation, details);
    }
}

/// Settings for repository configuration
#[derive(Debug, Clone)]
pub struct RepositorySettings {
    /// Default branch name
    pub default_branch: String,
}