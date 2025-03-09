//! Directory Template Manager
//!
//! This module handles the creation and management of directory templates for parts and libraries.
//! It provides functionality for creating directories based on templates and managing template files.

use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use crate::git_backend::{GitBackendConfig, GitBackendError, Result};

/// Directory template schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryTemplate {
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Template version
    pub version: String,
    /// Directories to create
    pub directories: Vec<DirectoryDefinition>,
    /// Files to create
    pub files: Vec<FileDefinition>,
}

/// Directory definition in a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryDefinition {
    /// Directory path relative to the part directory
    pub path: String,
    /// Directory description
    pub description: String,
    /// Whether the directory is required
    pub required: bool,
    /// Subdirectories
    #[serde(default)]
    pub subdirectories: Vec<DirectoryDefinition>,
}

/// File definition in a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDefinition {
    /// File path relative to the part directory
    pub path: String,
    /// File description
    pub description: String,
    /// Whether the file is required
    pub required: bool,
    /// Template file to use as a source
    pub template: Option<String>,
}

/// Template type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateType {
    /// Minimal template with only essential directories
    Minimal,
    /// Standard template with commonly used directories (default)
    Standard,
    /// Extended template with all possible directories
    Extended,
    /// Custom template
    Custom(String),
}

impl std::fmt::Display for TemplateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemplateType::Minimal => write!(f, "minimal"),
            TemplateType::Standard => write!(f, "standard"),
            TemplateType::Extended => write!(f, "extended"),
            TemplateType::Custom(name) => write!(f, "custom/{}", name),
        }
    }
}

/// Directory Template Manager
pub struct DirectoryTemplateManager<'a> {
    /// The Git Backend configuration
    #[allow(dead_code)]
    config: &'a GitBackendConfig,
    /// Repository path
    repo_path: PathBuf,
}

impl<'a> DirectoryTemplateManager<'a> {
    /// Creates a new DirectoryTemplateManager
    pub fn new(repo_path: &Path, config: &'a GitBackendConfig) -> Self {
        Self {
            config,
            repo_path: repo_path.to_path_buf(),
        }
    }

    /// Gets the path to the template directory
    fn template_dir(&self) -> PathBuf {
        self.repo_path.join("config").join("directory-templates")
    }

    /// Gets the path to a template file
    fn template_file_path(&self, template_type: TemplateType) -> PathBuf {
        match template_type {
            TemplateType::Minimal => self.template_dir().join("minimal.json"),
            TemplateType::Standard => self.template_dir().join("standard.json"),
            TemplateType::Extended => self.template_dir().join("extended.json"),
            TemplateType::Custom(name) => self.template_dir().join("custom").join(format!("{}.json", name)),
        }
    }

    /// Loads a template from a file
    pub fn load_template(&self, template_type: TemplateType) -> Result<DirectoryTemplate> {
        let path = self.template_file_path(template_type);
        
        if !path.exists() {
            return Err(GitBackendError::RepositoryError(
                format!("Template file not found: {}", path.display())
            ));
        }
        
        let content = fs::read_to_string(&path)
            .map_err(|e| GitBackendError::IoError(e))?;
        
        serde_json::from_str(&content)
            .map_err(|e| GitBackendError::RepositoryError(
                format!("Failed to parse template file: {}", e)
            ))
    }

    /// Creates the default templates if they don't exist
    pub fn create_default_templates(&self) -> Result<()> {
        // Create the template directory if it doesn't exist
        let template_dir = self.template_dir();
        if !template_dir.exists() {
            fs::create_dir_all(&template_dir)
                .map_err(|e| GitBackendError::IoError(e))?;
        }

        // Create the custom template directory if it doesn't exist
        let custom_dir = template_dir.join("custom");
        if !custom_dir.exists() {
            fs::create_dir_all(&custom_dir)
                .map_err(|e| GitBackendError::IoError(e))?;
        }

        // Create the minimal template if it doesn't exist
        let minimal_path = self.template_file_path(TemplateType::Minimal);
        if !minimal_path.exists() {
            let minimal_template = self.create_minimal_template();
            let content = serde_json::to_string_pretty(&minimal_template)
                .map_err(|e| GitBackendError::RepositoryError(
                    format!("Failed to serialize minimal template: {}", e)
                ))?;
            fs::write(&minimal_path, content)
                .map_err(|e| GitBackendError::IoError(e))?;
        }

        // Create the standard template if it doesn't exist
        let standard_path = self.template_file_path(TemplateType::Standard);
        if !standard_path.exists() {
            let standard_template = self.create_standard_template();
            let content = serde_json::to_string_pretty(&standard_template)
                .map_err(|e| GitBackendError::RepositoryError(
                    format!("Failed to serialize standard template: {}", e)
                ))?;
            fs::write(&standard_path, content)
                .map_err(|e| GitBackendError::IoError(e))?;
        }

        // Create the extended template if it doesn't exist
        let extended_path = self.template_file_path(TemplateType::Extended);
        if !extended_path.exists() {
            let extended_template = self.create_extended_template();
            let content = serde_json::to_string_pretty(&extended_template)
                .map_err(|e| GitBackendError::RepositoryError(
                    format!("Failed to serialize extended template: {}", e)
                ))?;
            fs::write(&extended_path, content)
                .map_err(|e| GitBackendError::IoError(e))?;
        }

        Ok(())
    }

