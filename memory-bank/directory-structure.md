# Directory Structure

## Overview

The directory structure for Implexa defines how files and directories are organized within the system, both at the repository level and within individual part directories. This document outlines the standardized directory structure, naming conventions, and organization patterns that ensure consistency and maintainability across the project.

## Core Principles

1. **Consistency**: Maintain a consistent structure across all parts and repositories
2. **Discoverability**: Make it easy to find files and understand their purpose
3. **Separation of Concerns**: Keep different types of files organized by their purpose
4. **Git-Friendly**: Optimize for Git operations and minimize conflicts
5. **CAD-Agnostic**: Support multiple CAD tools while maintaining consistency

## Repository Structure

The top-level repository structure organizes the entire PLM system:

```
/
├── parts/                  # All part data
│   ├── [Category]-[Subcategory]-[Number]/  # Individual part directories
│   └── library/            # Shared library components
├── templates/              # Templates for new parts and other entities
├── scripts/                # Utility scripts and tools
├── config/                 # Configuration files
│   ├── workflows/          # Workflow definitions
│   ├── categories/         # Category and subcategory definitions
│   └── settings/           # Application settings
├── .git/                   # Git repository data
└── .gitattributes          # Git attributes for LFS and line endings
```

## Part Directory Structure

Each part has a standardized directory structure:

```
/parts/[Category]-[Subcategory]-[Number]/  # Part number (e.g., EL-PCB-10001)
├── metadata.db             # SQLite database with part metadata
├── README.md               # Human-readable part description
├── design/                 # Design files
│   ├── [CAD-specific files]
│   ├── models/             # 3D models
│   └── schematics/         # Schematics and diagrams
├── manufacturing/          # Manufacturing output files
│   ├── gerbers/            # Gerber files for PCBs
│   ├── bom/                # Bill of Materials
│   ├── assembly/           # Assembly instructions
│   └── drawings/           # Technical drawings
├── documentation/          # Documentation files
│   ├── datasheets/         # Component datasheets
│   ├── specifications/     # Specifications
│   └── user-guides/        # User guides
└── tests/                  # Test files and results
    ├── test-plans/         # Test plans
    ├── test-results/       # Test results
    └── test-fixtures/      # Test fixtures
```

### Design Directory

The `design` directory contains all files related to the design of the part:

```
/design/
├── [CAD-specific files]    # Native CAD files
├── models/                 # 3D models
│   ├── step/               # STEP format models
│   ├── stl/                # STL format models
│   └── [other formats]/    # Other 3D model formats
├── schematics/             # Schematics and diagrams
│   ├── pdf/                # PDF exports
│   └── svg/                # SVG exports
└── source/                 # Source files for design
    ├── kicad/              # KiCad project files
    ├── freecad/            # FreeCAD project files
    └── [other CAD tools]/  # Files for other CAD tools
```

### Manufacturing Directory

The `manufacturing` directory contains all files needed for manufacturing:

```
/manufacturing/
├── gerbers/                # Gerber files for PCBs
│   ├── [layer files]       # Individual layer files
│   └── [drill files]       # Drill files
├── bom/                    # Bill of Materials
│   ├── bom.csv             # CSV format BOM
│   ├── bom.xlsx            # Excel format BOM
│   └── bom.json            # JSON format BOM
├── assembly/               # Assembly instructions
│   ├── assembly.md         # Assembly instructions in Markdown
│   └── images/             # Images for assembly instructions
└── drawings/               # Technical drawings
    ├── pdf/                # PDF format drawings
    └── dxf/                # DXF format drawings
```

### Documentation Directory

The `documentation` directory contains all documentation related to the part:

```
/documentation/
├── datasheets/             # Component datasheets
│   └── [component datasheets].pdf
├── specifications/         # Specifications
│   ├── electrical/         # Electrical specifications
│   ├── mechanical/         # Mechanical specifications
│   └── environmental/      # Environmental specifications
└── user-guides/            # User guides
    ├── installation/       # Installation guides
    ├── operation/          # Operation guides
    └── maintenance/        # Maintenance guides
```

### Tests Directory

The `tests` directory contains all test-related files:

