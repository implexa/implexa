//! Category module for Implexa
//!
//! This module provides functionality for managing categories and subcategories in the database.

use rusqlite::{params, Row, Result as SqliteResult};
use crate::database::schema::DatabaseResult;
use crate::database::connection_manager::ConnectionManager;

/// Represents a category in the system
#[derive(Debug, Clone)]
pub struct Category {
    /// Unique identifier for the category
    pub category_id: Option<i64>,
    /// Name of the category
    pub name: String,
    /// Code for the category (used in part numbers)
    pub code: String,
    /// Description of the category
    pub description: Option<String>,
}

impl Category {
    /// Create a new category
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the category
    /// * `code` - Code for the category (used in part numbers)
    /// * `description` - Description of the category
    ///
    /// # Returns
    ///
    /// A new Category instance
    pub fn new(name: String, code: String, description: Option<String>) -> Self {
        Self {
            category_id: None,
            name,
            code,
            description,
        }
    }
}

/// Represents a subcategory in the system
#[derive(Debug, Clone)]
pub struct Subcategory {
    /// Unique identifier for the subcategory
    pub subcategory_id: Option<i64>,
    /// ID of the category this subcategory belongs to
    pub category_id: i64,
    /// Name of the subcategory
    pub name: String,
    /// Code for the subcategory (used in part numbers)
    pub code: String,
    /// Description of the subcategory
    pub description: Option<String>,
}

impl Subcategory {
    /// Create a new subcategory
    ///
    /// # Arguments
    ///
    /// * `category_id` - ID of the category this subcategory belongs to
    /// * `name` - Name of the subcategory
    /// * `code` - Code for the subcategory (used in part numbers)
    /// * `description` - Description of the subcategory
    ///
    /// # Returns
    ///
    /// A new Subcategory instance
    pub fn new(category_id: i64, name: String, code: String, description: Option<String>) -> Self {
        Self {
            subcategory_id: None,
            category_id,
            name,
            code,
            description,
        }
    }
}

/// Manager for category and subcategory operations
pub struct CategoryManager<'a> {
    /// Connection manager for the SQLite database
    connection_manager: &'a ConnectionManager,
}

impl<'a> CategoryManager<'a> {
    /// Create a new CategoryManager
    ///
    /// # Arguments
    ///
    /// * `connection_manager` - Connection manager for the SQLite database
    ///
    /// # Returns
    ///
    /// A new CategoryManager instance
    pub fn new(connection_manager: &'a ConnectionManager) -> Self {
        Self { connection_manager }
    }

    /// Create a new category in the database
    ///
    /// # Arguments
    ///
    /// * `category` - The category to create
    ///
    /// # Returns
    ///
    /// The ID of the newly created category
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the category could not be created
    pub fn create_category(&self, category: &Category) -> DatabaseResult<i64> {
        self.connection_manager.execute_mut(|conn| {
            conn.execute(
                "INSERT INTO Categories (name, code, description)
                 VALUES (?1, ?2, ?3)",
                params![
                    category.name,
                    category.code,
                    category.description,
                ],
            )?;
            Ok(conn.last_insert_rowid())
        })
    }

    /// Get a category by its ID
    ///
    /// # Arguments
    ///
    /// * `category_id` - The ID of the category to get
    ///
    /// # Returns
    ///
    /// The category with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the category could not be found
    pub fn get_category(&self, category_id: i64) -> DatabaseResult<Category> {
        self.connection_manager.execute(|conn| {
            let category = conn.query_row(
                "SELECT category_id, name, code, description
                 FROM Categories
                 WHERE category_id = ?1",
                params![category_id],
                |row| self.row_to_category(row),
            )?;
            Ok(category)
        })
    }

    /// Get a category by its code
    ///
    /// # Arguments
    ///
    /// * `code` - The code of the category to get
    ///
    /// # Returns
    ///
    /// The category with the specified code
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the category could not be found
    pub fn get_category_by_code(&self, code: &str) -> DatabaseResult<Category> {
        self.connection_manager.execute(|conn| {
            let category = conn.query_row(
                "SELECT category_id, name, code, description
                 FROM Categories
                 WHERE code = ?1",
                params![code],
                |row| self.row_to_category(row),
            )?;
            Ok(category)
        })
    }

