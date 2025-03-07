//! Database schema module for Implexa
//!
//! This module provides functionality for managing the SQLite database schema,
//! including initialization, migrations, and version management.

use rusqlite::{Connection, Error as SqliteError, params};
use std::path::Path;
use thiserror::Error;
use crate::database::connection_manager::ConnectionManager;
use crate::git_backend::GitBackendError;

/// Errors that can occur during database operations
#[derive(Error, Debug)]
pub enum DatabaseError {
    /// SQLite error
    #[error("SQLite error: {0}")]
    Sqlite(#[from] SqliteError),

    /// Database initialization error
    #[error("Failed to initialize database: {0}")]
    InitializationError(String),

    /// Schema version error
    #[error("Schema version error: {0}")]
    SchemaVersionError(String),

    /// Migration error
    #[error("Migration error: {0}")]
    MigrationError(String),
    
    /// Git Backend error
    #[error("Git backend error: {0}")]
    GitBackend(#[from] GitBackendError),
}

/// Result type for database operations
pub type DatabaseResult<T> = std::result::Result<T, DatabaseError>;

/// Database manager for Implexa
///
/// This struct provides methods for initializing and interacting with the SQLite database.
pub struct DatabaseManager {
    /// Connection manager for the SQLite database
    connection_manager: ConnectionManager,
}

impl DatabaseManager {
    /// Create a new DatabaseManager with a connection to the specified database file
    ///
    /// # Arguments
    ///
    /// * `db_path` - Path to the SQLite database file
    ///
    /// # Returns
    ///
    /// A new DatabaseManager instance
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the connection cannot be established
    pub fn new<P: AsRef<Path>>(db_path: P) -> DatabaseResult<Self> {
        let connection_manager = ConnectionManager::new(db_path.as_ref())?;
        Ok(Self {
            connection_manager,
        })
    }

    /// Initialize the database schema
    ///
    /// This method creates all the necessary tables, indexes, and constraints
    /// for the Implexa database schema.
    ///
    /// # Returns
    ///
    /// Ok(()) if the schema was successfully initialized
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the schema initialization fails
    pub fn initialize_schema(&self) -> DatabaseResult<()> {
        // Use a transaction to ensure all schema changes are atomic
        self.connection_manager.transaction::<_, _, DatabaseError>(|tx| {

        // Create SchemaVersion table
        tx.execute(
            "CREATE TABLE IF NOT EXISTS SchemaVersion (
                version INTEGER PRIMARY KEY,
                applied_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                description TEXT
            )",
            [],
        )?;

        // Create Parts table with sequential number as primary key
        tx.execute(
            "CREATE TABLE IF NOT EXISTS Parts (
                part_id INTEGER PRIMARY KEY,
                category TEXT NOT NULL,
                subcategory TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                modified_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(category, subcategory, name)
            )",
            [],
        )?;

        // Create indexes for Parts table
        tx.execute("CREATE INDEX IF NOT EXISTS idx_parts_category ON Parts(category)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_parts_subcategory ON Parts(subcategory)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_parts_name ON Parts(name)", [])?;
        
        // Create a sequence table to track the next part_id
        tx.execute(
            "CREATE TABLE IF NOT EXISTS PartSequence (
                id INTEGER PRIMARY KEY CHECK (id = 1), -- Only one row allowed
                next_value INTEGER NOT NULL DEFAULT 10000
            )",
            [],
        )?;
        
        // Initialize the sequence with starting value 10000
        tx.execute(
            "INSERT OR IGNORE INTO PartSequence (id, next_value) VALUES (1, 10000)",
            [],
        )?;
        
        // Create Categories table for configurable categories
        tx.execute(
            "CREATE TABLE IF NOT EXISTS Categories (
                category_id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                code TEXT NOT NULL,
                description TEXT,
                UNIQUE(name),
                UNIQUE(code)
            )",
            [],
        )?;
        
        // Create Subcategories table for configurable subcategories
        tx.execute(
            "CREATE TABLE IF NOT EXISTS Subcategories (
                subcategory_id INTEGER PRIMARY KEY AUTOINCREMENT,
                category_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                code TEXT NOT NULL,
                description TEXT,
                FOREIGN KEY (category_id) REFERENCES Categories(category_id) ON DELETE CASCADE,
                UNIQUE(category_id, name),
                UNIQUE(category_id, code)
            )",
            [],
        )?;
        
