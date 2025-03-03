# Implexa: Product Context

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

## Project Name Origin
Implexa comes from the Latin word "implex," meaning "interweaving" or "entanglement." The name reflects how the system helps manage the complex, interconnected relationships in hardware product development—parts, assemblies, revisions, alternates, variants, and ECNs all woven together into a coherent system. As a bonus, it contains the letters P, L, and M.