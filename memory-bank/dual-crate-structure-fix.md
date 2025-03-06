# Dual Crate Structure Fix Guide

**Navigation:** [productContext](./productContext.md) | [activeContext](./activeContext.md) | [progress](./progress.md) | [decisionLog](./decisionLog.md) | [Memory Bank Index](./memory-bank-index.md)

## Issue Overview

The Implexa project has a dual crate structure:

1. **Library Crate (`lib.rs`)** - Exports core functionality, database models, and Git backend
2. **Binary Crate (`main.rs`)** - Implements the Tauri application using the library crate

The command files are part of the binary crate, but they're trying to access functionality from the library crate using incorrect import paths. Currently, they use `crate::` imports, which refer to the binary crate, but they need to use `implexa::` to import from the library crate.

## Required Changes

### 1. Import Path Fixes

The following command files need their import paths updated from `crate::` to `implexa::`:

- `src/commands.rs`
- `src/part_commands.rs`
- `src/relationship_commands.rs`
- `src/revision_commands.rs`
- `src/workflow_commands.rs`
- `src/file_commands.rs`
- `src/approval_commands.rs`
- `src/manufacturer_part_commands.rs`
- `src/property_commands.rs`

### 2. Specific Type References in `workflow_commands.rs`

In `workflow_commands.rs`, there are specific issues with type references:

1. **State Creation**: The code already uses the renamed type `DbWorkflowState` correctly:
   ```rust
   // This alias is correct:
   use crate::database::workflow::{WorkflowManager, Workflow, WorkflowState as DbWorkflowState, WorkflowTransition};
   
   // And the state is created with the correct alias:
   let state = DbWorkflowState {
       state_id: None,
       workflow_id: state_data.workflow_id,
       // ...
   };
   ```

2. **Transition Creation**: The code already uses the directly imported `WorkflowTransition` correctly:
   ```rust
   let transition = WorkflowTransition {
       transition_id: None,
       workflow_id: transition_data.workflow_id,
       // ...
   };
   ```

The issue with both of these is that they use `crate::` instead of `implexa::` for imports.

## Implementation Steps

To fix these issues, follow these steps:

1. Switch to Code mode to implement these changes
2. For each file in the list above:
   - Replace all instances of `use crate::` with `use implexa::`
   - Ensure all other references to `crate::` are also changed to `implexa::`

## Architectural Considerations

The current architecture with a separate library crate and binary crate is a common and recommended approach for Rust projects, especially those with a GUI or web interface. It provides:

1. **Separation of concerns**: Core business logic vs. application specifics
2. **Code reuse**: The library can be used by multiple applications
3. **Better testing**: The core logic can be tested independently
4. **Clear interfaces**: The binary crate only accesses the library through public interfaces

This change will properly reflect the architectural relationship between the binary crate and the library crate, allowing the command files to access the functionality exported by the library crate.

## Related Files
- [Product Context](./productContext.md) - Project overview and high-level design
- [Rust Module Refactoring Guide](./rust-module-refactoring-guide.md) - Guide for module organization
- [Coding Standards](./coding-standards.md) - Code style and practices