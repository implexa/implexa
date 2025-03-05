//! Database module for Implexa
//!
//! This module provides functionality for managing the SQLite database that stores
//! metadata for parts, relationships, and properties. It includes the database schema
//! implementation, connection management, and operations for interacting with the database.

pub mod schema;
pub mod connection_manager;
pub mod part;
pub mod revision;
pub mod relationship;
pub mod property;
pub mod manufacturer_part;
pub mod approval;
pub mod file;
pub mod workflow;
pub mod category;
pub mod part_management;

pub use schema::{DatabaseManager, DatabaseError, DatabaseResult};
pub use connection_manager::ConnectionManager;
pub use part::{Part, PartManager};
pub use revision::{Revision, RevisionStatus, RevisionManager};
pub use relationship::{Relationship, RelationshipType, RelationshipManager};
pub use property::{Property, PropertyType, PropertyManager};
pub use manufacturer_part::{ManufacturerPart, ManufacturerPartStatus, ManufacturerPartManager};
pub use approval::{Approval, ApprovalStatus, ApprovalManager};
pub use file::{File, FileType, FileManager};
pub use workflow::{Workflow, WorkflowState, WorkflowTransition, WorkflowManager};
pub use category::{Category, Subcategory, CategoryManager};
pub use part_management::{PartManagementManager, PartManagementError, PartManagementResult, User, UserRole};

/// Database module version
pub const VERSION: &str = "0.1.0";