# Git-Based PLM/PDM Solution: Product Requirements Document

## Executive Summary
This document outlines requirements for a hardware-focused Product Lifecycle Management (PLM) and Product Data Management (PDM) solution that leverages Git for version control while remaining CAD-agnostic. The system aims to bridge the gap between software engineering practices and hardware design workflows, enabling efficient management of design files across multiple CAD platforms, with specific focus on KiCad integration.

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
- **Electron + Web Application**: Local tool with embedded web server
- **Deployable Mode**: Same codebase deployable for team access
- **Visual Relationship Mapping**: Show dependencies between components and designs
- **Git Authentication**: Leverage existing Git permissions model

### 6. Part Numbering & Classification
- **Enhanced Hybrid Numbering Schema**: `[Category]-[Subcategory]-[Sequential Number]`
- **Descriptive Subcategories**: Use 2-3 letter subcategory codes for improved human readability
- **Metadata Tags**: Flexible tagging system for detailed categorization
- **Search-First Design**: Powerful search capabilities across metadata

## High-Level Architecture

```
┌─────────────────────────────────────────────────────┐
│                  User Interface                      │
│                                                     │
│  ┌─────────────────────────────────────────────┐    │
│  │ Electron Application with Embedded Web Server│    │
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
Manages interaction with Git repositories, handling commits, branches, and merges while preserving PLM metadata. Implements Git-LFS for binary files and configures sparse checkout for efficient handling of large repositories.

### 2. Metadata Manager
Uses SQLite database to store and maintain PLM-specific metadata (approvals, revisions, part properties) with each part directory. Manages relationships between parts and enforces data integrity.

### 3. CAD File Parsers
Plugins to extract information from various CAD file formats, prioritizing open formats (KiCad, FreeCAD, STEP) with graceful degradation for proprietary formats. Extracts BOMs, dimensions, and other critical metadata.

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
Electron application with embedded web server providing local file access and Git integration, with optional deployment mode for team access.

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
- Implement Git backend with basic metadata storage
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
- `EL-SYM-10001`: Electronic schematic symbol
- `EL-RES-10042`: Resistor component
- `EL-PCB-10103`: PCB design
- `ME-3DM-10054`: Mechanical 3D model

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