```
/tests/
├── test-plans/             # Test plans
│   ├── functional/         # Functional test plans
│   ├── performance/        # Performance test plans
│   └── compliance/         # Compliance test plans
├── test-results/           # Test results
│   ├── functional/         # Functional test results
│   ├── performance/        # Performance test results
│   └── compliance/         # Compliance test results
└── test-fixtures/          # Test fixtures
    ├── hardware/           # Hardware test fixtures
    └── software/           # Software test fixtures
```

## Library Structure

The `library` directory contains shared components that can be used across multiple parts:

```
/parts/library/
├── kicad_library.sqlite    # KiCad parts database
├── symbols/                # Symbol libraries
│   ├── EL-SYM-[Number]/    # Individual symbol directories
│   └── [library files]     # Library files
├── footprints/             # Footprint libraries
│   ├── EL-FPR-[Number]/    # Individual footprint directories
│   └── [library files]     # Library files
├── 3dmodels/               # 3D model libraries
│   ├── EL-3DM-[Number]/    # Individual 3D model directories
│   └── [library files]     # Library files
└── components/             # Component libraries
    ├── EL-[Subcategory]-[Number]/  # Individual component directories
    └── [library files]     # Library files
```

### Symbol Directory Structure

Each symbol has a standardized directory structure:

```
/parts/library/symbols/EL-SYM-[Number]/
├── metadata.db             # SQLite database with symbol metadata
├── README.md               # Human-readable symbol description
└── design/                 # Symbol design files
    ├── [CAD-specific files]
    └── exports/            # Exported formats
```

### Footprint Directory Structure

Each footprint has a standardized directory structure:

```
/parts/library/footprints/EL-FPR-[Number]/
├── metadata.db             # SQLite database with footprint metadata
├── README.md               # Human-readable footprint description
└── design/                 # Footprint design files
    ├── [CAD-specific files]
    └── exports/            # Exported formats
```

### 3D Model Directory Structure

Each 3D model has a standardized directory structure:

```
/parts/library/3dmodels/EL-3DM-[Number]/
├── metadata.db             # SQLite database with 3D model metadata
├── README.md               # Human-readable 3D model description
└── design/                 # 3D model design files
    ├── [CAD-specific files]
    ├── step/               # STEP format
    ├── stl/                # STL format
    └── [other formats]/    # Other 3D model formats
```

### Component Directory Structure

Each component has a standardized directory structure:

```
/parts/library/components/EL-[Subcategory]-[Number]/
├── metadata.db             # SQLite database with component metadata
├── README.md               # Human-readable component description
├── symbol/                 # Symbol reference
├── footprint/              # Footprint reference
├── 3dmodel/                # 3D model reference
└── documentation/          # Component documentation
    └── datasheets/         # Component datasheets
```

## Templates Directory

The `templates` directory contains templates for creating new parts and other entities:

```
/templates/
├── parts/                  # Part templates
│   ├── electronic/         # Electronic part templates
│   ├── mechanical/         # Mechanical part templates
│   └── assembly/           # Assembly templates
├── library/                # Library templates
│   ├── symbol/             # Symbol templates
│   ├── footprint/          # Footprint templates
│   ├── 3dmodel/            # 3D model templates
│   └── component/          # Component templates
└── documentation/          # Documentation templates
    ├── readme/             # README templates
    ├── specification/      # Specification templates
    └── user-guide/         # User guide templates
```

## Scripts Directory

The `scripts` directory contains utility scripts and tools:

```
/scripts/
├── setup/                  # Setup scripts
│   ├── install.sh          # Installation script
│   └── configure.sh        # Configuration script
├── import/                 # Import scripts
│   ├── kicad/              # KiCad import scripts
│   └── [other CAD tools]/  # Import scripts for other CAD tools
├── export/                 # Export scripts
│   ├── bom/                # BOM export scripts
│   └── gerber/             # Gerber export scripts
├── validation/             # Validation scripts
│   ├── drc/                # Design rule check scripts
│   └── erc/                # Electrical rule check scripts
└── ci/                     # Continuous integration scripts
    ├── build/              # Build scripts
    └── test/               # Test scripts
```

## Config Directory

The `config` directory contains configuration files:

```
/config/
├── workflows/              # Workflow definitions
│   ├── default.json        # Default workflow
│   └── [custom workflows]  # Custom workflows
├── categories/             # Category and subcategory definitions
│   ├── categories.json     # Category definitions
│   └── subcategories.json  # Subcategory definitions
└── settings/               # Application settings
    ├── app.json            # Application settings
    ├── git.json            # Git settings
    └── user.json           # User settings
```

