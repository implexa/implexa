# Implexa Crate Structure Architecture

**Navigation:** [productContext](./productContext.md) | [activeContext](./activeContext.md) | [progress](./progress.md) | [decisionLog](./decisionLog.md) | [Memory Bank Index](./memory-bank-index.md)

## Current Architecture

The current architecture has duplicate command modules in both crates, leading to import confusion:

```
src/
├── lib.rs                 // Library crate entry point
│   ├── git_backend/       // Git backend functionality
│   ├── database/          // Database functionality 
│   └── commands/          // Library commands module
│
└── main.rs                // Binary crate entry point
    ├── commands.rs        // Command implementations in binary crate
    ├── part_commands.rs   // Part command implementations in binary crate
    ├── relationship_commands.rs  // Relationship command implementations
    └── ... (other command files)
```

Problems with current architecture:
1. **Duplicated Module Names**: Both `lib.rs` and `main.rs` have modules with the same names
2. **Circular Dependencies**: Binary commands depend on library, but library also re-exports binary commands
3. **Import Confusion**: Using `crate::` in binary crate modules but needing functionality from library crate
4. **Build Failures**: Resulting in errors like `can't find crate for 'implexa'`

## Proposed Architecture

The proposed architecture (DEC-022) separates concerns more cleanly:

```
src/
├── lib.rs                 // Library crate entry point
│   ├── git_backend/       // Git backend functionality
│   ├── database/          // Database functionality 
│   └── commands/          // All command implementations in library crate
│       ├── mod.rs         // Re-exports all commands
│       ├── parts.rs       // Part command implementations
│       ├── repository.rs  // Repository command implementations
│       ├── relationships.rs // Relationship command implementations
│       └── ... (other command modules)
│
└── main.rs                // Binary crate entry point (only registers commands)
```

Benefits of proposed architecture:
1. **Clear Separation of Concerns**: Commands defined in library, registered in binary
2. **Single Source of Truth**: All command implementations live in one place
3. **Unidirectional Dependencies**: Binary depends on library, not vice versa
4. **Easier Maintenance**: No duplication or circular references
5. **Better Testing**: Library commands can be tested independently

## Architecture Diagrams

### Current Architecture Flow

```
┌─────────────────────────┐           ┌─────────────────────────┐
│                         │           │                         │
│  Binary Crate (main.rs) │<─────────>│  Library Crate (lib.rs) │
│                         │  Circular │                         │
└───────────┬─────────────┘  Imports  └───────────┬─────────────┘
            │                                     │
            ▼                                     ▼
┌─────────────────────────┐           ┌─────────────────────────┐
│                         │           │                         │
│  Command Files          │ crate::   │  Core Functionality     │
│  - commands.rs          │ vs        │  - git_backend         │
│  - part_commands.rs     │ implexa:: │  - database            │
│  - workflow_commands.rs │ confusion │  - shared types         │
│  - etc.                 │           │                         │
└─────────────────────────┘           └─────────────────────────┘
```

### Proposed Architecture Flow

```
┌─────────────────────────┐           ┌─────────────────────────┐
│                         │           │                         │
│  Binary Crate (main.rs) │───────────│  Library Crate (lib.rs) │
│                         │  Imports  │                         │
└─────────────────────────┘  Only     └───────────┬─────────────┘
  - Registers commands                             │
  - Sets up Tauri application                      │
  - Manages state                                  │
                                                   ▼
                                      ┌─────────────────────────┐
                                      │                         │
                                      │  Core Functionality     │
                                      │  - git_backend         │
                                      │  - database            │
                                      │  - shared types         │
                                      │                         │
                                      └───────────┬─────────────┘
                                                  │
                                                  ▼
                                      ┌─────────────────────────┐
                                      │                         │
                                      │  Command Modules        │
                                      │  - commands/mod.rs      │
                                      │  - commands/parts.rs    │
                                      │  - commands/workflow.rs │
                                      │  - etc.                 │
                                      └─────────────────────────┘
```

## Implementation Approach

1. **Create Command Directory Structure**: 
   - Create `src/commands/` directory
   - Create `src/commands/mod.rs` for re-exports
   - Create files for each command domain (parts.rs, repository.rs, etc.)

2. **Move Command Implementations**:
   - Move commands from binary crate files to library command modules
   - Update imports to use proper paths
   - Ensure consistent error handling and state management

3. **Update Binary Crate**:
   - Remove command module declarations
   - Import commands directly from library
   - Register commands with Tauri

4. **Test & Validate**:
   - Ensure all commands work correctly
   - Verify no circular dependencies
   - Check build succeeds without errors

## Conclusion

This refactoring will create a cleaner, more maintainable architecture with clear responsibility boundaries. It eliminates circular dependencies and strengthens the benefits of the dual crate structure.

## Related Files
- [Crate Structure Refactor Guide](./crate-structure-refactor.md) - Detailed refactoring guidance
- [Decision Log (DEC-022)](./decisionLog.md#dec-022---crate-structure-refactoring) - Architecture decision
- [Dual Crate Structure Fix Guide](./dual-crate-structure-fix.md) - Previous approach (now deprecated)
- [Coding Standards](./coding-standards.md) - Code style and practices