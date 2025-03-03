//! Relationship module for Implexa
//!
//! This module provides functionality for managing relationships between parts in the database.

use rusqlite::{Connection, params, Row, Result as SqliteResult};
use crate::database::schema::{DatabaseError, DatabaseResult};

/// Type of relationship between parts
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationshipType {
    /// Assembly relationship - parent contains child
    Assembly,
    /// Reference relationship - parent references child
    Reference,
    /// Variant relationship - parent is a variant of child
    Variant,
    /// Alternate relationship - parent is an alternate for child
    Alternate,
    /// Custom relationship type
    Custom(String),
}

impl RelationshipType {
    /// Convert a string to a RelationshipType
    ///
    /// # Arguments
    ///
    /// * `type_str` - The relationship type string
    ///
    /// # Returns
    ///
    /// The corresponding RelationshipType
    pub fn from_str(type_str: &str) -> Self {
        match type_str {
            "Assembly" => Self::Assembly,
            "Reference" => Self::Reference,
            "Variant" => Self::Variant,
            "Alternate" => Self::Alternate,
            _ => Self::Custom(type_str.to_string()),
        }
    }

    /// Convert a RelationshipType to a string
    ///
    /// # Returns
    ///
    /// The string representation of the relationship type
    pub fn to_str(&self) -> String {
        match self {
            Self::Assembly => "Assembly".to_string(),
            Self::Reference => "Reference".to_string(),
            Self::Variant => "Variant".to_string(),
            Self::Alternate => "Alternate".to_string(),
            Self::Custom(s) => s.clone(),
        }
    }
}

/// Represents a relationship between parts
#[derive(Debug, Clone)]
pub struct Relationship {
    /// Unique identifier for the relationship
    pub relationship_id: Option<i64>,
    /// ID of the parent part
    pub parent_part_id: String,
    /// ID of the child part
    pub child_part_id: String,
    /// Type of relationship
    pub relationship_type: RelationshipType,
    /// Quantity of child parts in the relationship
    pub quantity: i64,
}

impl Relationship {
    /// Create a new relationship
    ///
    /// # Arguments
    ///
    /// * `parent_part_id` - ID of the parent part
    /// * `child_part_id` - ID of the child part
    /// * `relationship_type` - Type of relationship
    /// * `quantity` - Quantity of child parts in the relationship
    ///
    /// # Returns
    ///
    /// A new Relationship instance
    pub fn new(
        parent_part_id: String,
        child_part_id: String,
        relationship_type: RelationshipType,
        quantity: i64,
    ) -> Self {
        Self {
            relationship_id: None,
            parent_part_id,
            child_part_id,
            relationship_type,
            quantity,
        }
    }
}

/// Manager for relationship operations
pub struct RelationshipManager<'a> {
    /// Connection to the SQLite database
    connection: &'a Connection,
}

