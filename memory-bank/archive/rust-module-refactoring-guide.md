# Rust Module Organization Refactoring Guide

This guide outlines the process for refactoring the Implexa codebase from the `mod.rs` pattern to the filename-as-module pattern.

## Background

As documented in DEC-010, we've decided to switch from the `mod.rs` pattern to the filename-as-module pattern for better maintainability and alignment with modern Rust practices.

### Current Structure (mod.rs pattern)
```
src/
  git_backend/
    mod.rs
    auth/
      mod.rs
    conflict/
      mod.rs
    hook/
      mod.rs
    lfs/
      mod.rs
    operation/
      mod.rs
    repository/
      mod.rs
```

### Target Structure (filename-as-module pattern)
```
src/
  git_backend.rs
  git_backend/
    auth.rs
    conflict.rs
    hook.rs
    lfs.rs
    operation.rs
    repository.rs
```

## Refactoring Steps

### 1. Preparation

1. Create a new branch for this refactoring:
   ```bash
   git checkout -b refactor/rust-modules
   ```

2. Ensure all tests pass before starting:
   ```bash
   cargo test
   ```

### 2. Refactoring Process

For each module with a `mod.rs` file:

1. Create a new file at the same level as the module directory, named after the module with a `.rs` extension.
   - Example: For `src/git_backend/conflict/mod.rs`, create `src/git_backend/conflict.rs`

2. Copy the contents of the `mod.rs` file to the new file.

3. Update any relative imports in the new file if necessary.
   - Imports that referenced sibling modules may need to be updated
   - Example: `use super::repository;` might need to change to `use crate::git_backend::repository;`

4. Update the parent module's declarations:
   - If `src/git_backend/mod.rs` had `pub mod conflict;`, keep it as is
   - The Rust compiler will now look for either `src/git_backend/conflict.rs` or `src/git_backend/conflict/mod.rs`

5. After copying all submodules, move the parent module:
   - Example: Move `src/git_backend/mod.rs` to `src/git_backend.rs`

6. Run tests after each module is moved to catch any issues early:
   ```bash
   cargo test
   ```

### 3. Special Considerations

1. **Nested Submodules**: If any module has nested submodules (e.g., `src/git_backend/auth/provider/mod.rs`), handle these first before moving up to their parent modules.

2. **Private Submodules**: For any non-public submodules (declared with `mod` instead of `pub mod`), ensure they're properly declared in their parent module.

3. **Re-exports**: If a module re-exports items from its submodules, ensure these re-exports still work after refactoring.

### 4. Verification

After completing the refactoring:

1. Ensure all tests pass:
   ```bash
   cargo test
   ```

2. Verify the application builds successfully:
   ```bash
   cargo build
   ```

3. Run any integration tests or manual tests to ensure functionality is preserved.

### 5. Cleanup

1. Remove any empty directories left after the refactoring.

2. Commit the changes:
   ```bash
   git add .
   git commit -m "Refactor: Convert from mod.rs pattern to filename-as-module pattern"
   ```

## Example: Refactoring the conflict module

### Before:
File: `src/git_backend/conflict/mod.rs`
```rust
//! Conflict Resolver
//!
//! This module handles merge conflicts in Git repositories...

use std::path::{Path, PathBuf};
use std::fs;
use git2::{Repository, IndexEntry};
use crate::git_backend::{GitBackendConfig, GitBackendError, Result, ConflictStrategy};

// Rest of the module...
```

### After:
File: `src/git_backend/conflict.rs`
```rust
//! Conflict Resolver
//!
//! This module handles merge conflicts in Git repositories...

use std::path::{Path, PathBuf};
use std::fs;
use git2::{Repository, IndexEntry};
use crate::git_backend::{GitBackendConfig, GitBackendError, Result, ConflictStrategy};

// Rest of the module...
```

The parent module declaration in `src/git_backend.rs` (previously `src/git_backend/mod.rs`) remains unchanged:
```rust
pub mod conflict;
```

## Benefits of This Refactoring

1. **Improved Navigation**: Easier to find modules in editors and IDEs
2. **Flatter Structure**: Reduces directory nesting
3. **Modern Practice**: Aligns with current Rust community standards
4. **Refactoring Friendly**: Makes future module reorganization simpler