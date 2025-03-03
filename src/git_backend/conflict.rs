//! Conflict Resolver
//!
//! This module handles merge conflicts in Git repositories. It provides functionality
//! for detecting and handling merge conflicts, providing strategies for automatic
//! conflict resolution where possible, and facilitating manual conflict resolution with clear context.

use std::path::{Path, PathBuf};
use std::fs;
use git2::{Repository, IndexEntry};
use crate::git_backend::{GitBackendConfig, GitBackendError, Result, ConflictStrategy};

/// Conflict Resolver
pub struct ConflictResolver<'a> {
    /// The Git repository
    repo: &'a Repository,
    /// The Git Backend configuration
    config: &'a GitBackendConfig,
}

impl<'a> ConflictResolver<'a> {
    /// Creates a new ConflictResolver
    pub fn new(repo: &'a Repository, config: &'a GitBackendConfig) -> Self {
        Self { repo, config }
    }
    
    /// Checks if there are any conflicts in the repository
    pub fn has_conflicts(&self) -> Result<bool> {
        let index = self.repo.index()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        Ok(index.has_conflicts())
    }
    
    /// Gets the list of conflicted files
    pub fn get_conflicted_files(&self) -> Result<Vec<PathBuf>> {
        let index = self.repo.index()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        if !index.has_conflicts() {
            return Ok(Vec::new());
        }
        
        let mut conflicted_files = Vec::new();
        
        // Iterate through the index entries
        for entry in index.iter() {
            // Check if the entry has a path
            let path_bytes = &entry.path;
            if !path_bytes.is_empty() {
                let path_str = String::from_utf8_lossy(path_bytes).to_string();
                let path = Path::new(&path_str).to_path_buf();
                
                // Check if this path has conflicts using the conflicts iterator
                let mut has_conflict = false;
                
                if let Ok(conflicts) = index.conflicts() {
                    for conflict in conflicts {
                        if let Ok(conflict) = conflict {
                            if let Some(our_entry) = conflict.our {
                                let our_path = String::from_utf8_lossy(&our_entry.path).to_string();
                                if our_path == path_str {
                                    has_conflict = true;
                                    break;
                                }
                            }
                        }
                    }
                }
                
                if has_conflict {
                    conflicted_files.push(path);
                }
            }
        }
        
        Ok(conflicted_files)
    }
    
    /// Resolves the conflict for the specified file using the specified strategy
    pub fn resolve_conflict(&self, file: &Path, strategy: ConflictStrategy) -> Result<()> {
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Get the full path to the file
        let file_path = if file.is_absolute() {
            file.to_path_buf()
        } else {
            repo_path.join(file)
        };
        
        // Check if the file exists
        if !file_path.exists() {
            return Err(GitBackendError::ConflictError(format!("File {} does not exist", file_path.display())));
        }
        
        // Get the index
        let mut index = self.repo.index()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Get the conflict entries
        let file_str = file.to_str().unwrap();
        let mut conflict_entries = (None, None, None);
        let mut found_conflict = false;
        
        if let Ok(conflicts) = index.conflicts() {
            for conflict in conflicts {
                if let Ok(conflict_data) = conflict {
                    // Check if any of the conflict entries match our file
                    let matches = if let Some(our_entry) = &conflict_data.our {
                        String::from_utf8_lossy(&our_entry.path).to_string() == file_str
                    } else if let Some(their_entry) = &conflict_data.their {
                        String::from_utf8_lossy(&their_entry.path).to_string() == file_str
                    } else if let Some(ancestor_entry) = &conflict_data.ancestor {
                        String::from_utf8_lossy(&ancestor_entry.path).to_string() == file_str
                    } else {
                        false
                    };
                    
                    if matches {
                        conflict_entries = (conflict_data.our, conflict_data.ancestor, conflict_data.their);
                        found_conflict = true;
                        break;
                    }
                }
            }
        }
        
        if !found_conflict {
            return Err(GitBackendError::ConflictError(format!("No conflict found for file {}", file_path.display())));
        }
        
        // Resolve the conflict based on the strategy
        match strategy {
            ConflictStrategy::Ours => {
                // Use our version
                if let Some(our_entry) = conflict_entries.0 {
                    self.resolve_with_entry(file, &our_entry)?;
                } else {
                    // Our version doesn't exist, so remove the file
                    if file_path.exists() {
                        fs::remove_file(&file_path)
                            .map_err(|e| GitBackendError::IoError(e))?;
                    }
                }
            },
            ConflictStrategy::Theirs => {
                // Use their version
                if let Some(their_entry) = conflict_entries.2 {
                    self.resolve_with_entry(file, &their_entry)?;
                } else {
                    // Their version doesn't exist, so remove the file
                    if file_path.exists() {
                        fs::remove_file(&file_path)
                            .map_err(|e| GitBackendError::IoError(e))?;
                    }
                }
            },
            ConflictStrategy::Union => {
                // Merge both versions
                self.resolve_with_union(file)?;
            },
        }
        
        // Mark the conflict as resolved
        index.add_path(file)
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Write the index
        index.write()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        Ok(())
    }
    
