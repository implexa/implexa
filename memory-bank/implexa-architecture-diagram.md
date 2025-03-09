# Implexa Architecture Diagram

**Navigation:** [productContext](./productContext.md) | [activeContext](./activeContext.md) | [progress](./progress.md) | [decisionLog](./decisionLog.md) | [Memory Bank Index](./memory-bank-index.md)

## Overview

This document provides a visual representation of the Implexa PLM system architecture, showing the relationships between major components and the data flow within the application.

## Component Architecture

The Implexa architecture follows a dual-crate structure with clear separation between the library functionality and the binary application layer. The diagram below illustrates the current and proposed architecture for better code organization.

```mermaid
graph TD
    %% Main Application Components
    UI[React UI with TypeScript\nand TailwindCSS]
    TAURI[Tauri Application Shell]
    COMMANDS[Command Interface Layer]
    
    %% Backend Core Components
    DB_CORE[Database Core]
    GIT_CORE[Git Backend]
    PART_MANAGER[Part Management]
    
    %% Database Components
    CONNECTION[Connection Manager\nArc<Mutex<Connection>>]
    SCHEMA[Database Schema]
    DB_TABLES[Database Tables]
    
    %% UI Context and Services
    UI_CONTEXT[UI Context Providers]
    UI_SERVICES[UI Services]
    
    %% Command Types
    PART_COMMANDS[Part Commands]
    RELATIONSHIP_COMMANDS[Relationship Commands]
    REVISION_COMMANDS[Revision Commands]
    FILE_COMMANDS[File Commands]
    WORKFLOW_COMMANDS[Workflow Commands]
    PROPERTY_COMMANDS[Property Commands]
    APPROVAL_COMMANDS[Approval Commands]
    WORKSPACE_COMMANDS[Workspace Commands]
    MANUFACTURER_PART_COMMANDS[Manufacturer Part Commands]
    
    %% Database Models
    PART_MODEL[Part Model]
    REVISION_MODEL[Revision Model]
    RELATIONSHIP_MODEL[Relationship Model]
    APPROVAL_MODEL[Approval Model]
    WORKFLOW_MODEL[Workflow Model]
    FILE_MODEL[File Model]
    PROPERTY_MODEL[Property Model]
    MANUFACTURER_PART_MODEL[Manufacturer Part Model]
    
    %% Connections between UI and Backend
    UI --> TAURI
    TAURI --> COMMANDS
    
    %% Command structure
    COMMANDS --> PART_COMMANDS
    COMMANDS --> RELATIONSHIP_COMMANDS
    COMMANDS --> REVISION_COMMANDS
    COMMANDS --> FILE_COMMANDS
    COMMANDS --> WORKFLOW_COMMANDS
    COMMANDS --> PROPERTY_COMMANDS
    COMMANDS --> APPROVAL_COMMANDS
    COMMANDS --> WORKSPACE_COMMANDS
    COMMANDS --> MANUFACTURER_PART_COMMANDS
    
    %% Command dependencies
    PART_COMMANDS --> PART_MANAGER
    RELATIONSHIP_COMMANDS --> PART_MANAGER
    REVISION_COMMANDS --> PART_MANAGER
    FILE_COMMANDS --> PART_MANAGER
    WORKFLOW_COMMANDS --> PART_MANAGER
    PROPERTY_COMMANDS --> PART_MANAGER
    APPROVAL_COMMANDS --> PART_MANAGER
    WORKSPACE_COMMANDS --> PART_MANAGER
    MANUFACTURER_PART_COMMANDS --> PART_MANAGER
    
    %% Part Manager dependencies
    PART_MANAGER --> DB_CORE
    PART_MANAGER --> GIT_CORE
    
    %% Database Core dependencies
    DB_CORE --> CONNECTION
    DB_CORE --> SCHEMA
    SCHEMA --> DB_TABLES
    
    %% Database Models connections
    DB_TABLES --> PART_MODEL
    DB_TABLES --> REVISION_MODEL
    DB_TABLES --> RELATIONSHIP_MODEL
    DB_TABLES --> APPROVAL_MODEL
    DB_TABLES --> WORKFLOW_MODEL
    DB_TABLES --> FILE_MODEL
    DB_TABLES --> PROPERTY_MODEL
    DB_TABLES --> MANUFACTURER_PART_MODEL
    
    %% UI Components structure
    UI --> UI_CONTEXT
    UI --> UI_SERVICES
    
    %% Styling
    classDef ui fill:#d4f1f9,stroke:#333,stroke-width:1px;
    classDef commands fill:#ffcc99,stroke:#333,stroke-width:1px;
    classDef core fill:#c9e6ff,stroke:#333,stroke-width:1px;
    classDef database fill:#e2f0cb,stroke:#333,stroke-width:1px;
    classDef models fill:#FFE6CC,stroke:#333,stroke-width:1px;
    
    class UI,UI_CONTEXT,UI_SERVICES ui;
    class COMMANDS,PART_COMMANDS,RELATIONSHIP_COMMANDS,REVISION_COMMANDS,FILE_COMMANDS,WORKFLOW_COMMANDS,PROPERTY_COMMANDS,APPROVAL_COMMANDS,WORKSPACE_COMMANDS,MANUFACTURER_PART_COMMANDS commands;
    class DB_CORE,GIT_CORE,PART_MANAGER core;
    class CONNECTION,SCHEMA,DB_TABLES database;
    class PART_MODEL,REVISION_MODEL,RELATIONSHIP_MODEL,APPROVAL_MODEL,WORKFLOW_MODEL,FILE_MODEL,PROPERTY_MODEL,MANUFACTURER_PART_MODEL models;
```

