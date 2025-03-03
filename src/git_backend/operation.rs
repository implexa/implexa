//! Operation Handler
//!
//! This module provides high-level abstractions for Git operations such as
//! commit, branch, merge, and tag. It ensures atomicity and consistency of operations
//! and maintains operation history and audit trail.

use std::path::{Path, PathBuf};
use git2::{Repository, Oid, Branch, Commit, Signature, BranchType, ObjectType, MergeOptions, MergeAnalysis};
use crate::git_backend::{GitBackendConfig, GitBackendError, Result, MergeResult};

/// Operation Handler
pub struct OperationHandler<'a> {
    /// The Git repository
    repo: &'a Repository,
    /// The Git Backend configuration
    config: &'a GitBackendConfig,
}

impl<'a> OperationHandler<'a> {
    /// Creates a new OperationHandler
    pub fn new(repo: &'a Repository, config: &'a GitBackendConfig) -> Self {
        Self { repo, config }
    }
    
    /// Creates a new commit with the specified message and changes
    pub fn commit(&self, message: &str, files: &[&Path]) -> Result<Oid> {
        // Get the repository index
        let mut index = self.repo.index()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Add the specified files to the index
        for file in files {
            // Convert the path to a relative path if it's absolute
            let repo_path = self.repo.path().parent()
                .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
            
            let relative_path = if file.is_absolute() {
                path_clean::clean(file.strip_prefix(repo_path)
                    .map_err(|_| GitBackendError::OperationError(format!("File {} is not in the repository", file.display())))?)
            } else {
                path_clean::clean(file)
            };
            
            // Add the file to the index
            index.add_path(&relative_path)
                .map_err(|e| GitBackendError::OperationError(format!("Failed to add file {}: {}", relative_path.display(), e)))?;
        }
        
        // Write the index to the repository
        let index_oid = index.write_tree()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Get the tree object
        let tree = self.repo.find_tree(index_oid)
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Get the signature
        let signature = self.repo.signature()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Get the parent commit
        let parent_commit = match self.repo.head() {
            Ok(head) => {
                let head_commit = head.peel_to_commit()
                    .map_err(|e| GitBackendError::GitError(e))?;
                Some(head_commit)
            },
            Err(_) => None,
        };
        
        // Create the commit
        let commit_oid = match parent_commit {
            Some(parent) => {
                self.repo.commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    message,
                    &tree,
                    &[&parent],
                )
            },
            None => {
                self.repo.commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    message,
                    &tree,
                    &[],
                )
            },
        }.map_err(|e| GitBackendError::GitError(e))?;
        
        Ok(commit_oid)
    }
    
    /// Creates a new branch with the specified name
    pub fn create_branch(&self, name: &str) -> Result<Branch> {
        // Get the HEAD commit
        let head = self.repo.head()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        let head_commit = head.peel_to_commit()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Create the branch
        let branch = self.repo.branch(name, &head_commit, false)
            .map_err(|e| GitBackendError::OperationError(format!("Failed to create branch {}: {}", name, e)))?;
        
        Ok(branch)
    }
    
    /// Switches to the specified branch
    pub fn checkout_branch(&self, name: &str) -> Result<()> {
        // Find the branch
        let branch = self.repo.find_branch(name, BranchType::Local)
            .map_err(|e| GitBackendError::OperationError(format!("Failed to find branch {}: {}", name, e)))?;
        
        // Get the branch reference
        let branch_ref = branch.get()
            .name()
            .ok_or_else(|| GitBackendError::OperationError(format!("Branch {} has no name", name)))?;
        
        // Get the branch commit
        let obj = self.repo.revparse_single(branch_ref)
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Create a checkout builder
        let mut checkout_builder = git2::build::CheckoutBuilder::new();
        checkout_builder.force();
        
        // Checkout the branch
        self.repo.checkout_tree(&obj, Some(&mut checkout_builder))
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Set the HEAD to the branch
        self.repo.set_head(branch_ref)
            .map_err(|e| GitBackendError::GitError(e))?;
        
        Ok(())
    }
    
    /// Merges the specified branch into the current branch
    pub fn merge_branch(&self, name: &str) -> Result<MergeResult> {
        // Find the branch to merge
        let branch = self.repo.find_branch(name, BranchType::Local)
            .map_err(|e| GitBackendError::OperationError(format!("Failed to find branch {}: {}", name, e)))?;
        
        // Get the annotated commit for the branch
        let branch_commit = branch.get().peel_to_commit()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        let annotated_commit = self.repo.find_annotated_commit(branch_commit.id())
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Get the current branch
        let head = self.repo.head()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Analyze the merge
        let analysis = self.repo.merge_analysis(&[&annotated_commit])
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Check if the merge is up-to-date
        if analysis.0.is_up_to_date() {
            return Ok(MergeResult {
                success: true,
                has_conflicts: false,
                conflicted_files: Vec::new(),
                commit_id: Some(branch_commit.id().to_string()),
            });
        }
        
        // Check if the merge is fast-forward
        if analysis.0.is_fast_forward() {
            // Get the reference to update
            let mut reference = head;
            
            // Fast-forward the reference
            reference.set_target(branch_commit.id(), "Fast-forward merge")
                .map_err(|e| GitBackendError::GitError(e))?;
            
            // Update the working directory
            let mut checkout_builder = git2::build::CheckoutBuilder::new();
            checkout_builder.force();
            
            self.repo.checkout_tree(&self.repo.find_object(branch_commit.id(), None)
                .map_err(|e| GitBackendError::GitError(e))?, Some(&mut checkout_builder))
                .map_err(|e| GitBackendError::GitError(e))?;
            
            return Ok(MergeResult {
                success: true,
                has_conflicts: false,
                conflicted_files: Vec::new(),
                commit_id: Some(branch_commit.id().to_string()),
            });
        }
        
        // Normal merge
        if analysis.0.is_normal() {
            // Start the merge
            let mut merge_options = MergeOptions::new();
            merge_options.fail_on_conflict(false);
            
            let mut checkout_builder = git2::build::CheckoutBuilder::new();
            checkout_builder.force();
            
            self.repo.merge(&[&annotated_commit], Some(&mut merge_options), Some(&mut checkout_builder))
                .map_err(|e| GitBackendError::GitError(e))?;
            
            // Check for conflicts
            let has_conflicts = self.repo.index()
                .map_err(|e| GitBackendError::GitError(e))?
                .has_conflicts();
            
            if has_conflicts {
                // Get the list of conflicted files
                let mut conflicted_files = Vec::new();
                let index = self.repo.index()
                    .map_err(|e| GitBackendError::GitError(e))?;
                
                for entry in index.iter() {
                    // Check if this path has conflicts using the conflicts iterator
                    let path_str = String::from_utf8_lossy(&entry.path).to_string();
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
                        conflicted_files.push(PathBuf::from(path_str));
                    }
                }
                
                return Ok(MergeResult {
                    success: false,
                    has_conflicts: true,
                    conflicted_files,
                    commit_id: None,
                });
            }
            
            // Create the merge commit
            let index_oid = self.repo.index()
                .map_err(|e| GitBackendError::GitError(e))?
                .write_tree()
                .map_err(|e| GitBackendError::GitError(e))?;
            
            let tree = self.repo.find_tree(index_oid)
                .map_err(|e| GitBackendError::GitError(e))?;
            
            let signature = self.repo.signature()
                .map_err(|e| GitBackendError::GitError(e))?;
            
            let head_commit = head.peel_to_commit()
                .map_err(|e| GitBackendError::GitError(e))?;
            
            let message = format!("Merge branch '{}' into {}", name, head.shorthand().unwrap_or("HEAD"));
            
            let commit_oid = self.repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &message,
                &tree,
                &[&head_commit, &branch_commit],
            ).map_err(|e| GitBackendError::GitError(e))?;
            
            // Clean up the merge state
            self.repo.cleanup_state()
                .map_err(|e| GitBackendError::GitError(e))?;
            
            return Ok(MergeResult {
                success: true,
                has_conflicts: false,
                conflicted_files: Vec::new(),
                commit_id: Some(commit_oid.to_string()),
            });
        }
        
        Err(GitBackendError::OperationError(format!("Unsupported merge type for branch {}", name)))
    }
    
    /// Creates a new tag with the specified name and message
    pub fn create_tag(&self, name: &str, message: &str) -> Result<Oid> {
        // Get the HEAD commit
        let head = self.repo.head()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        let head_commit = head.peel_to_commit()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Get the signature
        let signature = self.repo.signature()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Create the tag
        let tag_oid = self.repo.tag(
            name,
            &head_commit.into_object(),
            &signature,
            message,
            false,
        ).map_err(|e| GitBackendError::OperationError(format!("Failed to create tag {}: {}", name, e)))?;
        
        Ok(tag_oid)
    }
    
    /// Gets the commit history for the specified reference
    pub fn get_history(&self, reference: &str) -> Result<Vec<Commit>> {
        // Get the reference
        let obj = self.repo.revparse_single(reference)
            .map_err(|e| GitBackendError::OperationError(format!("Failed to find reference {}: {}", reference, e)))?;
        
        // Get the commit
        let commit = obj.peel_to_commit()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Create a revwalk
        let mut revwalk = self.repo.revwalk()
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Configure the revwalk
        revwalk.push(commit.id())
            .map_err(|e| GitBackendError::GitError(e))?;
        
        // Collect the commits
        let mut commits = Vec::new();
        for oid in revwalk {
            let oid = oid.map_err(|e| GitBackendError::GitError(e))?;
            let commit = self.repo.find_commit(oid)
                .map_err(|e| GitBackendError::GitError(e))?;
            commits.push(commit);
        }
        
        Ok(commits)
    }
}