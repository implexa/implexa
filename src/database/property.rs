//! Property module for Implexa
//!
//! This module provides functionality for managing properties of parts and revisions in the database.

use rusqlite::{Connection, params, Row, Result as SqliteResult};
use crate::database::schema::{DatabaseError, DatabaseResult};

/// Type of property value
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    /// String value
    String,
    /// Integer value
    Integer,
    /// Float value
    Float,
    /// Boolean value
    Boolean,
    /// Date value
    Date,
    /// URL value
    Url,
    /// JSON value
    Json,
}

impl PropertyType {
    /// Convert a string to a PropertyType
    ///
    /// # Arguments
    ///
    /// * `type_str` - The property type string
    ///
    /// # Returns
    ///
    /// The corresponding PropertyType
    pub fn from_str(type_str: &str) -> Option<Self> {
        match type_str {
            "string" => Some(Self::String),
            "integer" => Some(Self::Integer),
            "float" => Some(Self::Float),
            "boolean" => Some(Self::Boolean),
            "date" => Some(Self::Date),
            "url" => Some(Self::Url),
            "json" => Some(Self::Json),
            _ => None,
        }
    }

    /// Convert a PropertyType to a string
    ///
    /// # Returns
    ///
    /// The string representation of the property type
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Integer => "integer",
            Self::Float => "float",
            Self::Boolean => "boolean",
            Self::Date => "date",
            Self::Url => "url",
            Self::Json => "json",
        }
    }
}

/// Represents a property of a part or revision
#[derive(Debug, Clone)]
pub struct Property {
    /// Unique identifier for the property
    pub property_id: Option<i64>,
    /// ID of the part this property belongs to (if applicable)
    pub part_id: Option<String>,
    /// ID of the revision this property belongs to (if applicable)
    pub revision_id: Option<i64>,
    /// Key of the property
    pub key: String,
    /// Value of the property
    pub value: Option<String>,
    /// Type of the property value
    pub property_type: PropertyType,
}

impl Property {
    /// Create a new property for a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - ID of the part this property belongs to
    /// * `key` - Key of the property
    /// * `value` - Value of the property
    /// * `property_type` - Type of the property value
    ///
    /// # Returns
    ///
    /// A new Property instance
    pub fn new_part_property(
        part_id: String,
        key: String,
        value: Option<String>,
        property_type: PropertyType,
    ) -> Self {
        Self {
            property_id: None,
            part_id: Some(part_id),
            revision_id: None,
            key,
            value,
            property_type,
        }
    }

    /// Create a new property for a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - ID of the revision this property belongs to
    /// * `key` - Key of the property
    /// * `value` - Value of the property
    /// * `property_type` - Type of the property value
    ///
    /// # Returns
    ///
    /// A new Property instance
    pub fn new_revision_property(
        revision_id: i64,
        key: String,
        value: Option<String>,
        property_type: PropertyType,
    ) -> Self {
        Self {
            property_id: None,
            part_id: None,
            revision_id: Some(revision_id),
            key,
            value,
            property_type,
        }
    }
}

/// Manager for property operations
pub struct PropertyManager<'a> {
    /// Connection to the SQLite database
    connection: &'a Connection,
}