        // Create indexes for Categories and Subcategories tables
        tx.execute("CREATE INDEX IF NOT EXISTS idx_categories_code ON Categories(code)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_subcategories_code ON Subcategories(code)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_subcategories_category_id ON Subcategories(category_id)", [])?;
        
        // Insert default categories
        let default_categories = [
            ("Electronic", "EL", "Electronic components and PCBAs"),
            ("Mechanical", "ME", "Mechanical parts and assemblies"),
            ("Assembly", "AS", "Product-level assemblies"),
            ("Software", "SW", "Software components"),
            ("Documentation", "DO", "Documentation")
        ];
        
        for (name, code, description) in default_categories.iter() {
            tx.execute(
                "INSERT OR IGNORE INTO Categories (name, code, description) VALUES (?1, ?2, ?3)",
                params![name, code, description],
            )?;
        }
        
        // Insert default subcategories for Electronic category
        let electronic_id: i64 = tx.query_row(
            "SELECT category_id FROM Categories WHERE code = 'EL'",
            [],
            |row| row.get(0),
        )?;
        
        let electronic_subcategories = [
            ("Schematic Symbol", "SYM", "Schematic symbols"),
            ("Footprint", "FPR", "PCB footprints"),
            ("3D Model", "3DM", "3D models"),
            ("Resistor", "RES", "Resistors"),
            ("Capacitor", "CAP", "Capacitors"),
            ("Inductor", "IND", "Inductors"),
            ("Integrated Circuit", "ICT", "Integrated circuits"),
            ("Diode", "DIO", "Diodes"),
            ("Transistor", "FET", "Transistors and FETs"),
            ("Connector", "CON", "Connectors"),
            ("PCB", "PCB", "Printed circuit boards"),
            ("PCBA", "PCA", "Printed circuit assemblies")
        ];
        
        for (name, code, description) in electronic_subcategories.iter() {
            tx.execute(
                "INSERT OR IGNORE INTO Subcategories (category_id, name, code, description) VALUES (?1, ?2, ?3, ?4)",
                params![electronic_id, name, code, description],
            )?;
        }