## Crate Structure - Current vs Proposed

The diagram below shows the current crate structure with command duplication issues and the proposed refactored structure:

```mermaid
graph TD
    %% Current Structure
    subgraph "Current Structure"
        MAIN_CRATE[main.rs\nBinary Crate]
        LIB_CRATE[lib.rs\nLibrary Crate]
        
        MAIN_COMMANDS[Command Files\ncommands.rs\npart_commands.rs\nrevision_commands.rs\netc.]
        
        LIB_MODULES[Core Functionality\ngit_backend/\ndatabase/\netc.]
        
        MAIN_CRATE --- MAIN_COMMANDS
        MAIN_CRATE --- LIB_CRATE
        LIB_CRATE --- LIB_MODULES
        
        %% Issue: Circular Dependencies
        MAIN_COMMANDS -.-> LIB_MODULES
        LIB_MODULES -.-> MAIN_COMMANDS
    end
    
    %% Proposed Structure
    subgraph "Proposed Structure"
        MAIN_CRATE2[main.rs\nBinary Crate\n(Command Registration Only)]
        LIB_CRATE2[lib.rs\nLibrary Crate]
        
        LIB_MODULES2[Core Functionality\ngit_backend/\ndatabase/\netc.]
        
        COMMAND_MODULES[Command Modules\ncommands/mod.rs\ncommands/parts.rs\ncommands/workflow.rs\netc.]
        
        MAIN_CRATE2 --> LIB_CRATE2
        LIB_CRATE2 --> LIB_MODULES2
        LIB_CRATE2 --> COMMAND_MODULES
        COMMAND_MODULES --> LIB_MODULES2
    end
```

## Database Structure

The database schema includes the following key tables:

```mermaid
erDiagram
    Parts {
        int64 part_id PK
        string category
        string subcategory
        string name
        string description
        timestamp created_date
        timestamp modified_date
    }
    
    Revisions {
        int64 revision_id PK
        int64 part_id FK
        string version
        string status "Draft, In Review, Released, Obsolete"
        timestamp created_date
        string created_by
        string commit_hash
    }
    
    Relationships {
        int64 relationship_id PK
        int64 parent_part_id FK
        int64 child_part_id FK
        string type
        int quantity
    }
    
    Properties {
        int64 property_id PK
        int64 part_id FK
        int64 revision_id FK
        string key
        string value
        string type
    }
    
    Approvals {
        int64 approval_id PK
        int64 revision_id FK
        string approver
        string status "Pending, Approved, Rejected"
        timestamp date
        string comments
    }
    
    ManufacturerParts {
        int64 mpn_id PK
        int64 part_id FK
        string manufacturer
        string mpn
        string description
        string status "Active, Preferred, Alternate, Obsolete"
    }
    
    Workflows {
        int64 workflow_id PK
        string name
        string description
        boolean active
    }
    
    Files {
        int64 file_id PK
        int64 part_id FK
        int64 revision_id FK
        string path
        string type
        string description
    }
    
    Categories {
        int64 category_id PK
        string name
        string code
        string description
    }
    
    Subcategories {
        int64 subcategory_id PK
        int64 category_id FK
        string name
        string code
        string description
    }
    
    Parts ||--o{ Revisions : "has"
    Parts ||--o{ Properties : "has"
    Parts ||--o{ ManufacturerParts : "has"
    Parts ||--o{ Files : "has"
    Revisions ||--o{ Approvals : "requires"
    Revisions ||--o{ Properties : "has"
    Revisions ||--o{ Files : "has"
    Parts ||--o{ Relationships : "parent"
    Parts ||--o{ Relationships : "child"
    Categories ||--o{ Subcategories : "has"
    Categories ||--o{ Parts : "categorizes"
    Subcategories ||--o{ Parts : "subcategorizes"
```

