# Implexa: Product Context

**Navigation:** [productContext](./productContext.md) | [activeContext](./activeContext.md) | [progress](./progress.md) | [decisionLog](./decisionLog.md) | [Memory Bank Index](./memory-bank-index.md)

## Project Overview
Implexa is a hardware-focused Product Lifecycle Management (PLM) and Product Data Management (PDM) solution that leverages Git for version control while remaining CAD-agnostic. Built with Tauri and Rust, it bridges the gap between software engineering practices and hardware design workflows, enabling efficient management of design files across multiple CAD platforms.

## Core Problem Addressed
Current PLM/PDM solutions fall into two problematic categories:
1. Standalone systems that require manual file uploads and don't efficiently track design files
2. CAD-embedded systems that only work with their respective CAD suite

Engineers need a solution that works with raw files from any CAD tool while providing structured PLM/PDM capabilities integrated with modern Git-based workflows.

## Key Features
- **Git-Based Version Control**: Use Git for tracking all design files and metadata
- **CAD-Agnostic**: Work with multiple CAD tools, with special focus on KiCad integration
- **Unified Part Library Management**: Consistent structure for all components
- **BOM and Relationship Management**: Track relationships between parts
- **Modern, Lightweight UI**: Built with Tauri for better performance and security
- **Intelligent Part Numbering**: Enhanced hybrid schema with categories and subcategories

## Technical Architecture
- **Frontend**: React with TypeScript and TailwindCSS
- **Backend**: Rust for performance and reliability
- **Application Framework**: Tauri for cross-platform support
- **Database**: SQLite for metadata storage
- **Version Control**: Git with Git-LFS for large file support

## Implementation Approach
The project is being implemented in phases:

1. **Phase 1 (Core Infrastructure)**: Git backend, metadata storage, basic UI
2. **Phase 2 (BOM Management)**: KiCad integration, relationship tracking
3. **Phase 3 (Release Management)**: Workflow automation, manufacturing outputs
4. **Phase 4 (Advanced Features)**: Visual relationship mapping, search, diff tools

## Key Components
1. **Git Backend Manager**: Manages interaction with Git repositories
2. **Metadata Manager**: Uses SQLite database to store and maintain PLM-specific metadata
3. **CAD File Parsers**: Plugins to extract information from various CAD file formats
4. **Workflow Engine**: Defines and enforces workflows for design changes, reviews, and releases
5. **Diff Tools**: Specialized visualization tools for comparing different file types
6. **Part Library Management**: Organizes parts in a unified structure
7. **User Interface**: Tauri application with React frontend

## Part Numbering Strategy
The system uses an enhanced hybrid numbering approach:
```
[Category]-[Subcategory]-[Sequential Number]
```

## Data Storage Model
Each part has a standardized directory structure:
```
/parts/
  /[Category]-[Subcategory]-[Number]/  # Part number
    design/                # Design files
    manufacturing/         # Output files
    documentation/         # Documentation
    metadata.db            # SQLite metadata
```

## Memory Bank Structure
This Memory Bank contains the following core files:
- **productContext.md**: This file - Project overview and high-level design
- **activeContext.md**: Tracks the current session's context and focus
- **progress.md**: Tracks progress and manages tasks
- **decisionLog.md**: Logs architectural decisions and their rationale

## Related Files
- [Progress Tracking](./progress.md) - Current project status and task list
- [Active Context](./activeContext.md) - Current session focus and recent activities
- [Git Backend Architecture](./git-backend-architecture.md) - Design of the Git backend component
- [Database Schema Design](./database-schema-design.md) - SQLite database schema design
- [Part Management Workflow](./part-management-workflow.md) - Part lifecycle workflow design
- [User Interface Architecture](./user-interface-architecture.md) - UI design and components
- [Directory Structure](./directory-structure.md) - File and directory organization

## External References
- [projectBrief.md](../projectBrief.md) - Initial project brief in the root directory
- [README.md](../README.md) - Project README file



# APPENDIX: Implexa: Git-Based PLM/PDM Solution - Product Requirements Document

**Navigation:** [productContext](./productContext.md) | [activeContext](./activeContext.md) | [progress](./progress.md) | [decisionLog](./decisionLog.md) | [Memory Bank Index](./memory-bank-index.md)

