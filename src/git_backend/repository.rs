//! Repository Manager
//!
//! This module handles repository initialization, cloning, and configuration.
//! It provides functionality for managing repository structure and organization.

use std::path::{Path, PathBuf};
use git2::{Repository, RepositoryState};
use crate::git_backend::{GitBackendConfig, GitBackendError, Result, RepositoryInfo, RepositorySettings};

/// Repository Manager
pub struct RepositoryManager<'a> {
    /// The Git repository
    repo: &'a Repository,
    /// The Git Backend configuration
    config: &'a GitBackendConfig,
}

impl<'a> RepositoryManager<'a> {
    /// Creates a new RepositoryManager
    pub fn new(repo: &'a Repository, config: &'a GitBackendConfig) -> Self {
        Self { repo, config }
    }
    
    /// Configures the repository with the specified settings
    pub fn configure(&self, settings: &RepositorySettings) -> Result<()> {
        // Set the default branch name
        let mut config = self.repo.config()
            .map_err(|e| GitBackendError::RepositoryError(format!("Failed to get repository config: {}", e)))?;
        
        config.set_str("init.defaultBranch", &settings.default_branch)
            .map_err(|e| GitBackendError::RepositoryError(format!("Failed to set default branch: {}", e)))?;
        
        // Set up the repository for PLM use
        self.setup_plm_structure()?;
        
        Ok(())
    }
    
    /// Sets up the repository structure for PLM use
    pub fn setup_plm_structure(&self) -> Result<()> {
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Create the standard directory structure
        let directories = [
            "parts",
            "parts/libraries",
            "templates",
            "templates/readme",
            "templates/database",
            "scripts",
            "config",
            "config/workflows",
            "config/categories",
            "config/directory-templates",
            "config/directory-templates/custom",
            "config/settings",
        ];
        
        for dir in &directories {
            let dir_path = repo_path.join(dir);
            if !dir_path.exists() {
                std::fs::create_dir_all(&dir_path)
                    .map_err(|e| GitBackendError::IoError(e))?;
            }
        }
        
        // Create a .gitattributes file for LFS configuration
        let gitattributes_path = repo_path.join(".gitattributes");
        if !gitattributes_path.exists() {
            let gitattributes_content = r#"# Set default line ending behavior
* text=auto

# Binary files
*.pdf binary
*.png binary
*.jpg binary
*.step binary
*.stl binary

# LFS tracked files
*.pdf filter=lfs diff=lfs merge=lfs -text
*.png filter=lfs diff=lfs merge=lfs -text
*.jpg filter=lfs diff=lfs merge=lfs -text
*.step filter=lfs diff=lfs merge=lfs -text
*.stl filter=lfs diff=lfs merge=lfs -text
*.zip filter=lfs diff=lfs merge=lfs -text
*.bin filter=lfs diff=lfs merge=lfs -text
"#;
            std::fs::write(&gitattributes_path, gitattributes_content)
                .map_err(|e| GitBackendError::IoError(e))?;
        }
        
        // Create a .gitignore file
        let gitignore_path = repo_path.join(".gitignore");
        if !gitignore_path.exists() {
            let gitignore_content = r#"# Build artifacts
build/
dist/
out/

# Temporary files
*.tmp
*.temp
*.bak
*.swp
*~

# OS-specific files
.DS_Store
Thumbs.db

# IDE files
.idea/
.vscode/
*.sublime-*

# Log files
*.log
logs/

# Dependency directories
node_modules/
vendor/

# Environment files
.env
.env.local
"#;
            std::fs::write(&gitignore_path, gitignore_content)
                .map_err(|e| GitBackendError::IoError(e))?;
        }
        
        // Create directory templates
        let directory_manager = crate::git_backend::directory::DirectoryTemplateManager::new(repo_path, self.config);
        directory_manager.create_default_templates()?;
        directory_manager.create_readme_templates()?;
        
        Ok(())
    }
    