impl<'a> PropertyManager<'a> {
    /// Create a new PropertyManager
    ///
    /// # Arguments
    ///
    /// * `connection` - Connection to the SQLite database
    ///
    /// # Returns
    ///
    /// A new PropertyManager instance
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    /// Create a new property in the database
    ///
    /// # Arguments
    ///
    /// * `property` - The property to create
    ///
    /// # Returns
    ///
    /// The ID of the newly created property
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the property could not be created
    pub fn create_property(&self, property: &Property) -> DatabaseResult<i64> {
        self.connection.execute(
            "INSERT INTO Properties (part_id, revision_id, key, value, type)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                property.part_id,
                property.revision_id,
                property.key,
                property.value,
                property.property_type.to_str(),
            ],
        )?;
        Ok(self.connection.last_insert_rowid())
    }

    /// Get a property by its ID
    ///
    /// # Arguments
    ///
    /// * `property_id` - The ID of the property to get
    ///
    /// # Returns
    ///
    /// The property with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the property could not be found
    pub fn get_property(&self, property_id: i64) -> DatabaseResult<Property> {
        let property = self.connection.query_row(
            "SELECT property_id, part_id, revision_id, key, value, type
             FROM Properties
             WHERE property_id = ?1",
            params![property_id],
            |row| self.row_to_property(row),
        )?;
        Ok(property)
    }

    /// Get all properties for a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part
    ///
    /// # Returns
    ///
    /// A vector of properties for the specified part
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the properties could not be retrieved
    pub fn get_part_properties(&self, part_id: &str) -> DatabaseResult<Vec<Property>> {
        let mut stmt = self.connection.prepare(
            "SELECT property_id, part_id, revision_id, key, value, type
             FROM Properties
             WHERE part_id = ?1
             ORDER BY key",
        )?;
        let properties_iter = stmt.query_map(params![part_id], |row| self.row_to_property(row))?;
        let mut properties = Vec::new();
        for property_result in properties_iter {
            properties.push(property_result?);
        }
        Ok(properties)
    }

    /// Get all properties for a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision
    ///
    /// # Returns
    ///
    /// A vector of properties for the specified revision
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the properties could not be retrieved
    pub fn get_revision_properties(&self, revision_id: i64) -> DatabaseResult<Vec<Property>> {
        let mut stmt = self.connection.prepare(
            "SELECT property_id, part_id, revision_id, key, value, type
             FROM Properties
             WHERE revision_id = ?1
             ORDER BY key",
        )?;
        let properties_iter = stmt.query_map(params![revision_id], |row| self.row_to_property(row))?;
        let mut properties = Vec::new();
        for property_result in properties_iter {
            properties.push(property_result?);
        }
        Ok(properties)
    }

    /// Get a specific property for a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part
    /// * `key` - The key of the property
    ///
    /// # Returns
    ///
    /// The property with the specified key for the specified part
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the property could not be found
    pub fn get_part_property(&self, part_id: &str, key: &str) -> DatabaseResult<Property> {
        let property = self.connection.query_row(
            "SELECT property_id, part_id, revision_id, key, value, type
             FROM Properties
             WHERE part_id = ?1 AND key = ?2",
            params![part_id, key],
            |row| self.row_to_property(row),
        )?;
        Ok(property)
    }

    /// Get a specific property for a revision
    ///
    /// # Arguments
    ///
    /// * `revision_id` - The ID of the revision
    /// * `key` - The key of the property
    ///
    /// # Returns
    ///
    /// The property with the specified key for the specified revision
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the property could not be found
    pub fn get_revision_property(&self, revision_id: i64, key: &str) -> DatabaseResult<Property> {
        let property = self.connection.query_row(
            "SELECT property_id, part_id, revision_id, key, value, type
             FROM Properties
             WHERE revision_id = ?1 AND key = ?2",
            params![revision_id, key],
            |row| self.row_to_property(row),
        )?;
        Ok(property)
    }

    /// Update a property
    ///
    /// # Arguments
    ///
    /// * `property` - The property to update
    ///
    /// # Returns
    ///
    /// Ok(()) if the property was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the property could not be updated
    pub fn update_property(&self, property: &Property) -> DatabaseResult<()> {
        let property_id = property.property_id.ok_or_else(|| {
            DatabaseError::InitializationError("Property ID is required for update".to_string())
        })?;

        self.connection.execute(
            "UPDATE Properties
             SET part_id = ?2, revision_id = ?3, key = ?4, value = ?5, type = ?6
             WHERE property_id = ?1",
            params![
                property_id,
                property.part_id,
                property.revision_id,
                property.key,
                property.value,
                property.property_type.to_str(),
            ],
        )?;
        Ok(())
    }

    /// Delete a property
    ///
    /// # Arguments
    ///
    /// * `property_id` - The ID of the property to delete
    ///
    /// # Returns
    ///
    /// Ok(()) if the property was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the property could not be deleted
    pub fn delete_property(&self, property_id: i64) -> DatabaseResult<()> {
        self.connection.execute(
            "DELETE FROM Properties WHERE property_id = ?1",
            params![property_id],
        )?;
        Ok(())
    }

    /// Convert a database row to a Property
    ///
    /// # Arguments
    ///
    /// * `row` - The database row
    ///
    /// # Returns
    ///
    /// A Property instance
    ///
    /// # Errors
    ///
    /// Returns a SqliteError if the row could not be converted
    fn row_to_property(&self, row: &Row) -> SqliteResult<Property> {
        let type_str: String = row.get(5)?;
        let property_type = PropertyType::from_str(&type_str)
            .unwrap_or(PropertyType::String);

        Ok(Property {
            property_id: Some(row.get(0)?),
            part_id: row.get(1)?,
            revision_id: row.get(2)?,
            key: row.get(3)?,
            value: row.get(4)?,
            property_type,
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
    fn test_part_property_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let mut db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create a part manager and a property manager
        let part_manager = PartManager::new(db_manager.connection());
        let property_manager = PropertyManager::new(db_manager.connection());

        // Create a new part
        let part = Part::new(
            "ELE-RES-001".to_string(),
            "Electronic".to_string(),
            "Resistor".to_string(),
            "10K Resistor".to_string(),
            Some("1/4W 10K Ohm Resistor".to_string()),
        );

        // Save the part to the database
        part_manager.create_part(&part).unwrap();

        // Create a new property
        let property = Property::new_part_property(
            "ELE-RES-001".to_string(),
            "resistance".to_string(),
            Some("10000".to_string()),
            PropertyType::Integer,
        );

        // Save the property to the database
        let property_id = property_manager.create_property(&property).unwrap();

        // Retrieve the property from the database
        let retrieved_property = property_manager.get_property(property_id).unwrap();

        // Check that the retrieved property matches the original
        assert_eq!(retrieved_property.part_id, property.part_id);
        assert_eq!(retrieved_property.key, property.key);
        assert_eq!(retrieved_property.value, property.value);
        assert_eq!(retrieved_property.property_type.to_str(), property.property_type.to_str());
    }

    #[test]
    fn test_revision_property_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let mut db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create managers
        let part_manager = PartManager::new(db_manager.connection());
        let revision_manager = RevisionManager::new(db_manager.connection());
        let property_manager = PropertyManager::new(db_manager.connection());

        // Create a new part
        let part = Part::new(
            "ELE-RES-001".to_string(),
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

        // Create a new property
        let property = Property::new_revision_property(
            revision_id,
            "tolerance".to_string(),
            Some("5%".to_string()),
            PropertyType::String,
        );

        // Save the property to the database
        let property_id = property_manager.create_property(&property).unwrap();

        // Retrieve the property from the database
        let retrieved_property = property_manager.get_property(property_id).unwrap();

        // Check that the retrieved property matches the original
        assert_eq!(retrieved_property.revision_id, property.revision_id);
        assert_eq!(retrieved_property.key, property.key);
        assert_eq!(retrieved_property.value, property.value);
        assert_eq!(retrieved_property.property_type.to_str(), property.property_type.to_str());
    }

    #[test]
    fn test_property_type_conversion() {
        assert_eq!(PropertyType::from_str("string"), Some(PropertyType::String));
        assert_eq!(PropertyType::from_str("integer"), Some(PropertyType::Integer));
        assert_eq!(PropertyType::from_str("float"), Some(PropertyType::Float));
        assert_eq!(PropertyType::from_str("boolean"), Some(PropertyType::Boolean));
        assert_eq!(PropertyType::from_str("date"), Some(PropertyType::Date));
        assert_eq!(PropertyType::from_str("url"), Some(PropertyType::Url));
        assert_eq!(PropertyType::from_str("json"), Some(PropertyType::Json));
        assert_eq!(PropertyType::from_str("invalid"), None);

        assert_eq!(PropertyType::String.to_str(), "string");
        assert_eq!(PropertyType::Integer.to_str(), "integer");
        assert_eq!(PropertyType::Float.to_str(), "float");
        assert_eq!(PropertyType::Boolean.to_str(), "boolean");
        assert_eq!(PropertyType::Date.to_str(), "date");
        assert_eq!(PropertyType::Url.to_str(), "url");
        assert_eq!(PropertyType::Json.to_str(), "json");
    }
}