## Executive Summary
This document outlines requirements for Implexa, a hardware-focused Product Lifecycle Management (PLM) and Product Data Management (PDM) solution that leverages Git for version control while remaining CAD-agnostic. Built with Tauri and Rust, Implexa aims to bridge the gap between software engineering practices and hardware design workflows, enabling efficient management of design and manufacturing files across multiple CAD platforms, with an initial focus on KiCad integration.

## Problem Statement
Current PLM/PDM solutions fall into two problematic categories:
1. Standalone systems that require manual file uploads and don't efficiently track design files
2. CAD-embedded systems that only work with their respective CAD suite

Engineers need a solution that works with raw files from any CAD tool while providing structured PLM/PDM capabilities integrated with modern Git-based workflows.

## Core Requirements

### 1. Version Control Integration
- **Git-Based Backend**: Utilize Git for underlying version control
- **Branch Strategy Support**: Enable feature branches, release branches, etc. for compatible file formats
- **Lightweight Metadata Layer**: Store PLM metadata alongside design files in Git
- **Hooks Integration**: Leverage Git hooks for workflow automation
- **Large File Handling**: Utilize Git-LFS for binary files and large assets

### 2. CAD Agnosticism
- **File Format Support**: Store and track native files from multiple CAD tools, prioritizing open formats
- **Reference File Management**: Handle cross-references between files
- **Diff Visualization**: Show meaningful visual diffs for BOMs, Gerbers, and 3D models
- **Format Compatibility Tiers**: Implement graduated support based on file format openness

### 3. Component & BOM Management
- **Unified Part Library**: All parts from resistors to complete products follow same structure
- **BOM Generation**: Auto-extract BOMs from design files where possible
- **BOM Importers**: Support CSV/Excel import for mechanical assemblies
- **Relationship Management**: Track parent-child relationships between parts

### 4. Release Management
- **Release Process Automation**: Define and enforce release workflows
- **CI/CD Integration**: Run validation scripts in containers (DRC checks, etc.)
- **Release Packaging**: Generate consistent manufacturing outputs
- **Status Tracking**: Record part status (Draft, In Review, Released, Obsolete)

### 5. User Interface
- **Tauri + Web Frontend**: Local tool with secure, memory-safe backend
- **Deployable Mode**: Same codebase deployable for team access
- **Visual Relationship Mapping**: Show dependencies between components and designs
- **Git Authentication**: Leverage existing Git permissions model

### 6. Part Numbering & Classification
- **Enhanced Hybrid Numbering Schema**: `[Category]-[Subcategory]-[Sequential Number]`
- **Descriptive Subcategories**: Use 2-3 letter subcategory codes for improved human readability
- **Metadata Tags**: Flexible tagging system for detailed categorization
- **Search-First Design**: Powerful search capabilities across metadata

## Architecture & Technology Stack

### Technology Stack
- **Frontend**: Web technologies (HTML, CSS, TypeScript)
- **Frontend Framework**: React with modern state management
- **UI Framework**: TailwindCSS for styling
- **Backend**: Rust for performance, safety, and reliability
- **Application Framework**: Tauri for cross-platform desktop applications
- **Database**: SQLite (via rusqlite) for metadata storage
- **Version Control**: Git (via git2-rs or process invocation)