        // Create Revisions table
        tx.execute(
            "CREATE TABLE IF NOT EXISTS Revisions (
                revision_id INTEGER PRIMARY KEY AUTOINCREMENT,
                part_id INTEGER NOT NULL,
                version TEXT NOT NULL,
                status TEXT NOT NULL CHECK(status IN ('Draft', 'In Review', 'Released', 'Obsolete')),
                created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                created_by TEXT NOT NULL,
                commit_hash TEXT,
                FOREIGN KEY (part_id) REFERENCES Parts(part_id) ON DELETE CASCADE,
                UNIQUE(part_id, version)
            )",
            [],
        )?;

        // Create indexes for Revisions table
        tx.execute("CREATE INDEX IF NOT EXISTS idx_revisions_part_id ON Revisions(part_id)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_revisions_status ON Revisions(status)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_revisions_commit_hash ON Revisions(commit_hash)", [])?;

        // Create Relationships table
        tx.execute(
            "CREATE TABLE IF NOT EXISTS Relationships (
                relationship_id INTEGER PRIMARY KEY AUTOINCREMENT,
                parent_part_id INTEGER NOT NULL,
                child_part_id INTEGER NOT NULL,
                type TEXT NOT NULL,
                quantity INTEGER NOT NULL DEFAULT 1,
                FOREIGN KEY (parent_part_id) REFERENCES Parts(part_id) ON DELETE CASCADE,
                FOREIGN KEY (child_part_id) REFERENCES Parts(part_id) ON DELETE CASCADE,
                UNIQUE(parent_part_id, child_part_id, type)
            )",
            [],
        )?;

        // Create indexes for Relationships table
        tx.execute("CREATE INDEX IF NOT EXISTS idx_relationships_parent ON Relationships(parent_part_id)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_relationships_child ON Relationships(child_part_id)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_relationships_type ON Relationships(type)", [])?;

        // Create Properties table
        tx.execute(
            "CREATE TABLE IF NOT EXISTS Properties (
                property_id INTEGER PRIMARY KEY AUTOINCREMENT,
                part_id INTEGER,
                revision_id INTEGER,
                key TEXT NOT NULL,
                value TEXT,
                type TEXT NOT NULL DEFAULT 'string',
                FOREIGN KEY (part_id) REFERENCES Parts(part_id) ON DELETE CASCADE,
                FOREIGN KEY (revision_id) REFERENCES Revisions(revision_id) ON DELETE CASCADE,
                CHECK ((part_id IS NOT NULL AND revision_id IS NULL) OR (part_id IS NULL AND revision_id IS NOT NULL)),
                UNIQUE(part_id, revision_id, key)
            )",
            [],
        )?;

        // Create indexes for Properties table
        tx.execute("CREATE INDEX IF NOT EXISTS idx_properties_part_id ON Properties(part_id)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_properties_revision_id ON Properties(revision_id)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_properties_key ON Properties(key)", [])?;

        // Create ManufacturerParts table
        tx.execute(
            "CREATE TABLE IF NOT EXISTS ManufacturerParts (
                mpn_id INTEGER PRIMARY KEY AUTOINCREMENT,
                part_id INTEGER NOT NULL,
                manufacturer TEXT NOT NULL,
                mpn TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL DEFAULT 'Active' CHECK(status IN ('Active', 'Preferred', 'Alternate', 'Obsolete')),
                FOREIGN KEY (part_id) REFERENCES Parts(part_id) ON DELETE CASCADE,
                UNIQUE(manufacturer, mpn)
            )",
            [],
        )?;

        // Create indexes for ManufacturerParts table
        tx.execute("CREATE INDEX IF NOT EXISTS idx_mpn_part_id ON ManufacturerParts(part_id)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_mpn_manufacturer ON ManufacturerParts(manufacturer)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_mpn_mpn ON ManufacturerParts(mpn)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_mpn_status ON ManufacturerParts(status)", [])?;

        // Create Approvals table
        tx.execute(
            "CREATE TABLE IF NOT EXISTS Approvals (
                approval_id INTEGER PRIMARY KEY AUTOINCREMENT,
                revision_id INTEGER NOT NULL,
                approver TEXT NOT NULL,
                status TEXT NOT NULL CHECK(status IN ('Pending', 'Approved', 'Rejected')),
                date TIMESTAMP,
                comments TEXT,
                FOREIGN KEY (revision_id) REFERENCES Revisions(revision_id) ON DELETE CASCADE,
                UNIQUE(revision_id, approver)
            )",
            [],
        )?;

        // Create indexes for Approvals table
        tx.execute("CREATE INDEX IF NOT EXISTS idx_approvals_revision_id ON Approvals(revision_id)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_approvals_approver ON Approvals(approver)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_approvals_status ON Approvals(status)", [])?;

        // Create Files table
        tx.execute(
            "CREATE TABLE IF NOT EXISTS Files (
                file_id INTEGER PRIMARY KEY AUTOINCREMENT,
                part_id INTEGER,
                revision_id INTEGER,
                path TEXT NOT NULL,
                type TEXT NOT NULL,
                description TEXT,
                FOREIGN KEY (part_id) REFERENCES Parts(part_id) ON DELETE CASCADE,
                FOREIGN KEY (revision_id) REFERENCES Revisions(revision_id) ON DELETE CASCADE,
                CHECK ((part_id IS NOT NULL AND revision_id IS NULL) OR (part_id IS NULL AND revision_id IS NOT NULL))
            )",
            [],
        )?;

        // Create indexes for Files table
        tx.execute("CREATE INDEX IF NOT EXISTS idx_files_part_id ON Files(part_id)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_files_revision_id ON Files(revision_id)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_files_type ON Files(type)", [])?;

        // Create Workflows table
        tx.execute(
            "CREATE TABLE IF NOT EXISTS Workflows (
                workflow_id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                description TEXT,
                active BOOLEAN NOT NULL DEFAULT 1,
                UNIQUE(name)
            )",
            [],
        )?;

        // Create WorkflowStates table
        tx.execute(
            "CREATE TABLE IF NOT EXISTS WorkflowStates (
                state_id INTEGER PRIMARY KEY AUTOINCREMENT,
                workflow_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                is_initial BOOLEAN NOT NULL DEFAULT 0,
                is_terminal BOOLEAN NOT NULL DEFAULT 0,
                FOREIGN KEY (workflow_id) REFERENCES Workflows(workflow_id) ON DELETE CASCADE,
                UNIQUE(workflow_id, name)
            )",
            [],
        )?;

        // Create index for WorkflowStates table
        tx.execute("CREATE INDEX IF NOT EXISTS idx_workflow_states_workflow_id ON WorkflowStates(workflow_id)", [])?;

        // Create WorkflowTransitions table
        tx.execute(
            "CREATE TABLE IF NOT EXISTS WorkflowTransitions (
                transition_id INTEGER PRIMARY KEY AUTOINCREMENT,
                workflow_id INTEGER NOT NULL,
                from_state_id INTEGER NOT NULL,
                to_state_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                requires_approval BOOLEAN NOT NULL DEFAULT 0,
                FOREIGN KEY (workflow_id) REFERENCES Workflows(workflow_id) ON DELETE CASCADE,
                FOREIGN KEY (from_state_id) REFERENCES WorkflowStates(state_id) ON DELETE CASCADE,
                FOREIGN KEY (to_state_id) REFERENCES WorkflowStates(state_id) ON DELETE CASCADE,
                UNIQUE(workflow_id, from_state_id, to_state_id)
            )",
            [],
        )?;

        // Create indexes for WorkflowTransitions table
        tx.execute("CREATE INDEX IF NOT EXISTS idx_workflow_transitions_workflow_id ON WorkflowTransitions(workflow_id)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_workflow_transitions_from_state_id ON WorkflowTransitions(from_state_id)", [])?;
        tx.execute("CREATE INDEX IF NOT EXISTS idx_workflow_transitions_to_state_id ON WorkflowTransitions(to_state_id)", [])?;

        // Insert initial schema version (use OR IGNORE to handle multiple initializations)
        tx.execute(
            "INSERT OR IGNORE INTO SchemaVersion (version, description) VALUES (1, 'Initial schema creation')",
            [],
        )?;
            
            Ok(())
        })
    }

    /// Get the current schema version
    ///
    /// # Returns
    ///
    /// The current schema version
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the version cannot be retrieved
    pub fn get_schema_version(&self) -> DatabaseResult<i64> {
        self.connection_manager.execute::<_, _, DatabaseError>(|conn| {
            let version = conn.query_row(
                "SELECT MAX(version) FROM SchemaVersion",
                [],
                |row| row.get(0),
            )?;
            Ok(version)
        })
    }

    /// Get a reference to the connection manager
    ///
    /// # Returns
    ///
    /// A reference to the connection manager
    pub fn connection_manager(&self) -> &ConnectionManager {
        &self.connection_manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_database_initialization() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager
        let db_manager = DatabaseManager::new(&db_path).unwrap();

        // Initialize the schema
        let result = db_manager.initialize_schema();
        assert!(result.is_ok());

        // Check that the schema version is 1
        let version = db_manager.get_schema_version().unwrap();
        assert_eq!(version, 1);

        // Check that all tables were created
        let tables = db_manager
            .connection_manager()
            .execute::<_, _, DatabaseError>(|conn| {
                let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table'")?;
                let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
                let mut tables = Vec::new();
                for row_result in rows {
                    tables.push(row_result?);
                }
                Ok(tables)
            })
            .unwrap();

        // Check that all expected tables exist
        let expected_tables = vec![
            "SchemaVersion",
            "Parts",
            "Revisions",
            "Relationships",
            "Properties",
            "ManufacturerParts",
            "Approvals",
            "Files",
            "Workflows",
            "WorkflowStates",
            "WorkflowTransitions",
        ];

        for table in expected_tables {
            assert!(tables.contains(&table.to_string()));
        }
    }
}