    /// Creates a minimal template
    fn create_minimal_template(&self) -> DirectoryTemplate {
        DirectoryTemplate {
            name: "Minimal Template".to_string(),
            description: "Minimal directory structure for parts with only essential directories".to_string(),
            version: "1.0.0".to_string(),
            directories: vec![
                DirectoryDefinition {
                    path: "design".to_string(),
                    description: "Design files".to_string(),
                    required: true,
                    subdirectories: vec![],
                },
            ],
            files: vec![
                FileDefinition {
                    path: "README.md".to_string(),
                    description: "Human-readable part description".to_string(),
                    required: true,
                    template: Some("templates/readme/minimal.md".to_string()),
                },
                FileDefinition {
                    path: "metadata.db".to_string(),
                    description: "SQLite database with part metadata".to_string(),
                    required: true,
                    template: Some("templates/database/minimal.db".to_string()),
                },
            ],
        }
    }

    /// Creates a standard template
    fn create_standard_template(&self) -> DirectoryTemplate {
        DirectoryTemplate {
            name: "Standard Template".to_string(),
            description: "Standard directory structure for parts with commonly used directories".to_string(),
            version: "1.0.0".to_string(),
            directories: vec![
                DirectoryDefinition {
                    path: "design".to_string(),
                    description: "Design files".to_string(),
                    required: true,
                    subdirectories: vec![
                        DirectoryDefinition {
                            path: "models".to_string(),
                            description: "3D models".to_string(),
                            required: false,
                            subdirectories: vec![],
                        },
                        DirectoryDefinition {
                            path: "schematics".to_string(),
                            description: "Schematics and diagrams".to_string(),
                            required: false,
                            subdirectories: vec![],
                        },
                    ],
                },
                DirectoryDefinition {
                    path: "manufacturing".to_string(),
                    description: "Manufacturing output files".to_string(),
                    required: true,
                    subdirectories: vec![
                        DirectoryDefinition {
                            path: "bom".to_string(),
                            description: "Bill of Materials".to_string(),
                            required: true,
                            subdirectories: vec![],
                        },
                        DirectoryDefinition {
                            path: "assembly".to_string(),
                            description: "Assembly instructions".to_string(),
                            required: false,
                            subdirectories: vec![],
                        },
                    ],
                },
                DirectoryDefinition {
                    path: "documentation".to_string(),
                    description: "Documentation files".to_string(),
                    required: true,
                    subdirectories: vec![
                        DirectoryDefinition {
                            path: "datasheets".to_string(),
                            description: "Component datasheets".to_string(),
                            required: false,
                            subdirectories: vec![],
                        },
                        DirectoryDefinition {
                            path: "specifications".to_string(),
                            description: "Specifications".to_string(),
                            required: true,
                            subdirectories: vec![],
                        },
                    ],
                },
            ],
            files: vec![
                FileDefinition {
                    path: "README.md".to_string(),
                    description: "Human-readable part description".to_string(),
                    required: true,
                    template: Some("templates/readme/standard.md".to_string()),
                },
                FileDefinition {
                    path: "metadata.db".to_string(),
                    description: "SQLite database with part metadata".to_string(),
                    required: true,
                    template: Some("templates/database/standard.db".to_string()),
                },
            ],
        }
    }