```
┌─────────────────────────────────────────────────────┐
│                  User Interface                      │
│                                                     │
│  ┌─────────────────────────────────────────────┐    │
│  │    Tauri with Platform Native Webview       │    │
│  └─────────────────────────────────────────────┘    │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│                   Core Services                      │
│                                                     │
│  ┌─────────────┐ ┌─────────────┐ ┌───────────────┐  │
│  │  Metadata   │ │ Workflow    │ │ BOM           │  │
│  │  Manager    │ │ Engine      │ │ Management    │  │
│  └─────────────┘ └─────────────┘ └───────────────┘  │
│                                                     │
│  ┌─────────────┐ ┌─────────────┐ ┌───────────────┐  │
│  │ CAD File    │ │ Release     │ │ Part Library  │  │
│  │ Parsers     │ │ Manager     │ │ Management    │  │
│  └─────────────┘ └─────────────┘ └───────────────┘  │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│                  Git Integration                     │
│                                                     │
│  ┌─────────────┐ ┌─────────────┐ ┌───────────────┐  │
│  │ Git Backend │ │ CI/CD       │ │ Diff          │  │
│  │ Manager     │ │ Pipeline    │ │ Tools         │  │
│  └─────────────┘ └─────────────┘ └───────────────┘  │
└───────────────────────┬─────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────┐
│                  Data Storage                        │
│                                                     │
│  ┌─────────────┐ ┌─────────────┐ ┌───────────────┐  │
│  │ SQLite DB   │ │ Git Repo    │ │ Git-LFS       │  │
│  │ (Metadata)  │ │ (Files)     │ │ (Large Files) │  │
│  └─────────────┘ └─────────────┘ └───────────────┘  │
└─────────────────────────────────────────────────────┘
```

## Key Components

### 1. Git Backend Manager
Manages interaction with Git repositories, handling commits, branches, and merges while preserving PLM metadata. Implements Git-LFS for binary files and configures sparse checkout for efficient handling of large repositories. Implemented in Rust for reliability and performance.

### 2. Metadata Manager
Uses SQLite database to store and maintain PLM-specific metadata (approvals, revisions, part properties) with each part directory. Manages relationships between parts and enforces data integrity. Takes advantage of Rust's strong typing and error handling.

### 3. CAD File Parsers
Plugins to extract information from various CAD file formats, prioritizing open formats (KiCad, FreeCAD, STEP) with graceful degradation for proprietary formats. Extracts BOMs, dimensions, and other critical metadata. Memory-safe parsing in Rust prevents common security vulnerabilities.

### 4. Workflow Engine
Defines and enforces workflows for design changes, reviews, and releases using Git branch strategies and approval gates. Manages part status transitions and approval processes.

### 5. Diff Tools
Specialized visualization tools for comparing:
- BOM/metadata changes (text-based)
- PCB layouts (Gerber visual diff)
- 3D models (STEP/mesh comparison)
- SQLite databases (using specific diff tools for database content)

### 6. Part Library Management
Organizes parts in a unified structure where every component (from resistors to complete products) follows the same organization pattern with appropriate metadata and relationships.

### 7. User Interface
Tauri application with React frontend, providing local file access and Git integration, with optional deployment mode for team access. Optimized for performance with a small resource footprint.

## KiCad Integration

### KiCad Database Integration
- **Direct SQLite Integration**: Access KiCad's native SQLite database directly
- **ODBC Configuration**: Include documentation and setup tools for KiCad's ODBC connection
- **Database Version Control**: Use Git-LFS with SQLite diffing tools for database versioning
- **Data Consistency**: Ensure PLM system respects KiCad's database schema

### Library Component Management
- **Unified Part Numbering**: Use the same part numbering scheme across all assets
- **Symbol-Footprint-Component Relationship**: Leverage KiCad's native relationship model
- **Library File Storage**: Store KiCad symbol, footprint, and 3D model files in Git

### Component Alternates Management
- **Internal Part Numbers**: Use IPN as the primary key for components
- **Manufacturer Part Management**: Link multiple manufacturer parts to each IPN
- **Assembly-Level Constraints**: Track allowed and preferred parts for each position in an assembly

## Implementation Approach

### Phase 1: Core Infrastructure (MVP)
- Implement Git backend manager in Rust
- Create SQLite database schema for part information
- Implement part creation and basic status workflow
- Develop manual metadata entry interface
- Set up unified part directory structure

### Phase 2: BOM Management & KiCad Integration
- Implement KiCad database integration
- Create KiCad library management tools
- Build relationship tracking between parts
- Develop specialized diff tools for KiCad files and SQLite database

### Phase 3: Release Management
- Build release workflow automation
- Implement CI/CD integration for design validation
- Create manufacturing output packaging
- Add approval and signoff process

### Phase 4: Advanced Features
- Develop visual relationship mapping
- Create advanced diff tools for Gerbers and 3D models
- Implement search functionality across metadata
- Build reporting and analytics features