## File Naming Conventions

### General Naming Conventions

1. Use lowercase for all filenames and directories
2. Use hyphens (-) to separate words in filenames
3. Use descriptive names that indicate the purpose of the file
4. Include version numbers in filenames when appropriate
5. Use standard file extensions

### Part Numbering

Parts are numbered using the category-subcategory-sequential schema:

```
[Category]-[Subcategory]-[Sequential Number]
```

Examples:
- `EL-PCB-10001`: Electronic PCB part
- `ME-HSG-10042`: Mechanical housing part
- `AS-PRD-10103`: Product assembly

### Version Numbering

Version numbers are included in filenames when appropriate:

```
[filename]-v[major].[minor].[patch].[revision]
```

Examples:
- `schematic-v1.0.0.pdf`: Version 1.0.0 of a schematic
- `assembly-v2.1.3.md`: Version 2.1.3 of assembly instructions

### Date Formatting

Dates in filenames follow the ISO 8601 format:

```
[filename]-[YYYY-MM-DD]
```

Examples:
- `test-results-2025-03-02.pdf`: Test results from March 2, 2025
- `meeting-notes-2025-03-15.md`: Meeting notes from March 15, 2025

## Git Integration

The directory structure is designed to work well with Git:

### Git Attributes

The `.gitattributes` file configures Git for the repository:

```
# Set default line ending behavior
* text=auto

# Binary files
*.pdf binary
*.png binary
*.jpg binary
*.step binary
*.stl binary

# LFS tracked files
*.pdf filter=lfs diff=lfs merge=lfs -text
*.png filter=lfs diff=lfs merge=lfs -text
*.jpg filter=lfs diff=lfs merge=lfs -text
*.step filter=lfs diff=lfs merge=lfs -text
*.stl filter=lfs diff=lfs merge=lfs -text
*.zip filter=lfs diff=lfs merge=lfs -text
*.bin filter=lfs diff=lfs merge=lfs -text
```

### Git Ignore

The `.gitignore` file specifies files that should not be tracked by Git:

```
# Build artifacts
build/
dist/
out/

# Temporary files
*.tmp
*.temp
*.bak
*.swp
*~

# OS-specific files
.DS_Store
Thumbs.db

# IDE files
.idea/
.vscode/
*.sublime-*

# Log files
*.log
logs/

# Dependency directories
node_modules/
vendor/

# Environment files
.env
.env.local
```

### Branch Structure

The branch structure aligns with the part management workflow:

1. `main`: Contains all released and obsolete parts
2. `part/[IPN]/draft`: Feature branches for drafting new parts and revisions
3. `part/[IPN]/review`: Review branches for reviewing parts before release

## Database Integration

The directory structure integrates with the SQLite database:

1. Each part directory contains a `metadata.db` file with part-specific metadata
2. The database stores references to files in the directory structure
3. File paths in the database are relative to the part directory
4. The database tracks file versions and changes

## Workspace Management

The directory structure supports workspace management for multiple parts in draft state:

1. Each workspace corresponds to a specific part in draft state
2. Workspaces are created by checking out the appropriate feature branch
3. The system maintains separate working directories for each workspace
4. CAD tools are configured to use the appropriate workspace directory

## Implementation Approach

The directory structure will be implemented in phases:

### Phase 1: Basic Structure

1. Implement the top-level repository structure
2. Implement the basic part directory structure
3. Set up Git attributes and ignore files
4. Create basic templates for new parts

### Phase 2: Library Structure

1. Implement the library directory structure
2. Set up symbol, footprint, and 3D model directories
3. Create templates for library components
4. Implement the component directory structure

### Phase 3: Advanced Features

1. Implement the scripts directory with utility scripts
2. Set up the config directory with workflow definitions
3. Implement the templates directory with advanced templates
4. Add support for custom directory structures

## Conclusion

The directory structure for Implexa provides a standardized, consistent, and maintainable organization for all files and directories in the system. It is designed to work well with Git, support multiple CAD tools, and integrate with the SQLite database. The structure is flexible enough to accommodate different types of parts and workflows while maintaining consistency across the project.