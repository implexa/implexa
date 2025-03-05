//! File module for Implexa
//!
//! This module provides functionality for managing files associated with parts and revisions in the database.

use rusqlite::{Connection, Transaction, params, Row, Result as SqliteResult};
use std::path::PathBuf;
use crate::database::schema::{DatabaseError, DatabaseResult};
use crate::database::connection_manager::ConnectionManager;

/// Type of file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    /// Design file (e.g., CAD file)
    Design,
    /// Documentation file (e.g., datasheet, manual)
    Documentation,
    /// Manufacturing file (e.g., Gerber, BOM)
    Manufacturing,
    /// Test file (e.g., test procedure, test results)
    Test,
    /// Image file (e.g., photo, render)
    Image,
    /// 3D model file
    Model3D,
    /// Source code file
    SourceCode,
    /// Other file type
    Other(String),
}

impl FileType {
    /// Convert a string to a FileType
    ///
    /// # Arguments
    ///
    /// * `type_str` - The file type string
    ///
    /// # Returns
    ///
    /// The corresponding FileType
    pub fn from_str(type_str: &str) -> Self {
        match type_str {
            "Design" => Self::Design,
            "Documentation" => Self::Documentation,
            "Manufacturing" => Self::Manufacturing,
            "Test" => Self::Test,
            "Image" => Self::Image,
            "Model3D" => Self::Model3D,
            "SourceCode" => Self::SourceCode,
            _ => Self::Other(type_str.to_string()),
        }
    }

    /// Convert a FileType to a string
    ///
    /// # Returns
    ///
    /// The string representation of the file type
    pub fn to_str(&self) -> String {
        match self {
            Self::Design => "Design".to_string(),
            Self::Documentation => "Documentation".to_string(),
            Self::Manufacturing => "Manufacturing".to_string(),
            Self::Test => "Test".to_string(),
            Self::Image => "Image".to_string(),
            Self::Model3D => "Model3D".to_string(),
            Self::SourceCode => "SourceCode".to_string(),
            Self::Other(s) => s.clone(),
        }
    }
}

/// Represents a file associated with a part or revision
#[derive(Debug, Clone)]
pub struct File {
    /// Unique identifier for the file
    pub file_id: Option<i64>,
    /// ID of the part this file is associated with (if applicable)
    pub part_id: Option<String>,
    /// ID of the revision this file is associated with (if applicable)
    pub revision_id: Option<i64>,
    /// Path to the file
    pub path: PathBuf,
    /// Type of the file
    pub file_type: FileType,
    /// Description of the file
    pub description: Option<String>,
}

impl File {
    /// Create a new file for a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - ID of the part this file is associated with
    /// * `path` - Path to the file
    /// * `file_type` - Type of the file
    /// * `description` - Description of the file
    ///
    /// # Returns
    ///
    /// A new File instance
    pub fn new_part_file(
        part_id: String,
        path: PathBuf,
        file_type: FileType,
        description: Option<String>,
    ) -> Self {
        Self {
            file_id: None,
            part_id: Some(part_id),
            revision_id: None,
            path,
            file_type,
            description,
        }
    }

    /// Create a new file for a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - ID of the revision this file is associated with
    /// * `path` - Path to the file
    /// * `file_type` - Type of the file
    /// * `description` - Description of the file
    ///
    /// # Returns
    ///
    /// A new File instance
    pub fn new_revision_file(
        revision_id: i64,
        path: PathBuf,
        file_type: FileType,
        description: Option<String>,
    ) -> Self {
        Self {
            file_id: None,
            part_id: None,
            revision_id: Some(revision_id),
            path,
            file_type,
            description,
        }
    }
}

/// Manager for file operations
pub struct FileManager<'a> {
    /// Connection manager for the SQLite database
    connection_manager: &'a ConnectionManager,
}

impl<'a> FileManager<'a> {
    /// Create a new FileManager
    ///
    /// # Arguments
    ///
    /// * `connection_manager` - Connection manager for the SQLite database
    ///
    /// # Returns
    ///
    /// A new FileManager instance
    pub fn new(connection_manager: &'a ConnectionManager) -> Self {
        Self { connection_manager }
    }
    