## Thread Safety Implementation

The connection manager uses a thread-safe pattern with mutex protection:

```mermaid
graph TD
    APP[Tauri Application]
    COMMANDS[Command Handlers]
    CONNECTION_MANAGER[Connection Manager]
    CONNECTION[SQLite Connection]
    
    APP --> COMMANDS
    COMMANDS --> CONNECTION_MANAGER
    CONNECTION_MANAGER -->|"Arc<Mutex<>>>"| CONNECTION
    
    subgraph "Connection Manager Implementation"
        TRANSACTION["transaction()"]
        EXECUTE["execute()"]
        EXECUTE_MUT["execute_mut()"]
        
        TRANSACTION -->|"Lock mutex"| CONNECTION
        EXECUTE -->|"Lock mutex"| CONNECTION
        EXECUTE_MUT -->|"Lock mutex"| CONNECTION
    end
```

## Part Management Workflow

The part lifecycle is managed through state transitions:

```mermaid
stateDiagram-v2
    [*] --> Draft
    Draft --> InReview: submit_for_review()
    InReview --> Draft: reject_revision()
    InReview --> Released: release_revision()
    Released --> Obsolete: mark_as_obsolete()
    Released --> Draft: create_revision()
    Obsolete --> [*]
    
    note right of Draft
        Initial state for new parts
        and rejected revisions
    end note
    
    note right of InReview
        Requires approval from reviewers
        before proceeding to Released
    end note
    
    note right of Released
        Production-ready state
        Can be superseded by new revisions
    end note
    
    note right of Obsolete
        End of lifecycle
        No longer in active use
    end note
```

## Code Structure

Based on the project review, here is the high-level source code structure:

- **src/**
  - **lib.rs** - Library crate entry point
    - **database/** - Database functionality
      - **connection_manager.rs** - SQLite connection management
      - **schema.rs** - Database schema definition
      - **part.rs** - Part model and operations
      - **part_management.rs** - Part lifecycle management
      - **revision.rs** - Revision model and operations
      - **approval.rs** - Approval model and operations 
      - **relationship.rs** - Relationship model and operations
      - **property.rs** - Property model and operations
      - **manufacturer_part.rs** - Manufacturer part model and operations
      - **file.rs** - File tracking model and operations
      - **workflow.rs** - Workflow model and operations
    - **git_backend/** - Git integration functionality
      - **repository.rs** - Git repository operations
      - **operation.rs** - Git operation handling
      - **auth.rs** - Authentication handling
      - **conflict.rs** - Conflict resolution
      - **directory.rs** - Directory management
      - **hook.rs** - Git hook management
      - **lfs.rs** - Git LFS support
  - **main.rs** - Binary crate entry point
    - **commands.rs** - Command registration
    - Various command handlers:
      - **part_commands.rs**
      - **relationship_commands.rs**
      - **revision_commands.rs**
      - **file_commands.rs** 
      - **approval_commands.rs**
      - **manufacturer_part_commands.rs**
      - **property_commands.rs**
      - **workflow_commands.rs**
      - **workspace_commands.rs**
  - **ui/** - Frontend code
    - **App.tsx** - Main application component
    - **components/** - UI components
    - **context/** - React context providers
    - **hooks/** - Custom React hooks
    - **layouts/** - Page layouts
    - **pages/** - Application pages
    - **services/** - Service layer for API calls

## Conclusion

This architecture diagram illustrates the current and planned structure of the Implexa PLM system. The application uses a dual-crate structure where the library crate provides core functionality and the binary crate focuses on exposing that functionality through Tauri commands.

The proposed refactoring will improve code organization by moving all command implementations into the library crate, eliminating circular dependencies, and providing a cleaner separation of concerns.

## Related Files
- [Crate Structure Architecture](./crate-structure-architecture.md) - Details on the crate structure
- [SQLite Thread Safety Implementation](./sqlite-thread-safety-implementation.md) - Details on database connection management
- [Database Schema Design](./database-schema-design.md) - Details on the database schema
- [Part Management Workflow](./part-management-workflow.md) - Details on part lifecycle