//! Hook Manager
//!
//! This module handles Git hooks for workflow automation. It provides functionality
//! for setting up and managing Git hooks, providing hook templates for common PLM workflows,
//! and ensuring hooks are maintained across repository operations.

use std::path::PathBuf;
use std::fs::{self, OpenOptions};
use std::io::{Write};
use std::os::windows::fs::OpenOptionsExt;
use git2::Repository;
use crate::git_backend::{GitBackendConfig, GitBackendError, Result, HookType};

/// Hook Manager
pub struct HookManager<'a> {
    /// The Git repository
    repo: &'a Repository,
    /// The Git Backend configuration
    config: &'a GitBackendConfig,
}

impl<'a> HookManager<'a> {
    /// Creates a new HookManager
    pub fn new(repo: &'a Repository, config: &'a GitBackendConfig) -> Self {
        Self { repo, config }
    }
    
    /// Installs the specified hook
    pub fn install_hook(&self, hook_type: HookType, script: &str) -> Result<()> {
        // Get the hook path
        let hook_path = self.get_hook_path(hook_type)?;
        
        // Create the hook directory if it doesn't exist
        if let Some(parent) = hook_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| GitBackendError::IoError(e))?;
        }
        
        // Write the hook script
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .custom_flags(0o755) // Set executable permission
            .open(&hook_path)
            .map_err(|e| GitBackendError::IoError(e))?;
        
        // Add shebang if not present
        let script_with_shebang = if !script.starts_with("#!") {
            format!("#!/bin/sh\n{}", script)
        } else {
            script.to_string()
        };
        
        // Write the script
        file.write_all(script_with_shebang.as_bytes())
            .map_err(|e| GitBackendError::IoError(e))?;
        
        Ok(())
    }
    
    /// Removes the specified hook
    pub fn remove_hook(&self, hook_type: HookType) -> Result<()> {
        // Get the hook path
        let hook_path = self.get_hook_path(hook_type)?;
        
        // Remove the hook if it exists
        if hook_path.exists() {
            fs::remove_file(&hook_path)
                .map_err(|e| GitBackendError::IoError(e))?;
        }
        
        Ok(())
    }
    
    /// Gets the script for the specified hook
    pub fn get_hook(&self, hook_type: HookType) -> Result<String> {
        // Get the hook path
        let hook_path = self.get_hook_path(hook_type)?;
        
        // Read the hook script
        if hook_path.exists() {
            fs::read_to_string(&hook_path)
                .map_err(|e| GitBackendError::IoError(e))
        } else {
            Ok(String::new())
        }
    }
    
    /// Installs the default PLM hooks
    pub fn install_default_plm_hooks(&self) -> Result<()> {
        // Install pre-commit hook
        self.install_hook(HookType::PreCommit, PRE_COMMIT_HOOK)?;
        
        // Install post-commit hook
        self.install_hook(HookType::PostCommit, POST_COMMIT_HOOK)?;
        
        // Install pre-merge hook
        self.install_hook(HookType::PreMerge, PRE_MERGE_HOOK)?;
        
        // Install post-merge hook
        self.install_hook(HookType::PostMerge, POST_MERGE_HOOK)?;
        
        // Install pre-push hook
        self.install_hook(HookType::PrePush, PRE_PUSH_HOOK)?;
        
        // Install post-checkout hook
        self.install_hook(HookType::PostCheckout, POST_CHECKOUT_HOOK)?;
        
        Ok(())
    }
    
    /// Gets the path to the specified hook
    fn get_hook_path(&self, hook_type: HookType) -> Result<PathBuf> {
        // Get the hooks directory
        let hooks_dir = self.repo.path().join("hooks");
        
        // Get the hook name
        let hook_name = match hook_type {
            HookType::PreCommit => "pre-commit",
            HookType::PostCommit => "post-commit",
            HookType::PreMerge => "pre-merge-commit",
            HookType::PostMerge => "post-merge",
            HookType::PrePush => "pre-push",
            HookType::PostPush => "post-push",
            HookType::PostCheckout => "post-checkout",
            HookType::PostClone => "post-clone",
            HookType::PostInit => "post-init",
            HookType::PostUpdate => "post-update",
            HookType::PostReferenceUpdate => "post-reference-update",
        };
        
        Ok(hooks_dir.join(hook_name))
    }
    
    /// Installs a workflow hook
    pub fn install_workflow_hook(
        &self,
        hook_type: HookType,
        workflow: &Workflow,
    ) -> Result<()> {
        // Convert the workflow to a script
        let script = self.workflow_to_script(workflow)?;
        
        // Install the hook
        self.install_hook(hook_type, &script)
    }
    
    /// Converts a workflow to a script
    fn workflow_to_script(&self, workflow: &Workflow) -> Result<String> {
        // Get the repository path
        let repo_path = self.repo.path().parent()
            .ok_or_else(|| GitBackendError::RepositoryError("Repository path has no parent".to_string()))?;
        
        // Build the script
        let mut script = String::new();
        
        // Add shebang
        script.push_str("#!/bin/sh\n\n");
        
        // Add workflow name and description
        script.push_str(&format!("# Workflow: {}\n", workflow.name));
        if let Some(description) = &workflow.description {
            script.push_str(&format!("# Description: {}\n\n", description));
        } else {
            script.push_str("\n");
        }
        
        // Add workflow steps
        for (i, step) in workflow.steps.iter().enumerate() {
            script.push_str(&format!("# Step {}: {}\n", i + 1, step.name));
            
            // Add step command
            script.push_str(&format!("{}\n\n", step.command));
            
            // Add error handling
            script.push_str("if [ $? -ne 0 ]; then\n");
            script.push_str(&format!("  echo \"Error in step {}: {}\"\n", i + 1, step.name));
            script.push_str("  exit 1\n");
            script.push_str("fi\n\n");
        }
        
        // Add success message
        script.push_str(&format!("echo \"Workflow '{}' completed successfully\"\n", workflow.name));
        script.push_str("exit 0\n");
        
        Ok(script)
    }
}