    /// Create a new FileManager with a transaction
    ///
    /// # Arguments
    ///
    /// * `transaction` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// A new FileManager instance
    ///
    /// # Note
    ///
    /// This is a temporary method for backward compatibility during migration
    pub fn new_with_transaction(_transaction: &'a Transaction) -> Self {
        unimplemented!("This method is a placeholder for backward compatibility during migration")
    }

    /// Create a new file in the database
    ///
    /// # Arguments
    ///
    /// * `file` - The file to create
    ///
    /// # Returns
    ///
    /// The ID of the newly created file
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the file could not be created
    pub fn create_file(&self, file: &File) -> DatabaseResult<i64> {
        self.connection_manager.execute_mut(|conn| {
            conn.execute(
                "INSERT INTO Files (part_id, revision_id, path, type, description)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    file.part_id,
                    file.revision_id,
                    file.path.to_string_lossy().to_string(),
                    file.file_type.to_str(),
                    file.description,
                ],
            )?;
            Ok(conn.last_insert_rowid())
        }).map_err(DatabaseError::from)
    }
    
    /// Create a new file in the database within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `file` - The file to create
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// The ID of the newly created file
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the file could not be created
    pub fn create_file_in_transaction(&self, file: &File, tx: &Transaction) -> DatabaseResult<i64> {
        tx.execute(
            "INSERT INTO Files (part_id, revision_id, path, type, description)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                file.part_id,
                file.revision_id,
                file.path.to_string_lossy().to_string(),
                file.file_type.to_str(),
                file.description,
            ],
        )?;
        Ok(tx.last_insert_rowid())
    }

    /// Get a file by its ID
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to get
    ///
    /// # Returns
    ///
    /// The file with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the file could not be found
    pub fn get_file(&self, file_id: i64) -> DatabaseResult<File> {
        self.connection_manager.execute(|conn| {
            let file = conn.query_row(
                "SELECT file_id, part_id, revision_id, path, type, description
                 FROM Files
                 WHERE file_id = ?1",
                params![file_id],
                |row| self.row_to_file(row),
            )?;
            Ok(file)
        }).map_err(DatabaseError::from)
    }

    /// Get all files for a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part
    ///
    /// # Returns
    ///
    /// A vector of files for the specified part
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the files could not be retrieved
    pub fn get_part_files(&self, part_id: &str) -> DatabaseResult<Vec<File>> {
        self.connection_manager.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT file_id, part_id, revision_id, path, type, description
                 FROM Files
                 WHERE part_id = ?1
                 ORDER BY type, path",
            )?;
            let files_iter = stmt.query_map(params![part_id], |row| self.row_to_file(row))?;
            let mut files = Vec::new();
            for file_result in files_iter {
                files.push(file_result?);
            }
            Ok(files)
        }).map_err(DatabaseError::from)
    }

    /// Get all files for a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision
    ///
    /// # Returns
    ///
    /// A vector of files for the specified revision
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the files could not be retrieved
    pub fn get_revision_files(&self, revision_id: i64) -> DatabaseResult<Vec<File>> {
        self.connection_manager.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT file_id, part_id, revision_id, path, type, description
                 FROM Files
                 WHERE revision_id = ?1
                 ORDER BY type, path",
            )?;
            let files_iter = stmt.query_map(params![revision_id], |row| self.row_to_file(row))?;
            let mut files = Vec::new();
            for file_result in files_iter {
                files.push(file_result?);
            }
            Ok(files)
        }).map_err(DatabaseError::from)
    }

    /// Get files by type for a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part
    /// * `file_type` - The type of files to get
    ///
    /// # Returns
    ///
    /// A vector of files of the specified type for the specified part
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the files could not be retrieved
    pub fn get_part_files_by_type(&self, part_id: &str, file_type: &FileType) -> DatabaseResult<Vec<File>> {
        self.connection_manager.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT file_id, part_id, revision_id, path, type, description
                 FROM Files
                 WHERE part_id = ?1 AND type = ?2
                 ORDER BY path",
            )?;
            let files_iter = stmt.query_map(params![part_id, file_type.to_str()], |row| self.row_to_file(row))?;
            let mut files = Vec::new();
            for file_result in files_iter {
                files.push(file_result?);
            }
            Ok(files)
        }).map_err(DatabaseError::from)
    }

    /// Get files by type for a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision
    /// * `file_type` - The type of files to get
    ///
    /// # Returns
    ///
    /// A vector of files of the specified type for the specified revision
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the files could not be retrieved
    pub fn get_revision_files_by_type(&self, revision_id: i64, file_type: &FileType) -> DatabaseResult<Vec<File>> {
        self.connection_manager.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT file_id, part_id, revision_id, path, type, description
                 FROM Files
                 WHERE revision_id = ?1 AND type = ?2
                 ORDER BY path",
            )?;
            let files_iter = stmt.query_map(params![revision_id, file_type.to_str()], |row| self.row_to_file(row))?;
            let mut files = Vec::new();
            for file_result in files_iter {
                files.push(file_result?);
            }
            Ok(files)
        }).map_err(DatabaseError::from)
    }

    /// Update a file
    ///
    /// # Arguments
    ///
    /// * `file` - The file to update
    ///
    /// # Returns
    ///
    /// Ok(()) if the file was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the file could not be updated
    pub fn update_file(&self, file: &File) -> DatabaseResult<()> {
        let file_id = file.file_id.ok_or_else(|| {
            DatabaseError::InitializationError("File ID is required for update".to_string())
        })?;

        self.connection_manager.execute_mut(|conn| {
            conn.execute(
                "UPDATE Files
                 SET part_id = ?2, revision_id = ?3, path = ?4, type = ?5, description = ?6
                 WHERE file_id = ?1",
                params![
                    file_id,
                    file.part_id,
                    file.revision_id,
                    file.path.to_string_lossy().to_string(),
                    file.file_type.to_str(),
                    file.description,
                ],
            )?;
            Ok(())
        }).map_err(DatabaseError::from)
    }
    
    /// Update a file within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `file` - The file to update
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// Ok(()) if the file was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the file could not be updated
    pub fn update_file_in_transaction(&self, file: &File, tx: &Transaction) -> DatabaseResult<()> {
        let file_id = file.file_id.ok_or_else(|| {
            DatabaseError::InitializationError("File ID is required for update".to_string())
        })?;

        tx.execute(
            "UPDATE Files
             SET part_id = ?2, revision_id = ?3, path = ?4, type = ?5, description = ?6
             WHERE file_id = ?1",
            params![
                file_id,
                file.part_id,
                file.revision_id,
                file.path.to_string_lossy().to_string(),
                file.file_type.to_str(),
                file.description,
            ],
        )?;
        Ok(())
    }

    /// Delete a file
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to delete
    ///
    /// # Returns
    ///
    /// Ok(()) if the file was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the file could not be deleted
    pub fn delete_file(&self, file_id: i64) -> DatabaseResult<()> {
        self.connection_manager.execute_mut(|conn| {
            conn.execute(
                "DELETE FROM Files WHERE file_id = ?1",
                params![file_id],
            )?;
            Ok(())
        }).map_err(DatabaseError::from)
    }
    
    /// Delete a file within an existing transaction
    ///
    /// # Arguments
    ///
    /// * `file_id` - The ID of the file to delete
    /// * `tx` - Transaction to use for database operations
    ///
    /// # Returns
    ///
    /// Ok(()) if the file was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the file could not be deleted
    pub fn delete_file_in_transaction(&self, file_id: i64, tx: &Transaction) -> DatabaseResult<()> {
        tx.execute(
            "DELETE FROM Files WHERE file_id = ?1",
            params![file_id],
        )?;
        Ok(())
    }

    /// Convert a database row to a File
    ///
    /// # Arguments
    ///
    /// * `row` - The database row
    ///
    /// # Returns
    ///
    /// A File instance
    ///
    /// # Errors
    ///
    /// Returns a SqliteError if the row could not be converted
    fn row_to_file(&self, row: &Row) -> SqliteResult<File> {
        let path_str: String = row.get(3)?;
        let path = PathBuf::from(path_str);

        let type_str: String = row.get(4)?;
        let file_type = FileType::from_str(&type_str);

        Ok(File {
            file_id: Some(row.get(0)?),
            part_id: row.get(1)?,
            revision_id: row.get(2)?,
            path,
            file_type,
            description: row.get(5)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::DatabaseManager;
    use crate::database::part::{Part, PartManager};
    use crate::database::revision::{Revision, RevisionStatus, RevisionManager};
    use tempfile::tempdir;

    #[test]
    fn test_part_file_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create managers
        let part_manager = PartManager::new(db_manager.connection_manager());
        let file_manager = FileManager::new(db_manager.connection_manager());

        // Create a new part
        let part = Part::new(
            10001,
            "Electronic".to_string(),
            "Resistor".to_string(),
            "10K Resistor".to_string(),
            Some("1/4W 10K Ohm Resistor".to_string()),
        );

        // Save the part to the database
        part_manager.create_part(&part).unwrap();

        // Create a new file
        let file = File::new_part_file(
            "ELE-RES-001".to_string(),
            PathBuf::from("design/resistor.kicad_sch"),
            FileType::Design,
            Some("KiCad schematic for 10K resistor".to_string()),
        );

        // Save the file to the database
        let file_id = file_manager.create_file(&file).unwrap();

        // Retrieve the file from the database
        let retrieved_file = file_manager.get_file(file_id).unwrap();

        // Check that the retrieved file matches the original
        assert_eq!(retrieved_file.part_id, file.part_id);
        assert_eq!(retrieved_file.path, file.path);
        assert_eq!(retrieved_file.file_type.to_str(), file.file_type.to_str());
        assert_eq!(retrieved_file.description, file.description);
    }

    #[test]
    fn test_revision_file_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create managers
        let part_manager = PartManager::new(db_manager.connection_manager());
        let revision_manager = RevisionManager::new(db_manager.connection_manager());
        let file_manager = FileManager::new(db_manager.connection_manager());

        // Create a new part
        let part = Part::new(
            10001,
            "Electronic".to_string(),
            "Resistor".to_string(),
            "10K Resistor".to_string(),
            Some("1/4W 10K Ohm Resistor".to_string()),
        );

        // Save the part to the database
        part_manager.create_part(&part).unwrap();

        // Create a new revision
        let revision = Revision::new(
            "ELE-RES-001".to_string(),
            "1".to_string(),
            RevisionStatus::Draft,
            "test_user".to_string(),
            Some("abc123".to_string()),
        );

        // Save the revision to the database
        let revision_id = revision_manager.create_revision(&revision).unwrap();

        // Create a new file
        let file = File::new_revision_file(
            revision_id,
            PathBuf::from("manufacturing/resistor_v1.gerber"),
            FileType::Manufacturing,
            Some("Gerber files for 10K resistor v1".to_string()),
        );

        // Save the file to the database
        let file_id = file_manager.create_file(&file).unwrap();

        // Retrieve the file from the database
        let retrieved_file = file_manager.get_file(file_id).unwrap();

        // Check that the retrieved file matches the original
        assert_eq!(retrieved_file.revision_id, file.revision_id);
        assert_eq!(retrieved_file.path, file.path);
        assert_eq!(retrieved_file.file_type.to_str(), file.file_type.to_str());
        assert_eq!(retrieved_file.description, file.description);
    }

    #[test]
    fn test_file_type_conversion() {
        assert_eq!(FileType::from_str("Design").to_str(), "Design");
        assert_eq!(FileType::from_str("Documentation").to_str(), "Documentation");
        assert_eq!(FileType::from_str("Manufacturing").to_str(), "Manufacturing");
        assert_eq!(FileType::from_str("Test").to_str(), "Test");
        assert_eq!(FileType::from_str("Image").to_str(), "Image");
        assert_eq!(FileType::from_str("Model3D").to_str(), "Model3D");
        assert_eq!(FileType::from_str("SourceCode").to_str(), "SourceCode");
        assert_eq!(FileType::from_str("Custom").to_str(), "Custom");
    }
}