    /// Configures sparse checkout for the repository
    pub fn configure_sparse_checkout(&self, patterns: &[&str]) -> Result<()> {
        // Check if the repository is already initialized
        if self.repo.is_empty()
            .map_err(|e| GitBackendError::GitError(e))? {
            return Err(GitBackendError::RepositoryError(
                "Cannot configure sparse checkout on an empty repository".to_string()
            ));
        }
        
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Enable sparse checkout
        let mut command = std::process::Command::new(&self.config.git_executable);
        command.current_dir(repo_path)
            .args(["sparse-checkout", "init"]);
        
        let output = command.output()
            .map_err(|e| GitBackendError::IoError(e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(GitBackendError::RepositoryError(
                format!("Failed to initialize sparse checkout: {}", error)
            ));
        }
        
        // Set the sparse checkout patterns
        let mut command = std::process::Command::new(&self.config.git_executable);
        command.current_dir(repo_path)
            .arg("sparse-checkout")
            .arg("set");
        
        for pattern in patterns {
            command.arg(pattern);
        }
        
        let output = command.output()
            .map_err(|e| GitBackendError::IoError(e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(GitBackendError::RepositoryError(
                format!("Failed to set sparse checkout patterns: {}", error)
            ));
        }
        
        Ok(())
    }
    
    /// Creates a part directory with the specified template
    pub fn create_part_directory(
        &self,
        part_number: &str,
        category: &str,
        subcategory: &str,
        template_type: crate::git_backend::directory::TemplateType,
    ) -> Result<std::path::PathBuf> {
        let directory_manager = crate::git_backend::directory::DirectoryTemplateManager::new(
            self.repo.path().parent().unwrap_or(std::path::Path::new("")),
            self.config
        );
        
        directory_manager.create_part_directory(part_number, category, subcategory, template_type)
    }
    
    /// Gets information about the repository
    pub fn get_info(&self) -> Result<RepositoryInfo> {
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?
            .to_path_buf();
        
        // Get the current branch
        let head = self.repo.head()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        let current_branch = if head.is_branch() {
            head.shorthand().unwrap_or("HEAD").to_string()
        } else {
            "HEAD".to_string()
        };
        
        // Check if the repository has uncommitted changes
        let statuses = self.repo.statuses(None)
            .map_err(|e| GitBackendError::GitError(e))?;
        
        let has_changes = statuses.iter().any(|entry| {
            entry.status().is_wt_modified() ||
            entry.status().is_wt_deleted() ||
            entry.status().is_wt_typechange() ||
            entry.status().is_wt_renamed() ||
            entry.status().is_wt_new()
        });
        
        // Check if LFS is enabled
        let lfs_enabled = self.is_lfs_enabled()?;
        
        Ok(RepositoryInfo {
            path: repo_path,
            current_branch,
            has_changes,
            lfs_enabled,
        })
    }
    
    /// Checks if LFS is enabled for the repository
    fn is_lfs_enabled(&self) -> Result<bool> {
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Check if the .gitattributes file contains LFS configuration
        let gitattributes_path = repo_path.join(".gitattributes");
        if !gitattributes_path.exists() {
            return Ok(false);
        }
        
        let content = std::fs::read_to_string(&gitattributes_path)
            .map_err(|e| GitBackendError::IoError(e))?;
        
        Ok(content.contains("filter=lfs diff=lfs merge=lfs"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::git_backend::GitBackendConfig;
    use crate::git_backend::directory::TemplateType;

    #[test]
    fn test_setup_plm_structure() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();
        
        // Initialize a Git repository
        let repo = git2::Repository::init(repo_path).unwrap();
        
        // Create a repository manager
        let config = GitBackendConfig::default();
        let repo_manager = RepositoryManager::new(&repo, &config);
        
        // Set up the PLM structure
        repo_manager.setup_plm_structure().unwrap();
        
        // Check that the directories were created
        assert!(repo_path.join("parts").exists());
        assert!(repo_path.join("templates").exists());
        assert!(repo_path.join("scripts").exists());
        assert!(repo_path.join("config").exists());
        assert!(repo_path.join("config/directory-templates").exists());
        
        // Check that the .gitattributes file was created
        assert!(repo_path.join(".gitattributes").exists());
        
        // Check that the .gitignore file was created
        assert!(repo_path.join(".gitignore").exists());
        
        // Check that the directory templates were created
        assert!(repo_path.join("config/directory-templates/minimal.json").exists());
        assert!(repo_path.join("config/directory-templates/standard.json").exists());
        assert!(repo_path.join("config/directory-templates/extended.json").exists());
        
        // Check that the README templates were created
        assert!(repo_path.join("templates/readme/minimal.md").exists());
        assert!(repo_path.join("templates/readme/standard.md").exists());
        assert!(repo_path.join("templates/readme/extended.md").exists());
    }
    
    #[test]
    fn test_create_part_directory() {
        // Create a temporary directory for the test
        let temp_dir = TempDir::new().unwrap();
        let repo_path = temp_dir.path();
        
        // Initialize a Git repository
        let repo = git2::Repository::init(repo_path).unwrap();
        
        // Create a repository manager
        let config = GitBackendConfig::default();
        let repo_manager = RepositoryManager::new(&repo, &config);
        
        // Set up the PLM structure
        repo_manager.setup_plm_structure().unwrap();
        
        // Create a part directory with the minimal template
        let part_dir = repo_manager.create_part_directory("10001", "EL", "PCB", TemplateType::Minimal).unwrap();
        
        // Check that the part directory was created
        assert!(part_dir.exists());
        assert_eq!(part_dir, repo_path.join("parts/EL-PCB-10001"));
        
        // Check that the required directories were created
        assert!(part_dir.join("design").exists());
        
        // Check that the required files were created
        assert!(part_dir.join("README.md").exists());
        assert!(part_dir.join("metadata.db").exists());
    }
}