/// Workflow for Git hooks
#[derive(Debug, Clone)]
pub struct Workflow {
    /// Name of the workflow
    pub name: String,
    /// Description of the workflow
    pub description: Option<String>,
    /// Steps in the workflow
    pub steps: Vec<WorkflowStep>,
}

/// Step in a workflow
#[derive(Debug, Clone)]
pub struct WorkflowStep {
    /// Name of the step
    pub name: String,
    /// Command to execute
    pub command: String,
}

// Default hook scripts

/// Default pre-commit hook
const PRE_COMMIT_HOOK: &str = r#"#!/bin/sh
#
# Implexa pre-commit hook
#
# This hook is called before a commit is created.
# It checks for PLM metadata and validates the commit.

# Check for PLM metadata
if [ ! -f "metadata.db" ]; then
  echo "Error: PLM metadata not found. Please create metadata before committing."
  exit 1
fi

# Check for binary files without LFS
git diff --cached --name-only | grep -E '\.(pdf|png|jpg|step|stl|zip|bin)$' | while read file; do
  if ! git check-attr filter "$file" | grep -q "lfs"; then
    echo "Error: Binary file $file is not tracked by Git LFS."
    echo "Please run: git lfs track \"$file\""
    exit 1
  fi
done

# Run any custom validation scripts
if [ -f ".implexa/hooks/pre-commit-custom" ]; then
  ./.implexa/hooks/pre-commit-custom
  if [ $? -ne 0 ]; then
    echo "Error: Custom pre-commit validation failed."
    exit 1
  fi
fi

exit 0
"#;

/// Default post-commit hook
const POST_COMMIT_HOOK: &str = r#"#!/bin/sh
#
# Implexa post-commit hook
#
# This hook is called after a commit is created.
# It updates PLM metadata and performs post-commit actions.