## Part Numbering Strategy

The system will use an enhanced hybrid numbering approach combining category and subcategory codes with sequential numbers:

```
[Category]-[Subcategory]-[Sequential Number]
```

### Primary Categories:
- **EL**: Electronic components and PCBAs
- **ME**: Mechanical parts and assemblies  
- **AS**: Product-level assemblies
- **SW**: Software components
- **DO**: Documentation

### Electronic Subcategories:
- **SYM**: Schematic symbols
- **FPR**: PCB footprints
- **3DM**: 3D models
- **RES**: Resistors
- **CAP**: Capacitors
- **IND**: Inductors
- **ICT**: Integrated circuits
- **DIO**: Diodes
- **FET**: Transistors
- **CON**: Connectors
- **PCB**: Printed circuit boards
- **PCA**: Printed circuit assemblies

Example part numbers:
- `EL-SYM-100001`: Electronic schematic symbol
- `EL-RES-100042`: Resistor component
- `EL-PCB-100103`: PCB design
- `ME-3DM-100054`: Mechanical 3D model

The sequential number will be the primary key in the database, with the category and subcategory codes as metadata. This approach balances human readability with system flexibility.

## Data Storage Model

### Part Structure
Each part will have a standardized directory structure:
```
/parts/
  /[Category]-[Subcategory]-[Number]/  # Part number
    design/                # Design files
    manufacturing/         # Output files
    documentation/         # Documentation
    metadata.db            # SQLite metadata
```

### Library Structure
KiCad libraries will be stored in the repository:
```
/parts/
  /library/
    kicad_library.sqlite   # KiCad parts database
    /symbols/              # Symbol libraries
    /footprints/           # Footprint libraries (.pretty folders)
    /3dmodels/             # 3D model libraries (.3dshapes folders)
```

### Metadata Schema
The SQLite database for parts will contain tables for:
- Basic part information (IPN as primary key)
- Revision history
- Status tracking
- Relationships (parent-child)
- Properties (key-value pairs)
- Manufacturer parts with MPNs
- Assembly part usage and constraints

### External Interfaces
The system will interface with:
- Git hosting services (GitHub, GitLab)
- CI/CD pipelines for validation
- KiCad via ODBC connection
- Optional integrations with supplier databases

## Tauri-Specific Advantages (ie Why did we pick Tauri over Electron?)

### 1. Security
- Granular permissions model with least-privilege access
- Memory safety from Rust preventing common vulnerabilities
- Reduced attack surface compared to Electron

### 2. Performance
- Smaller application size (typically 10-20x smaller than Electron)
- Lower memory usage for long-running sessions
- Faster startup and response times

### 3. Cross-Platform Consistency
- Platform-specific optimizations while maintaining core functionality
- Native OS integration with smaller footprint
- Better accessibility support through native controls

### 4. Development Experience
- Type safety across entire stack
- Robust error handling in backend code
- Better serialization/deserialization with strong typing

### 5. The devs wanted to learn some rust
- :)

## Development Environment Setup

To set up the development environment:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js dependencies
npm install

# Start development mode
npm run tauri dev
```

## Building the Application

To build the application for distribution:

```bash
# Build for all platforms
npm run tauri build

# Build for specific platform
npm run tauri build -- --target [platform]
```

## Testing Strategy

1. **Unit Tests**: Test individual Rust functions and React components
2. **Integration Tests**: Test interaction between frontend and Rust backend
3. **UI Tests**: Test component rendering and user interactions
4. **End-to-End Tests**: Test complete workflows from UI to filesystem


## Project Name Origin
Implexa comes from the Latin word "implex," meaning "interweaving" or "entanglement." The name reflects how the system helps manage the complex, interconnected relationships in hardware product development—parts, assemblies, revisions, alternates, variants, and ECNs all woven together into a coherent system. As a bonus, it contains the letters P, L, and M.

## Conclusion

Implexa represents a modern approach to PLM/PDM for hardware developers, combining Git-based version control with a structured metadata system. Built with Tauri and Rust, it offers superior performance, security, and reliability compared to traditional PLM systems, while maintaining a user-friendly interface that integrates naturally with hardware development workflows.