    /// Creates an extended template
    fn create_extended_template(&self) -> DirectoryTemplate {
        DirectoryTemplate {
            name: "Extended Template".to_string(),
            description: "Extended directory structure for parts with all possible directories".to_string(),
            version: "1.0.0".to_string(),
            directories: vec![
                DirectoryDefinition {
                    path: "design".to_string(),
                    description: "Design files".to_string(),
                    required: true,
                    subdirectories: vec![
                        DirectoryDefinition {
                            path: "models".to_string(),
                            description: "3D models".to_string(),
                            required: true,
                            subdirectories: vec![
                                DirectoryDefinition {
                                    path: "step".to_string(),
                                    description: "STEP format models".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "stl".to_string(),
                                    description: "STL format models".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                            ],
                        },
                        DirectoryDefinition {
                            path: "schematics".to_string(),
                            description: "Schematics and diagrams".to_string(),
                            required: true,
                            subdirectories: vec![
                                DirectoryDefinition {
                                    path: "pdf".to_string(),
                                    description: "PDF exports".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "svg".to_string(),
                                    description: "SVG exports".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                            ],
                        },
                        DirectoryDefinition {
                            path: "source".to_string(),
                            description: "Source files for design".to_string(),
                            required: true,
                            subdirectories: vec![
                                DirectoryDefinition {
                                    path: "kicad".to_string(),
                                    description: "KiCad project files".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "freecad".to_string(),
                                    description: "FreeCAD project files".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                            ],
                        },
                    ],
                },
                DirectoryDefinition {
                    path: "manufacturing".to_string(),
                    description: "Manufacturing output files".to_string(),
                    required: true,
                    subdirectories: vec![
                        DirectoryDefinition {
                            path: "gerbers".to_string(),
                            description: "Gerber files for PCBs".to_string(),
                            required: false,
                            subdirectories: vec![],
                        },
                        DirectoryDefinition {
                            path: "bom".to_string(),
                            description: "Bill of Materials".to_string(),
                            required: true,
                            subdirectories: vec![],
                        },
                        DirectoryDefinition {
                            path: "assembly".to_string(),
                            description: "Assembly instructions".to_string(),
                            required: true,
                            subdirectories: vec![
                                DirectoryDefinition {
                                    path: "images".to_string(),
                                    description: "Images for assembly instructions".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                            ],
                        },
                        DirectoryDefinition {
                            path: "drawings".to_string(),
                            description: "Technical drawings".to_string(),
                            required: true,
                            subdirectories: vec![
                                DirectoryDefinition {
                                    path: "pdf".to_string(),
                                    description: "PDF format drawings".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "dxf".to_string(),
                                    description: "DXF format drawings".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                            ],
                        },
                    ],
                },
                DirectoryDefinition {
                    path: "documentation".to_string(),
                    description: "Documentation files".to_string(),
                    required: true,
                    subdirectories: vec![
                        DirectoryDefinition {
                            path: "datasheets".to_string(),
                            description: "Component datasheets".to_string(),
                            required: true,
                            subdirectories: vec![],
                        },
                        DirectoryDefinition {
                            path: "specifications".to_string(),
                            description: "Specifications".to_string(),
                            required: true,
                            subdirectories: vec![
                                DirectoryDefinition {
                                    path: "electrical".to_string(),
                                    description: "Electrical specifications".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "mechanical".to_string(),
                                    description: "Mechanical specifications".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "environmental".to_string(),
                                    description: "Environmental specifications".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                            ],
                        },
                        DirectoryDefinition {
                            path: "user-guides".to_string(),
                            description: "User guides".to_string(),
                            required: true,
                            subdirectories: vec![
                                DirectoryDefinition {
                                    path: "installation".to_string(),
                                    description: "Installation guides".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "operation".to_string(),
                                    description: "Operation guides".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "maintenance".to_string(),
                                    description: "Maintenance guides".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                            ],
                        },
                    ],
                },
                DirectoryDefinition {
                    path: "tests".to_string(),
                    description: "Test files and results".to_string(),
                    required: true,
                    subdirectories: vec![
                        DirectoryDefinition {
                            path: "test-plans".to_string(),
                            description: "Test plans".to_string(),
                            required: true,
                            subdirectories: vec![
                                DirectoryDefinition {
                                    path: "functional".to_string(),
                                    description: "Functional test plans".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "performance".to_string(),
                                    description: "Performance test plans".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "compliance".to_string(),
                                    description: "Compliance test plans".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                            ],
                        },
                        DirectoryDefinition {
                            path: "test-results".to_string(),
                            description: "Test results".to_string(),
                            required: true,
                            subdirectories: vec![
                                DirectoryDefinition {
                                    path: "functional".to_string(),
                                    description: "Functional test results".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "performance".to_string(),
                                    description: "Performance test results".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "compliance".to_string(),
                                    description: "Compliance test results".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                            ],
                        },
                        DirectoryDefinition {
                            path: "test-fixtures".to_string(),
                            description: "Test fixtures".to_string(),
                            required: true,
                            subdirectories: vec![
                                DirectoryDefinition {
                                    path: "hardware".to_string(),
                                    description: "Hardware test fixtures".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                                DirectoryDefinition {
                                    path: "software".to_string(),
                                    description: "Software test fixtures".to_string(),
                                    required: false,
                                    subdirectories: vec![],
                                },
                            ],
                        },
                    ],
                },
            ],
            files: vec![
                FileDefinition {
                    path: "README.md".to_string(),
                    description: "Human-readable part description".to_string(),
                    required: true,
                    template: Some("templates/readme/extended.md".to_string()),
                },
                FileDefinition {
                    path: "metadata.db".to_string(),
                    description: "SQLite database with part metadata".to_string(),
                    required: true,
                    template: Some("templates/database/extended.db".to_string()),
                },
            ],
        }
    }

    /// Validates if a category code is valid
    #[allow(dead_code)]
    fn is_valid_category(&self, category: &str) -> bool {
        // Simple validation: non-empty string with 2-3 uppercase letters
        !category.is_empty() && category.len() <= 3 &&
        category.chars().all(|c| c.is_ascii_uppercase())
    }

    /// Validates if a subcategory code is valid
    #[allow(dead_code)]
    fn is_valid_subcategory(&self, subcategory: &str) -> bool {
        // Simple validation: non-empty string with 2-4 uppercase letters
        !subcategory.is_empty() && subcategory.len() <= 4 &&
        subcategory.chars().all(|c| c.is_ascii_uppercase())
    }

    /// Lists available templates
    pub fn list_templates(&self) -> Result<Vec<String>> {
        let mut templates = vec![
            "minimal".to_string(),
            "standard".to_string(),
            "extended".to_string(),
        ];
        
        // Add custom templates
        let custom_dir = self.template_dir().join("custom");
        if custom_dir.exists() {
            let entries = fs::read_dir(&custom_dir)
                .map_err(|e| GitBackendError::IoError(e))?;
            
            for entry in entries {
                let entry = entry.map_err(|e| GitBackendError::IoError(e))?;
                let path = entry.path();
                
                if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
                    if let Some(stem) = path.file_stem() {
                        if let Some(name) = stem.to_str() {
                            templates.push(format!("custom/{}", name));
                        }
                    }
                }
            }
        }
        
        Ok(templates)
    }

    /// Creates a part directory with the specified template
    pub fn create_part_directory(
        &self,
        part_number: &str,
        category: &str,
        subcategory: &str,
        template_type: TemplateType,
    ) -> Result<PathBuf> {
        // Load the template
        let template = self.load_template(template_type)?;
        
        // Create the part directory
        let part_dir = self.repo_path
            .join("parts")
            .join(format!("{}-{}-{}", category, subcategory, part_number));
        
        if !part_dir.exists() {
            fs::create_dir_all(&part_dir)
                .map_err(|e| GitBackendError::IoError(e))?;
        }
        
        // Create directories from the template
        for dir in &template.directories {
            self.create_directory_from_template(&part_dir, dir)?;
        }
        
        // Create files from the template
        for file in &template.files {
            self.create_file_from_template(&part_dir, file)?;
        }
        
        Ok(part_dir)
    }

    /// Creates a directory from a template definition
    fn create_directory_from_template(&self, base_dir: &Path, dir: &DirectoryDefinition) -> Result<()> {
        let dir_path = base_dir.join(&dir.path);
        
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path)
                .map_err(|e| GitBackendError::IoError(e))?;
        }
        
        // Create subdirectories
        for subdir in &dir.subdirectories {
            self.create_directory_from_template(&dir_path, subdir)?;
        }
        
        Ok(())
    }

    /// Creates a file from a template definition
    fn create_file_from_template(&self, base_dir: &Path, file: &FileDefinition) -> Result<()> {
        let file_path = base_dir.join(&file.path);
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| GitBackendError::IoError(e))?;
            }
        }
        
        // If the file already exists, skip it
        if file_path.exists() {
            return Ok(());
        }
        
        // If there's a template file, copy it
        if let Some(template_path) = &file.template {
            let template_file = self.repo_path.join(template_path);
            
            if template_file.exists() {
                fs::copy(&template_file, &file_path)
                    .map_err(|e| GitBackendError::IoError(e))?;
            } else {
                // If the template file doesn't exist, create an empty file
                fs::write(&file_path, "")
                    .map_err(|e| GitBackendError::IoError(e))?;
            }
        } else {
            // If there's no template file, create an empty file
            fs::write(&file_path, "")
                .map_err(|e| GitBackendError::IoError(e))?;
        }
        
        Ok(())
    }

    /// Creates a custom template
    pub fn create_custom_template(
        &self,
        name: &str,
        description: &str,
        directories: Vec<DirectoryDefinition>,
        files: Vec<FileDefinition>,
    ) -> Result<()> {
        let template = DirectoryTemplate {
            name: name.to_string(),
            description: description.to_string(),
            version: "1.0.0".to_string(),
            directories,
            files,
        };
        
        let custom_dir = self.template_dir().join("custom");
        if !custom_dir.exists() {
            fs::create_dir_all(&custom_dir)
                .map_err(|e| GitBackendError::IoError(e))?;
        }
        
        let template_path = custom_dir.join(format!("{}.json", name));
        let content = serde_json::to_string_pretty(&template)
            .map_err(|e| GitBackendError::RepositoryError(
                format!("Failed to serialize custom template: {}", e)
            ))?;
        
        fs::write(&template_path, content)
            .map_err(|e| GitBackendError::IoError(e))?;
        
        Ok(())
    }

    /// Creates template files for README.md
    pub fn create_readme_templates(&self) -> Result<()> {
        let templates_dir = self.repo_path.join("templates").join("readme");
        
        if !templates_dir.exists() {
            fs::create_dir_all(&templates_dir)
                .map_err(|e| GitBackendError::IoError(e))?;
        }
        
        // Create minimal README template
        let minimal_path = templates_dir.join("minimal.md");
        if !minimal_path.exists() {
            let content = r#"# [Part Number]

## Description

Brief description of the part.

## Specifications

- Specification 1
- Specification 2

## Usage

How to use this part.
"#;
            fs::write(&minimal_path, content)
                .map_err(|e| GitBackendError::IoError(e))?;
        }
        
        // Create standard README template
        let standard_path = templates_dir.join("standard.md");
        if !standard_path.exists() {
            let content = r#"# [Part Number]

## Description

Detailed description of the part.

## Specifications

### Electrical
- Voltage: 
- Current: 
- Power: 

### Mechanical
- Dimensions: 
- Weight: 
- Material: 

## Usage

Detailed instructions on how to use this part.

## Manufacturing Notes

Notes for manufacturing this part.

## Related Parts

- Related part 1
- Related part 2
"#;
            fs::write(&standard_path, content)
                .map_err(|e| GitBackendError::IoError(e))?;
        }
        
        // Create extended README template
        let extended_path = templates_dir.join("extended.md");
        if !extended_path.exists() {
            let content = r#"# [Part Number]

## Description

Comprehensive description of the part, including its purpose, features, and benefits.

## Specifications

### Electrical
- Voltage: 
- Current: 
- Power: 
- Resistance: 
- Capacitance: 
- Inductance: 

### Mechanical
- Dimensions: 
- Weight: 
- Material: 
- Finish: 
- Mounting: 

### Environmental
- Operating Temperature: 
- Storage Temperature: 
- Humidity: 
- IP Rating: 

## Usage

Detailed instructions on how to use this part, including:
- Installation
- Configuration
- Operation
- Maintenance

## Manufacturing Notes

Comprehensive notes for manufacturing this part, including:
- Process requirements
- Special considerations
- Quality control measures

## Testing

Testing procedures and requirements for this part.

## Compliance

Compliance information for this part, including:
- RoHS
- REACH
- UL
- CE

## Related Parts

- Related part 1
- Related part 2
- Related part 3

## Revision History

| Revision | Date | Description |
|----------|------|-------------|
| 1.0.0    |      | Initial release |
"#;
            fs::write(&extended_path, content)
                .map_err(|e| GitBackendError::IoError(e))?;
        }
        
        Ok(())
    }
}