# Get the commit hash
COMMIT_HASH=$(git rev-parse HEAD)

# Update PLM metadata
if [ -f "metadata.db" ]; then
  echo "Updating PLM metadata for commit $COMMIT_HASH"
  # This would be replaced with actual metadata update command
fi

# Run any custom post-commit scripts
if [ -f ".implexa/hooks/post-commit-custom" ]; then
  ./.implexa/hooks/post-commit-custom
fi

exit 0
"#;

/// Default pre-merge hook
const PRE_MERGE_HOOK: &str = r#"#!/bin/sh
#
# Implexa pre-merge hook
#
# This hook is called before a merge is performed.
# It validates the merge and checks for conflicts.

# Get the branch being merged
BRANCH_NAME=$(git rev-parse --abbrev-ref HEAD)

# Check for PLM metadata conflicts
echo "Checking for PLM metadata conflicts in branch $BRANCH_NAME"
# This would be replaced with actual metadata conflict check

# Run any custom pre-merge scripts
if [ -f ".implexa/hooks/pre-merge-custom" ]; then
  ./.implexa/hooks/pre-merge-custom
  if [ $? -ne 0 ]; then
    echo "Error: Custom pre-merge validation failed."
    exit 1
  fi
fi

exit 0
"#;

/// Default post-merge hook
const POST_MERGE_HOOK: &str = r#"#!/bin/sh
#
# Implexa post-merge hook
#
# This hook is called after a merge is performed.
# It updates PLM metadata and performs post-merge actions.

# Check if the merge was successful
if [ $? -ne 0 ]; then
  echo "Merge failed, skipping post-merge actions."
  exit 0
fi

# Update PLM metadata
if [ -f "metadata.db" ]; then
  echo "Updating PLM metadata after merge"
  # This would be replaced with actual metadata update command
fi

# Pull LFS objects
git lfs pull

# Run any custom post-merge scripts
if [ -f ".implexa/hooks/post-merge-custom" ]; then
  ./.implexa/hooks/post-merge-custom
fi

exit 0
"#;

/// Default pre-push hook
const PRE_PUSH_HOOK: &str = r#"#!/bin/sh
#
# Implexa pre-push hook
#
# This hook is called before a push is performed.
# It validates the push and checks for required metadata.

# Check for PLM metadata
if [ ! -f "metadata.db" ]; then
  echo "Error: PLM metadata not found. Please create metadata before pushing."
  exit 1
fi

# Check for binary files without LFS
git diff --name-only | grep -E '\.(pdf|png|jpg|step|stl|zip|bin)$' | while read file; do
  if ! git check-attr filter "$file" | grep -q "lfs"; then
    echo "Error: Binary file $file is not tracked by Git LFS."
    echo "Please run: git lfs track \"$file\""
    exit 1
  fi
done

# Run any custom pre-push scripts
if [ -f ".implexa/hooks/pre-push-custom" ]; then
  ./.implexa/hooks/pre-push-custom
  if [ $? -ne 0 ]; then
    echo "Error: Custom pre-push validation failed."
    exit 1
  fi
fi

exit 0
"#;

/// Default post-checkout hook
const POST_CHECKOUT_HOOK: &str = r#"#!/bin/sh
#
# Implexa post-checkout hook
#
# This hook is called after a checkout is performed.
# It updates PLM metadata and performs post-checkout actions.

# Get the previous and current branch/commit
PREV_HEAD=$1
NEW_HEAD=$2
BRANCH_CHECKOUT=$3

# Pull LFS objects
git lfs pull

# Update PLM metadata
if [ -f "metadata.db" ]; then
  echo "Updating PLM metadata after checkout"
  # This would be replaced with actual metadata update command
fi

# Run any custom post-checkout scripts
if [ -f ".implexa/hooks/post-checkout-custom" ]; then
  ./.implexa/hooks/post-checkout-custom
fi

exit 0
"#;