    /// Aborts the current merge operation
    pub fn abort_merge(&self) -> Result<()> {
        // Check if we're in a merge state
        if self.repo.state() != git2::RepositoryState::Merge {
            return Err(GitBackendError::ConflictError("Not in a merge state".to_string()));
        }
        
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Run git merge --abort
        let output = std::process::Command::new(&self.config.git_executable)
            .current_dir(repo_path)
            .args(["merge", "--abort"])
            .output()
            .map_err(|e| GitBackendError::IoError(e))?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(GitBackendError::ConflictError(format!("Failed to abort merge: {}", error)));
        }
        
        Ok(())
    }
    
    /// Resolves a conflict with the specified entry
    fn resolve_with_entry(&self, file: &Path, entry: &IndexEntry) -> Result<()> {
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Get the full path to the file
        let file_path = if file.is_absolute() {
            file.to_path_buf()
        } else {
            repo_path.join(file)
        };
        
        // Get the object from the repository
        let object = self.repo.find_object(entry.id, None)
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Get the blob
        let blob = object.as_blob()
            .ok_or_else(|| GitBackendError::ConflictError("Entry is not a blob".to_string()))?;
        
        // Write the blob content to the file
        fs::write(&file_path, blob.content())
            .map_err(|e| GitBackendError::IoError(e))?;
        
        Ok(())
    }
    
    /// Resolves a conflict by merging both versions
    fn resolve_with_union(&self, file: &Path) -> Result<()> {
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Get the full path to the file
        let file_path = if file.is_absolute() {
            file.to_path_buf()
        } else {
            repo_path.join(file)
        };
        
        // Read the current file content (with conflict markers)
        let content = fs::read_to_string(&file_path)
            .map_err(|e| GitBackendError::IoError(e))?;
        
        // Parse the conflict markers
        let mut result = String::new();
        let mut in_conflict = false;
        let mut ours = String::new();
        let mut theirs = String::new();
        
        for line in content.lines() {
            if line.starts_with("<<<<<<< ") {
                in_conflict = true;
                continue;
            } else if line.starts_with("=======") {
                in_conflict = false;
                continue;
            } else if line.starts_with(">>>>>>> ") {
                // Add both versions
                result.push_str(&ours);
                result.push_str(&theirs);
                
                // Reset for the next conflict
                ours.clear();
                theirs.clear();
                continue;
            }
            
            if in_conflict {
                ours.push_str(line);
                ours.push('\n');
            } else if !ours.is_empty() {
                // We're in the "theirs" section
                theirs.push_str(line);
                theirs.push('\n');
            } else {
                // We're outside a conflict
                result.push_str(line);
                result.push('\n');
            }
        }
        
        // Write the merged content back to the file
        fs::write(&file_path, result)
            .map_err(|e| GitBackendError::IoError(e))?;
        
        Ok(())
    }
    
    /// Resolves conflicts in BOM files
    pub fn resolve_bom_conflict(
        &self,
        file: &Path,
        strategy: BomConflictStrategy,
    ) -> Result<()> {
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Get the full path to the file
        let file_path = if file.is_absolute() {
            file.to_path_buf()
        } else {
            repo_path.join(file)
        };
        
        // Check if the file exists
        if !file_path.exists() {
            return Err(GitBackendError::ConflictError(format!("File {} does not exist", file_path.display())));
        }
        
        // Read the current file content (with conflict markers)
        let content = fs::read_to_string(&file_path)
            .map_err(|e| GitBackendError::IoError(e))?;
        
        // Parse the BOM file and resolve conflicts
        let resolved_content = match strategy {
            BomConflictStrategy::PreferOurs => {
                self.resolve_bom_prefer_ours(&content)?
            },
            BomConflictStrategy::PreferTheirs => {
                self.resolve_bom_prefer_theirs(&content)?
            },
            BomConflictStrategy::MergeQuantities => {
                self.resolve_bom_merge_quantities(&content)?
            },
        };
        
        // Write the resolved content back to the file
        fs::write(&file_path, resolved_content)
            .map_err(|e| GitBackendError::IoError(e))?;
        
        // Mark the conflict as resolved
        let mut index = self.repo.index()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        index.add_path(file)
            .map_err(|e| GitBackendError::GitError(e))?;
        
        index.write()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        Ok(())
    }
    
    /// Resolves BOM conflicts by preferring our version
    fn resolve_bom_prefer_ours(&self, content: &str) -> Result<String> {
        // This is a simplified implementation
        // In a real implementation, we would parse the BOM file format
        // and resolve conflicts based on the BOM structure
        
        let mut result = String::new();
        let mut in_conflict = false;
        let mut ours = String::new();
        
        for line in content.lines() {
            if line.starts_with("<<<<<<< ") {
                in_conflict = true;
                continue;
            } else if line.starts_with("=======") {
                in_conflict = false;
                continue;
            } else if line.starts_with(">>>>>>> ") {
                // Add our version
                result.push_str(&ours);
                
                // Reset for the next conflict
                ours.clear();
                continue;
            }
            
            if in_conflict {
                ours.push_str(line);
                ours.push('\n');
            } else if !ours.is_empty() {
                // We're in the "theirs" section, ignore
            } else {
                // We're outside a conflict
                result.push_str(line);
                result.push('\n');
            }
        }
        
        Ok(result)
    }
    
    /// Resolves BOM conflicts by preferring their version
    fn resolve_bom_prefer_theirs(&self, content: &str) -> Result<String> {
        // This is a simplified implementation
        // In a real implementation, we would parse the BOM file format
        // and resolve conflicts based on the BOM structure
        
        let mut result = String::new();
        let mut in_conflict = false;
        let mut theirs = String::new();
        
        for line in content.lines() {
            if line.starts_with("<<<<<<< ") {
                in_conflict = true;
                continue;
            } else if line.starts_with("=======") {
                in_conflict = false;
                theirs.clear();
                continue;
            } else if line.starts_with(">>>>>>> ") {
                // Add their version
                result.push_str(&theirs);
                
                // Reset for the next conflict
                theirs.clear();
                continue;
            }
            
            if in_conflict {
                // We're in the "ours" section, ignore
            } else if !theirs.is_empty() || in_conflict == false {
                // We're in the "theirs" section or outside a conflict
                theirs.push_str(line);
                theirs.push('\n');
            }
        }
        
        Ok(result)
    }
    
    /// Resolves BOM conflicts by merging quantities
    fn resolve_bom_merge_quantities(&self, content: &str) -> Result<String> {
        // This is a simplified implementation
        // In a real implementation, we would parse the BOM file format
        // and merge quantities based on the BOM structure
        
        // For now, we'll just use the union strategy
        let mut result = String::new();
        let mut in_conflict = false;
        let mut ours = String::new();
        let mut theirs = String::new();
        
        for line in content.lines() {
            if line.starts_with("<<<<<<< ") {
                in_conflict = true;
                continue;
            } else if line.starts_with("=======") {
                in_conflict = false;
                continue;
            } else if line.starts_with(">>>>>>> ") {
                // Add both versions with a comment
                result.push_str("# BEGIN MERGED BOM ENTRIES\n");
                result.push_str("# OUR VERSION:\n");
                result.push_str(&ours);
                result.push_str("# THEIR VERSION:\n");
                result.push_str(&theirs);
                result.push_str("# END MERGED BOM ENTRIES\n");
                
                // Reset for the next conflict
                ours.clear();
                theirs.clear();
                continue;
            }
            
            if in_conflict {
                ours.push_str(line);
                ours.push('\n');
            } else if !ours.is_empty() {
                // We're in the "theirs" section
                theirs.push_str(line);
                theirs.push('\n');
            } else {
                // We're outside a conflict
                result.push_str(line);
                result.push('\n');
            }
        }
        
        Ok(result)
    }
}

/// Strategy for resolving conflicts in BOM files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BomConflictStrategy {
    /// Prefer our version of the BOM
    PreferOurs,
    /// Prefer their version of the BOM
    PreferTheirs,
    /// Merge quantities from both BOMs
    MergeQuantities,
}