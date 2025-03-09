//! LFS Manager
//!
//! This module handles Git-LFS (Large File Storage) operations for binary files.
//! It configures and manages Git-LFS, handles LFS pointer files and object storage,
//! and optimizes storage and retrieval of large files.

use std::process::Command;
use git2::Repository;
use crate::git_backend::{GitBackendConfig, GitBackendError, Result, LfsStatus};

/// LFS Manager
pub struct LfsManager<'a> {
    /// The Git repository
    repo: &'a Repository,
    /// The Git Backend configuration
    config: &'a GitBackendConfig,
}

impl<'a> LfsManager<'a> {
    /// Creates a new LfsManager
    pub fn new(repo: &'a Repository, config: &'a GitBackendConfig) -> Self {
        Self { repo, config }
    }
    
    /// Initializes LFS for the repository
    pub fn init_lfs(&self) -> Result<()> {
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Run git lfs install
        let output = Command::new(&self.config.git_lfs_executable)
            .current_dir(repo_path)
            .arg("install")
            .output()
            .map_err(|e| GitBackendError::IoError(e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(GitBackendError::LfsError(format!("Failed to initialize LFS: {}", error)));
        }
        
        // Update .gitattributes with default LFS patterns
        self.track_patterns(&self.config.lfs_patterns.iter().map(|s| s.as_str()).collect::<Vec<_>>())?;
        
        Ok(())
    }
    
    /// Configures LFS to track the specified file patterns
    pub fn track_patterns(&self, patterns: &[&str]) -> Result<()> {
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Track each pattern
        for pattern in patterns {
            let output = Command::new(&self.config.git_lfs_executable)
                .current_dir(repo_path)
                .arg("track")
                .arg(pattern)
                .output()
                .map_err(|e| GitBackendError::IoError(e))?;
            
            if !output.status.success() {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(GitBackendError::LfsError(format!("Failed to track pattern {}: {}", pattern, error)));
            }
        }
        
        Ok(())
    }
    
    /// Gets the status of LFS objects in the repository
    pub fn get_lfs_status(&self) -> Result<LfsStatus> {
        // Get the repository path
        let _repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Check if LFS is enabled
        let enabled = self.is_lfs_enabled()?;
        
        if !enabled {
            return Ok(LfsStatus {
                enabled: false,
                tracked_patterns: Vec::new(),
                object_count: 0,
                total_size: 0,
            });
        }
        
        // Get tracked patterns
        let tracked_patterns = self.get_tracked_patterns()?;
        
        // Get LFS object count and size
        let (object_count, total_size) = self.get_lfs_object_stats()?;
        
        Ok(LfsStatus {
            enabled,
            tracked_patterns,
            object_count,
            total_size,
        })
    }
    
    /// Pulls LFS objects for the current checkout
    pub fn pull_objects(&self) -> Result<()> {
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Run git lfs pull
        let output = Command::new(&self.config.git_lfs_executable)
            .current_dir(repo_path)
            .arg("pull")
            .output()
            .map_err(|e| GitBackendError::IoError(e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(GitBackendError::LfsError(format!("Failed to pull LFS objects: {}", error)));
        }
        
        Ok(())
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
    
    /// Gets the list of patterns tracked by LFS
    fn get_tracked_patterns(&self) -> Result<Vec<String>> {
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Run git lfs track
        let output = Command::new(&self.config.git_lfs_executable)
            .current_dir(repo_path)
            .arg("track")
            .output()
            .map_err(|e| GitBackendError::IoError(e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(GitBackendError::LfsError(format!("Failed to get tracked patterns: {}", error)));
        }
        
        // Parse the output
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut patterns = Vec::new();
        
        for line in output_str.lines() {
            if line.starts_with("Tracking ") && line.ends_with("through .gitattributes") {
                let pattern = line.strip_prefix("Tracking ")
                    .unwrap()
                    .strip_suffix(" through .gitattributes")
                    .unwrap()
                    .trim_matches('"');
                patterns.push(pattern.to_string());
            }
        }
        
        Ok(patterns)
    }
    
    /// Gets the count and total size of LFS objects
    fn get_lfs_object_stats(&self) -> Result<(usize, u64)> {
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Run git lfs ls-files
        let output = Command::new(&self.config.git_lfs_executable)
            .current_dir(repo_path)
            .args(["ls-files", "--size"])
            .output()
            .map_err(|e| GitBackendError::IoError(e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(GitBackendError::LfsError(format!("Failed to get LFS object stats: {}", error)));
        }
        
        // Parse the output
        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut object_count = 0;
        let mut total_size = 0;
        
        for line in output_str.lines() {
            object_count += 1;
            
            // Extract the size
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let size_str = parts[1];
                if let Some(size_value) = size_str.strip_suffix('k') {
                    if let Ok(size) = size_value.parse::<u64>() {
                        total_size += size * 1024;
                    }
                } else if let Some(size_value) = size_str.strip_suffix('m') {
                    if let Ok(size) = size_value.parse::<u64>() {
                        total_size += size * 1024 * 1024;
                    }
                } else if let Some(size_value) = size_str.strip_suffix('g') {
                    if let Ok(size) = size_value.parse::<u64>() {
                        total_size += size * 1024 * 1024 * 1024;
                    }
                } else if let Ok(size) = size_str.parse::<u64>() {
                    total_size += size;
                }
            }
        }
        
        Ok((object_count, total_size))
    }
}