    /// Get all categories
    ///
    /// # Returns
    ///
    /// A vector of all categories
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the categories could not be retrieved
    pub fn get_all_categories(&self) -> DatabaseResult<Vec<Category>> {
        self.connection_manager.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT category_id, name, code, description
                 FROM Categories
                 ORDER BY name",
            )?;
            let categories_iter = stmt.query_map([], |row| self.row_to_category(row))?;
            let mut categories = Vec::new();
            for category_result in categories_iter {
                categories.push(category_result?);
            }
            Ok(categories)
        })
    }

    /// Update a category
    ///
    /// # Arguments
    ///
    /// * `category` - The category to update
    ///
    /// # Returns
    ///
    /// Ok(()) if the category was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the category could not be updated
    pub fn update_category(&self, category: &Category) -> DatabaseResult<()> {
        let category_id = category.category_id.ok_or_else(|| {
            rusqlite::Error::InvalidParameterName("Category ID is required for update".to_string())
        })?;

        self.connection_manager.execute_mut(|conn| {
            conn.execute(
                "UPDATE Categories
                 SET name = ?2, code = ?3, description = ?4
                 WHERE category_id = ?1",
                params![
                    category_id,
                    category.name,
                    category.code,
                    category.description,
                ],
            )?;
            Ok(())
        })
    }

    /// Delete a category
    ///
    /// # Arguments
    ///
    /// * `category_id` - The ID of the category to delete
    ///
    /// # Returns
    ///
    /// Ok(()) if the category was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the category could not be deleted
    pub fn delete_category(&self, category_id: i64) -> DatabaseResult<()> {
        self.connection_manager.execute_mut(|conn| {
            conn.execute(
                "DELETE FROM Categories WHERE category_id = ?1",
                params![category_id],
            )?;
            Ok(())
        })
    }

    /// Create a new subcategory in the database
    ///
    /// # Arguments
    ///
    /// * `subcategory` - The subcategory to create
    ///
    /// # Returns
    ///
    /// The ID of the newly created subcategory
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the subcategory could not be created
    pub fn create_subcategory(&self, subcategory: &Subcategory) -> DatabaseResult<i64> {
        self.connection_manager.execute_mut(|conn| {
            conn.execute(
                "INSERT INTO Subcategories (category_id, name, code, description)
                 VALUES (?1, ?2, ?3, ?4)",
                params![
                    subcategory.category_id,
                    subcategory.name,
                    subcategory.code,
                    subcategory.description,
                ],
            )?;
            Ok(conn.last_insert_rowid())
        })
    }

    /// Get a subcategory by its ID
    ///
    /// # Arguments
    ///
    /// * `subcategory_id` - The ID of the subcategory to get
    ///
    /// # Returns
    ///
    /// The subcategory with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the subcategory could not be found
    pub fn get_subcategory(&self, subcategory_id: i64) -> DatabaseResult<Subcategory> {
        self.connection_manager.execute(|conn| {
            let subcategory = conn.query_row(
                "SELECT subcategory_id, category_id, name, code, description
                 FROM Subcategories
                 WHERE subcategory_id = ?1",
                params![subcategory_id],
                |row| self.row_to_subcategory(row),
            )?;
            Ok(subcategory)
        })
    }

    /// Get a subcategory by its code and category ID
    ///
    /// # Arguments
    ///
    /// * `category_id` - The ID of the category
    /// * `code` - The code of the subcategory to get
    ///
    /// # Returns
    ///
    /// The subcategory with the specified code and category ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the subcategory could not be found
    pub fn get_subcategory_by_code(&self, category_id: i64, code: &str) -> DatabaseResult<Subcategory> {
        self.connection_manager.execute(|conn| {
            let subcategory = conn.query_row(
                "SELECT subcategory_id, category_id, name, code, description
                 FROM Subcategories
                 WHERE category_id = ?1 AND code = ?2",
                params![category_id, code],
                |row| self.row_to_subcategory(row),
            )?;
            Ok(subcategory)
        })
    }

    /// Get all subcategories for a category
    ///
    /// # Arguments
    ///
    /// * `category_id` - The ID of the category
    ///
    /// # Returns
    ///
    /// A vector of all subcategories for the specified category
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the subcategories could not be retrieved
    pub fn get_subcategories_for_category(&self, category_id: i64) -> DatabaseResult<Vec<Subcategory>> {
        self.connection_manager.execute(|conn| {
            let mut stmt = conn.prepare(
                "SELECT subcategory_id, category_id, name, code, description
                 FROM Subcategories
                 WHERE category_id = ?1
                 ORDER BY name",
            )?;
            let subcategories_iter = stmt.query_map(params![category_id], |row| self.row_to_subcategory(row))?;
            let mut subcategories = Vec::new();
            for subcategory_result in subcategories_iter {
                subcategories.push(subcategory_result?);
            }
            Ok(subcategories)
        })
    }

    /// Update a subcategory
    ///
    /// # Arguments
    ///
    /// * `subcategory` - The subcategory to update
    ///
    /// # Returns
    ///
    /// Ok(()) if the subcategory was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the subcategory could not be updated
    pub fn update_subcategory(&self, subcategory: &Subcategory) -> DatabaseResult<()> {
        let subcategory_id = subcategory.subcategory_id.ok_or_else(|| {
            rusqlite::Error::InvalidParameterName("Subcategory ID is required for update".to_string())
        })?;

        self.connection_manager.execute_mut(|conn| {
            conn.execute(
                "UPDATE Subcategories
                 SET category_id = ?2, name = ?3, code = ?4, description = ?5
                 WHERE subcategory_id = ?1",
                params![
                    subcategory_id,
                    subcategory.category_id,
                    subcategory.name,
                    subcategory.code,
                    subcategory.description,
                ],
            )?;
            Ok(())
        })
    }

    /// Delete a subcategory
    ///
    /// # Arguments
    ///
    /// * `subcategory_id` - The ID of the subcategory to delete
    ///
    /// # Returns
    ///
    /// Ok(()) if the subcategory was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the subcategory could not be deleted
    pub fn delete_subcategory(&self, subcategory_id: i64) -> DatabaseResult<()> {
        self.connection_manager.execute_mut(|conn| {
            conn.execute(
                "DELETE FROM Subcategories WHERE subcategory_id = ?1",
                params![subcategory_id],
            )?;
            Ok(())
        })
    }

    /// Convert a database row to a Category
    ///
    /// # Arguments
    ///
    /// * `row` - The database row
    ///
    /// # Returns
    ///
    /// A Category instance
    ///
    /// # Errors
    ///
    /// Returns a SqliteError if the row could not be converted
    fn row_to_category(&self, row: &Row) -> SqliteResult<Category> {
        Ok(Category {
            category_id: Some(row.get(0)?),
            name: row.get(1)?,
            code: row.get(2)?,
            description: row.get(3)?,
        })
    }

    /// Convert a database row to a Subcategory
    ///
    /// # Arguments
    ///
    /// * `row` - The database row
    ///
    /// # Returns
    ///
    /// A Subcategory instance
    ///
    /// # Errors
    ///
    /// Returns a SqliteError if the row could not be converted
    fn row_to_subcategory(&self, row: &Row) -> SqliteResult<Subcategory> {
        Ok(Subcategory {
            subcategory_id: Some(row.get(0)?),
            category_id: row.get(1)?,
            name: row.get(2)?,
            code: row.get(3)?,
            description: row.get(4)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::DatabaseManager;
    use tempfile::tempdir;

    #[test]
    fn test_category_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create a category manager
        let category_manager = CategoryManager::new(db_manager.connection_manager());

        // Create a new category
        let category = Category::new(
            "Test Category".to_string(),
            "TST".to_string(),
            Some("Test category description".to_string()),
        );

        // Save the category to the database
        let category_id = category_manager.create_category(&category).unwrap();

        // Retrieve the category from the database
        let retrieved_category = category_manager.get_category(category_id).unwrap();

        // Check that the retrieved category matches the original
        assert_eq!(retrieved_category.name, category.name);
        assert_eq!(retrieved_category.code, category.code);
        assert_eq!(retrieved_category.description, category.description);

        // Create a new subcategory
        let subcategory = Subcategory::new(
            category_id,
            "Test Subcategory".to_string(),
            "SUB".to_string(),
            Some("Test subcategory description".to_string()),
        );

        // Save the subcategory to the database
        let subcategory_id = category_manager.create_subcategory(&subcategory).unwrap();

        // Retrieve the subcategory from the database
        let retrieved_subcategory = category_manager.get_subcategory(subcategory_id).unwrap();

        // Check that the retrieved subcategory matches the original
        assert_eq!(retrieved_subcategory.category_id, subcategory.category_id);
        assert_eq!(retrieved_subcategory.name, subcategory.name);
        assert_eq!(retrieved_subcategory.code, subcategory.code);
        assert_eq!(retrieved_subcategory.description, subcategory.description);

        // Get all subcategories for the category
        let subcategories = category_manager.get_subcategories_for_category(category_id).unwrap();
        assert_eq!(subcategories.len(), 1);
        assert_eq!(subcategories[0].name, subcategory.name);
    }
}