impl<'a> RelationshipManager<'a> {
    /// Create a new RelationshipManager
    ///
    /// # Arguments
    ///
    /// * `connection` - Connection to the SQLite database
    ///
    /// # Returns
    ///
    /// A new RelationshipManager instance
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    /// Create a new relationship in the database
    ///
    /// # Arguments
    ///
    /// * `relationship` - The relationship to create
    ///
    /// # Returns
    ///
    /// The ID of the newly created relationship
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the relationship could not be created
    pub fn create_relationship(&self, relationship: &Relationship) -> DatabaseResult<i64> {
        self.connection.execute(
            "INSERT INTO Relationships (parent_part_id, child_part_id, type, quantity)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                relationship.parent_part_id,
                relationship.child_part_id,
                relationship.relationship_type.to_str(),
                relationship.quantity,
            ],
        )?;
        Ok(self.connection.last_insert_rowid())
    }

    /// Get a relationship by its ID
    ///
    /// # Arguments
    ///
    /// * `relationship_id` - The ID of the relationship to get
    ///
    /// # Returns
    ///
    /// The relationship with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the relationship could not be found
    pub fn get_relationship(&self, relationship_id: i64) -> DatabaseResult<Relationship> {
        let relationship = self.connection.query_row(
            "SELECT relationship_id, parent_part_id, child_part_id, type, quantity
             FROM Relationships
             WHERE relationship_id = ?1",
            params![relationship_id],
            |row| self.row_to_relationship(row),
        )?;
        Ok(relationship)
    }

    /// Get all relationships where the specified part is the parent
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the parent part
    ///
    /// # Returns
    ///
    /// A vector of relationships where the specified part is the parent
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the relationships could not be retrieved
    pub fn get_child_relationships(&self, part_id: &str) -> DatabaseResult<Vec<Relationship>> {
        let mut stmt = self.connection.prepare(
            "SELECT relationship_id, parent_part_id, child_part_id, type, quantity
             FROM Relationships
             WHERE parent_part_id = ?1",
        )?;
        let relationships_iter = stmt.query_map(params![part_id], |row| self.row_to_relationship(row))?;
        let mut relationships = Vec::new();
        for relationship_result in relationships_iter {
            relationships.push(relationship_result?);
        }
        Ok(relationships)
    }

    /// Get all relationships where the specified part is the child
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the child part
    ///
    /// # Returns
    ///
    /// A vector of relationships where the specified part is the child
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the relationships could not be retrieved
    pub fn get_parent_relationships(&self, part_id: &str) -> DatabaseResult<Vec<Relationship>> {
        let mut stmt = self.connection.prepare(
            "SELECT relationship_id, parent_part_id, child_part_id, type, quantity
             FROM Relationships
             WHERE child_part_id = ?1",
        )?;
        let relationships_iter = stmt.query_map(params![part_id], |row| self.row_to_relationship(row))?;
        let mut relationships = Vec::new();
        for relationship_result in relationships_iter {
            relationships.push(relationship_result?);
        }
        Ok(relationships)
    }

    /// Update a relationship
    ///
    /// # Arguments
    ///
    /// * `relationship` - The relationship to update
    ///
    /// # Returns
    ///
    /// Ok(()) if the relationship was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the relationship could not be updated
    pub fn update_relationship(&self, relationship: &Relationship) -> DatabaseResult<()> {
        let relationship_id = relationship.relationship_id.ok_or_else(|| {
            DatabaseError::InitializationError("Relationship ID is required for update".to_string())
        })?;

        self.connection.execute(
            "UPDATE Relationships
             SET parent_part_id = ?2, child_part_id = ?3, type = ?4, quantity = ?5
             WHERE relationship_id = ?1",
            params![
                relationship_id,
                relationship.parent_part_id,
                relationship.child_part_id,
                relationship.relationship_type.to_str(),
                relationship.quantity,
            ],
        )?;
        Ok(())
    }

    /// Delete a relationship
    ///
    /// # Arguments
    ///
    /// * `relationship_id` - The ID of the relationship to delete
    ///
    /// # Returns
    ///
    /// Ok(()) if the relationship was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the relationship could not be deleted
    pub fn delete_relationship(&self, relationship_id: i64) -> DatabaseResult<()> {
        self.connection.execute(
            "DELETE FROM Relationships WHERE relationship_id = ?1",
            params![relationship_id],
        )?;
        Ok(())
    }

    /// Get the bill of materials (BOM) for a part
    ///
    /// # Arguments
    ///
    /// * `part_id` - The ID of the part
    ///
    /// # Returns
    ///
    /// A vector of (part_id, name, category, subcategory, quantity) tuples
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the BOM could not be retrieved
    pub fn get_bom(&self, part_id: &str) -> DatabaseResult<Vec<(String, String, String, String, i64)>> {
        let mut stmt = self.connection.prepare(
            "SELECT p.part_id, p.name, p.category, p.subcategory, r.quantity
             FROM Parts p
             JOIN Relationships r ON p.part_id = r.child_part_id
             WHERE r.parent_part_id = ?1 AND r.type = 'Assembly'",
        )?;
        let bom_iter = stmt.query_map(params![part_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4)?,
            ))
        })?;
        let mut bom = Vec::new();
        for bom_result in bom_iter {
            bom.push(bom_result?);
        }
        Ok(bom)
    }

    /// Convert a database row to a Relationship
    ///
    /// # Arguments
    ///
    /// * `row` - The database row
    ///
    /// # Returns
    ///
    /// A Relationship instance
    ///
    /// # Errors
    ///
    /// Returns a SqliteError if the row could not be converted
    fn row_to_relationship(&self, row: &Row) -> SqliteResult<Relationship> {
        let type_str: String = row.get(3)?;
        let relationship_type = RelationshipType::from_str(&type_str);

        Ok(Relationship {
            relationship_id: Some(row.get(0)?),
            parent_part_id: row.get(1)?,
            child_part_id: row.get(2)?,
            relationship_type,
            quantity: row.get(4)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::DatabaseManager;
    use crate::database::part::{Part, PartManager};
    use tempfile::tempdir;

    #[test]
    fn test_relationship_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let mut db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create a part manager and a relationship manager
        let part_manager = PartManager::new(db_manager.connection());
        let relationship_manager = RelationshipManager::new(db_manager.connection());

        // Create two parts
        let parent_part = Part::new(
            "ASM-PCB-001".to_string(),
            "Assembly".to_string(),
            "PCB".to_string(),
            "Main PCB Assembly".to_string(),
            Some("Main PCB Assembly for Product X".to_string()),
        );

        let child_part = Part::new(
            "ELE-RES-001".to_string(),
            "Electronic".to_string(),
            "Resistor".to_string(),
            "10K Resistor".to_string(),
            Some("1/4W 10K Ohm Resistor".to_string()),
        );

        // Save the parts to the database
        part_manager.create_part(&parent_part).unwrap();
        part_manager.create_part(&child_part).unwrap();

        // Create a relationship
        let relationship = Relationship::new(
            "ASM-PCB-001".to_string(),
            "ELE-RES-001".to_string(),
            RelationshipType::Assembly,
            10,
        );

        // Save the relationship to the database
        let relationship_id = relationship_manager.create_relationship(&relationship).unwrap();

        // Retrieve the relationship from the database
        let retrieved_relationship = relationship_manager.get_relationship(relationship_id).unwrap();

        // Check that the retrieved relationship matches the original
        assert_eq!(retrieved_relationship.parent_part_id, relationship.parent_part_id);
        assert_eq!(retrieved_relationship.child_part_id, relationship.child_part_id);
        assert_eq!(
            retrieved_relationship.relationship_type.to_str(),
            relationship.relationship_type.to_str()
        );
        assert_eq!(retrieved_relationship.quantity, relationship.quantity);
    }

    #[test]
    fn test_relationship_type_conversion() {
        assert_eq!(RelationshipType::from_str("Assembly").to_str(), "Assembly");
        assert_eq!(RelationshipType::from_str("Reference").to_str(), "Reference");
        assert_eq!(RelationshipType::from_str("Variant").to_str(), "Variant");
        assert_eq!(RelationshipType::from_str("Alternate").to_str(), "Alternate");
        assert_eq!(RelationshipType::from_str("Custom").to_str(), "Custom");
    }
}