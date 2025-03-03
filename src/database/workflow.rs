//! Workflow module for Implexa
//!
//! This module provides functionality for managing workflows, workflow states, and workflow transitions in the database.

use rusqlite::{Connection, params, Row, Result as SqliteResult};
use crate::database::schema::{DatabaseError, DatabaseResult};

/// Represents a workflow
#[derive(Debug, Clone)]
pub struct Workflow {
    /// Unique identifier for the workflow
    pub workflow_id: Option<i64>,
    /// Name of the workflow
    pub name: String,
    /// Description of the workflow
    pub description: Option<String>,
    /// Whether the workflow is active
    pub active: bool,
}

impl Workflow {
    /// Create a new workflow
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the workflow
    /// * `description` - Description of the workflow
    /// * `active` - Whether the workflow is active
    ///
    /// # Returns
    ///
    /// A new Workflow instance
    pub fn new(
        name: String,
        description: Option<String>,
        active: bool,
    ) -> Self {
        Self {
            workflow_id: None,
            name,
            description,
            active,
        }
    }
}

/// Represents a state in a workflow
#[derive(Debug, Clone)]
pub struct WorkflowState {
    /// Unique identifier for the workflow state
    pub state_id: Option<i64>,
    /// ID of the workflow this state belongs to
    pub workflow_id: i64,
    /// Name of the state
    pub name: String,
    /// Description of the state
    pub description: Option<String>,
    /// Whether this is an initial state
    pub is_initial: bool,
    /// Whether this is a terminal state
    pub is_terminal: bool,
}

impl WorkflowState {
    /// Create a new workflow state
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - ID of the workflow this state belongs to
    /// * `name` - Name of the state
    /// * `description` - Description of the state
    /// * `is_initial` - Whether this is an initial state
    /// * `is_terminal` - Whether this is a terminal state
    ///
    /// # Returns
    ///
    /// A new WorkflowState instance
    pub fn new(
        workflow_id: i64,
        name: String,
        description: Option<String>,
        is_initial: bool,
        is_terminal: bool,
    ) -> Self {
        Self {
            state_id: None,
            workflow_id,
            name,
            description,
            is_initial,
            is_terminal,
        }
    }
}

/// Represents a transition between states in a workflow
#[derive(Debug, Clone)]
pub struct WorkflowTransition {
    /// Unique identifier for the workflow transition
    pub transition_id: Option<i64>,
    /// ID of the workflow this transition belongs to
    pub workflow_id: i64,
    /// ID of the state this transition is from
    pub from_state_id: i64,
    /// ID of the state this transition is to
    pub to_state_id: i64,
    /// Name of the transition
    pub name: String,
    /// Description of the transition
    pub description: Option<String>,
    /// Whether this transition requires approval
    pub requires_approval: bool,
}

impl WorkflowTransition {
    /// Create a new workflow transition
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - ID of the workflow this transition belongs to
    /// * `from_state_id` - ID of the state this transition is from
    /// * `to_state_id` - ID of the state this transition is to
    /// * `name` - Name of the transition
    /// * `description` - Description of the transition
    /// * `requires_approval` - Whether this transition requires approval
    ///
    /// # Returns
    ///
    /// A new WorkflowTransition instance
    pub fn new(
        workflow_id: i64,
        from_state_id: i64,
        to_state_id: i64,
        name: String,
        description: Option<String>,
        requires_approval: bool,
    ) -> Self {
        Self {
            transition_id: None,
            workflow_id,
            from_state_id,
            to_state_id,
            name,
            description,
            requires_approval,
        }
    }
}

/// Manager for workflow operations
pub struct WorkflowManager<'a> {
    /// Connection to the SQLite database
    connection: &'a Connection,
}

impl<'a> WorkflowManager<'a> {
    /// Create a new WorkflowManager
    ///
    /// # Arguments
    ///
    /// * `connection` - Connection to the SQLite database
    ///
    /// # Returns
    ///
    /// A new WorkflowManager instance
    pub fn new(connection: &'a Connection) -> Self {
        Self { connection }
    }

    /// Create a new workflow in the database
    ///
    /// # Arguments
    ///
    /// * `workflow` - The workflow to create
    ///
    /// # Returns
    ///
    /// The ID of the newly created workflow
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow could not be created
    pub fn create_workflow(&self, workflow: &Workflow) -> DatabaseResult<i64> {
        self.connection.execute(
            "INSERT INTO Workflows (name, description, active)
             VALUES (?1, ?2, ?3)",
            params![
                workflow.name,
                workflow.description,
                workflow.active,
            ],
        )?;
        Ok(self.connection.last_insert_rowid())
    }

    /// Get a workflow by its ID
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The ID of the workflow to get
    ///
    /// # Returns
    ///
    /// The workflow with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow could not be found
    pub fn get_workflow(&self, workflow_id: i64) -> DatabaseResult<Workflow> {
        let workflow = self.connection.query_row(
            "SELECT workflow_id, name, description, active
             FROM Workflows
             WHERE workflow_id = ?1",
            params![workflow_id],
            |row| self.row_to_workflow(row),
        )?;
        Ok(workflow)
    }

    /// Get a workflow by its name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the workflow to get
    ///
    /// # Returns
    ///
    /// The workflow with the specified name
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow could not be found
    pub fn get_workflow_by_name(&self, name: &str) -> DatabaseResult<Workflow> {
        let workflow = self.connection.query_row(
            "SELECT workflow_id, name, description, active
             FROM Workflows
             WHERE name = ?1",
            params![name],
            |row| self.row_to_workflow(row),
        )?;
        Ok(workflow)
    }

    /// Get all workflows
    ///
    /// # Returns
    ///
    /// A vector of all workflows
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflows could not be retrieved
    pub fn get_all_workflows(&self) -> DatabaseResult<Vec<Workflow>> {
        let mut stmt = self.connection.prepare(
            "SELECT workflow_id, name, description, active
             FROM Workflows
             ORDER BY name",
        )?;
        let workflows_iter = stmt.query_map([], |row| self.row_to_workflow(row))?;
        let mut workflows = Vec::new();
        for workflow_result in workflows_iter {
            workflows.push(workflow_result?);
        }
        Ok(workflows)
    }

    /// Get all active workflows
    ///
    /// # Returns
    ///
    /// A vector of all active workflows
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflows could not be retrieved
    pub fn get_active_workflows(&self) -> DatabaseResult<Vec<Workflow>> {
        let mut stmt = self.connection.prepare(
            "SELECT workflow_id, name, description, active
             FROM Workflows
             WHERE active = 1
             ORDER BY name",
        )?;
        let workflows_iter = stmt.query_map([], |row| self.row_to_workflow(row))?;
        let mut workflows = Vec::new();
        for workflow_result in workflows_iter {
            workflows.push(workflow_result?);
        }
        Ok(workflows)
    }

    /// Update a workflow
    ///
    /// # Arguments
    ///
    /// * `workflow` - The workflow to update
    ///
    /// # Returns
    ///
    /// Ok(()) if the workflow was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow could not be updated
    pub fn update_workflow(&self, workflow: &Workflow) -> DatabaseResult<()> {
        let workflow_id = workflow.workflow_id.ok_or_else(|| {
            DatabaseError::InitializationError("Workflow ID is required for update".to_string())
        })?;

        self.connection.execute(
            "UPDATE Workflows
             SET name = ?2, description = ?3, active = ?4
             WHERE workflow_id = ?1",
            params![
                workflow_id,
                workflow.name,
                workflow.description,
                workflow.active,
            ],
        )?;
        Ok(())
    }

    /// Delete a workflow
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The ID of the workflow to delete
    ///
    /// # Returns
    ///
    /// Ok(()) if the workflow was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow could not be deleted
    pub fn delete_workflow(&self, workflow_id: i64) -> DatabaseResult<()> {
        self.connection.execute(
            "DELETE FROM Workflows WHERE workflow_id = ?1",
            params![workflow_id],
        )?;
        Ok(())
    }

    /// Create a new workflow state in the database
    ///
    /// # Arguments
    ///
    /// * `state` - The workflow state to create
    ///
    /// # Returns
    ///
    /// The ID of the newly created workflow state
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow state could not be created
    pub fn create_workflow_state(&self, state: &WorkflowState) -> DatabaseResult<i64> {
        self.connection.execute(
            "INSERT INTO WorkflowStates (workflow_id, name, description, is_initial, is_terminal)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                state.workflow_id,
                state.name,
                state.description,
                state.is_initial,
                state.is_terminal,
            ],
        )?;
        Ok(self.connection.last_insert_rowid())
    }

    /// Get a workflow state by its ID
    ///
    /// # Arguments
    ///
    /// * `state_id` - The ID of the workflow state to get
    ///
    /// # Returns
    ///
    /// The workflow state with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow state could not be found
    pub fn get_workflow_state(&self, state_id: i64) -> DatabaseResult<WorkflowState> {
        let state = self.connection.query_row(
            "SELECT state_id, workflow_id, name, description, is_initial, is_terminal
             FROM WorkflowStates
             WHERE state_id = ?1",
            params![state_id],
            |row| self.row_to_workflow_state(row),
        )?;
        Ok(state)
    }

    /// Get all workflow states for a workflow
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The ID of the workflow
    ///
    /// # Returns
    ///
    /// A vector of all workflow states for the specified workflow
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow states could not be retrieved
    pub fn get_workflow_states(&self, workflow_id: i64) -> DatabaseResult<Vec<WorkflowState>> {
        let mut stmt = self.connection.prepare(
            "SELECT state_id, workflow_id, name, description, is_initial, is_terminal
             FROM WorkflowStates
             WHERE workflow_id = ?1
             ORDER BY name",
        )?;
        let states_iter = stmt.query_map(params![workflow_id], |row| self.row_to_workflow_state(row))?;
        let mut states = Vec::new();
        for state_result in states_iter {
            states.push(state_result?);
        }
        Ok(states)
    }

    /// Get the initial state for a workflow
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The ID of the workflow
    ///
    /// # Returns
    ///
    /// The initial state for the specified workflow
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the initial state could not be found
    pub fn get_initial_state(&self, workflow_id: i64) -> DatabaseResult<WorkflowState> {
        let state = self.connection.query_row(
            "SELECT state_id, workflow_id, name, description, is_initial, is_terminal
             FROM WorkflowStates
             WHERE workflow_id = ?1 AND is_initial = 1",
            params![workflow_id],
            |row| self.row_to_workflow_state(row),
        )?;
        Ok(state)
    }

    /// Update a workflow state
    ///
    /// # Arguments
    ///
    /// * `state` - The workflow state to update
    ///
    /// # Returns
    ///
    /// Ok(()) if the workflow state was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow state could not be updated
    pub fn update_workflow_state(&self, state: &WorkflowState) -> DatabaseResult<()> {
        let state_id = state.state_id.ok_or_else(|| {
            DatabaseError::InitializationError("Workflow State ID is required for update".to_string())
        })?;

        self.connection.execute(
            "UPDATE WorkflowStates
             SET workflow_id = ?2, name = ?3, description = ?4, is_initial = ?5, is_terminal = ?6
             WHERE state_id = ?1",
            params![
                state_id,
                state.workflow_id,
                state.name,
                state.description,
                state.is_initial,
                state.is_terminal,
            ],
        )?;
        Ok(())
    }

    /// Delete a workflow state
    ///
    /// # Arguments
    ///
    /// * `state_id` - The ID of the workflow state to delete
    ///
    /// # Returns
    ///
    /// Ok(()) if the workflow state was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow state could not be deleted
    pub fn delete_workflow_state(&self, state_id: i64) -> DatabaseResult<()> {
        self.connection.execute(
            "DELETE FROM WorkflowStates WHERE state_id = ?1",
            params![state_id],
        )?;
        Ok(())
    }

    /// Create a new workflow transition in the database
    ///
    /// # Arguments
    ///
    /// * `transition` - The workflow transition to create
    ///
    /// # Returns
    ///
    /// The ID of the newly created workflow transition
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow transition could not be created
    pub fn create_workflow_transition(&self, transition: &WorkflowTransition) -> DatabaseResult<i64> {
        self.connection.execute(
            "INSERT INTO WorkflowTransitions (workflow_id, from_state_id, to_state_id, name, description, requires_approval)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                transition.workflow_id,
                transition.from_state_id,
                transition.to_state_id,
                transition.name,
                transition.description,
                transition.requires_approval,
            ],
        )?;
        Ok(self.connection.last_insert_rowid())
    }

    /// Get a workflow transition by its ID
    ///
    /// # Arguments
    ///
    /// * `transition_id` - The ID of the workflow transition to get
    ///
    /// # Returns
    ///
    /// The workflow transition with the specified ID
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow transition could not be found
    pub fn get_workflow_transition(&self, transition_id: i64) -> DatabaseResult<WorkflowTransition> {
        let transition = self.connection.query_row(
            "SELECT transition_id, workflow_id, from_state_id, to_state_id, name, description, requires_approval
             FROM WorkflowTransitions
             WHERE transition_id = ?1",
            params![transition_id],
            |row| self.row_to_workflow_transition(row),
        )?;
        Ok(transition)
    }

    /// Get all workflow transitions for a workflow
    ///
    /// # Arguments
    ///
    /// * `workflow_id` - The ID of the workflow
    ///
    /// # Returns
    ///
    /// A vector of all workflow transitions for the specified workflow
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow transitions could not be retrieved
    pub fn get_workflow_transitions(&self, workflow_id: i64) -> DatabaseResult<Vec<WorkflowTransition>> {
        let mut stmt = self.connection.prepare(
            "SELECT transition_id, workflow_id, from_state_id, to_state_id, name, description, requires_approval
             FROM WorkflowTransitions
             WHERE workflow_id = ?1
             ORDER BY name",
        )?;
        let transitions_iter = stmt.query_map(params![workflow_id], |row| self.row_to_workflow_transition(row))?;
        let mut transitions = Vec::new();
        for transition_result in transitions_iter {
            transitions.push(transition_result?);
        }
        Ok(transitions)
    }

    /// Get all workflow transitions from a specific state
    ///
    /// # Arguments
    ///
    /// * `from_state_id` - The ID of the state to get transitions from
    ///
    /// # Returns
    ///
    /// A vector of all workflow transitions from the specified state
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow transitions could not be retrieved
    pub fn get_transitions_from_state(&self, from_state_id: i64) -> DatabaseResult<Vec<WorkflowTransition>> {
        let mut stmt = self.connection.prepare(
            "SELECT transition_id, workflow_id, from_state_id, to_state_id, name, description, requires_approval
             FROM WorkflowTransitions
             WHERE from_state_id = ?1
             ORDER BY name",
        )?;
        let transitions_iter = stmt.query_map(params![from_state_id], |row| self.row_to_workflow_transition(row))?;
        let mut transitions = Vec::new();
        for transition_result in transitions_iter {
            transitions.push(transition_result?);
        }
        Ok(transitions)
    }

    /// Update a workflow transition
    ///
    /// # Arguments
    ///
    /// * `transition` - The workflow transition to update
    ///
    /// # Returns
    ///
    /// Ok(()) if the workflow transition was successfully updated
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow transition could not be updated
    pub fn update_workflow_transition(&self, transition: &WorkflowTransition) -> DatabaseResult<()> {
        let transition_id = transition.transition_id.ok_or_else(|| {
            DatabaseError::InitializationError("Workflow Transition ID is required for update".to_string())
        })?;

        self.connection.execute(
            "UPDATE WorkflowTransitions
             SET workflow_id = ?2, from_state_id = ?3, to_state_id = ?4, name = ?5, description = ?6, requires_approval = ?7
             WHERE transition_id = ?1",
            params![
                transition_id,
                transition.workflow_id,
                transition.from_state_id,
                transition.to_state_id,
                transition.name,
                transition.description,
                transition.requires_approval,
            ],
        )?;
        Ok(())
    }

    /// Delete a workflow transition
    ///
    /// # Arguments
    ///
    /// * `transition_id` - The ID of the workflow transition to delete
    ///
    /// # Returns
    ///
    /// Ok(()) if the workflow transition was successfully deleted
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow transition could not be deleted
    pub fn delete_workflow_transition(&self, transition_id: i64) -> DatabaseResult<()> {
        self.connection.execute(
            "DELETE FROM WorkflowTransitions WHERE transition_id = ?1",
            params![transition_id],
        )?;
        Ok(())
    }

    /// Create a default part workflow
    ///
    /// # Returns
    ///
    /// The ID of the newly created workflow
    ///
    /// # Errors
    ///
    /// Returns a DatabaseError if the workflow could not be created
    pub fn create_default_part_workflow(&self) -> DatabaseResult<i64> {
        // Create the workflow
        let workflow = Workflow::new(
            "Part Workflow".to_string(),
            Some("Default workflow for parts".to_string()),
            true,
        );
        let workflow_id = self.create_workflow(&workflow)?;

        // Create the states
        let draft_state = WorkflowState::new(
            workflow_id,
            "Draft".to_string(),
            Some("Initial state for new parts".to_string()),
            true,
            false,
        );
        let draft_state_id = self.create_workflow_state(&draft_state)?;

        let in_review_state = WorkflowState::new(
            workflow_id,
            "In Review".to_string(),
            Some("Part is being reviewed".to_string()),
            false,
            false,
        );
        let in_review_state_id = self.create_workflow_state(&in_review_state)?;

        let released_state = WorkflowState::new(
            workflow_id,
            "Released".to_string(),
            Some("Part has been approved and released".to_string()),
            false,
            false,
        );
        let released_state_id = self.create_workflow_state(&released_state)?;

        let obsolete_state = WorkflowState::new(
            workflow_id,
            "Obsolete".to_string(),
            Some("Part is no longer active".to_string()),
            false,
            true,
        );
        let obsolete_state_id = self.create_workflow_state(&obsolete_state)?;

        // Create the transitions
        let submit_for_review = WorkflowTransition::new(
            workflow_id,
            draft_state_id,
            in_review_state_id,
            "Submit for Review".to_string(),
            Some("Submit the part for review".to_string()),
            false,
        );
        self.create_workflow_transition(&submit_for_review)?;

        let approve = WorkflowTransition::new(
            workflow_id,
            in_review_state_id,
            released_state_id,
            "Approve".to_string(),
            Some("Approve the part for release".to_string()),
            true,
        );
        self.create_workflow_transition(&approve)?;

        let reject = WorkflowTransition::new(
            workflow_id,
            in_review_state_id,
            draft_state_id,
            "Reject".to_string(),
            Some("Reject the part and return to draft".to_string()),
            true,
        );
        self.create_workflow_transition(&reject)?;

        let obsolete = WorkflowTransition::new(
            workflow_id,
            released_state_id,
            obsolete_state_id,
            "Obsolete".to_string(),
            Some("Mark the part as obsolete".to_string()),
            true,
        );
        self.create_workflow_transition(&obsolete)?;

        let revise = WorkflowTransition::new(
            workflow_id,
            released_state_id,
            draft_state_id,
            "Revise".to_string(),
            Some("Create a new revision of the part".to_string()),
            false,
        );
        self.create_workflow_transition(&revise)?;

        Ok(workflow_id)
    }

    /// Convert a database row to a Workflow
    ///
    /// # Arguments
    ///
    /// * `row` - The database row
    ///
    /// # Returns
    ///
    /// A Workflow instance
    ///
    /// # Errors
    ///
    /// Returns a SqliteError if the row could not be converted
    fn row_to_workflow(&self, row: &Row) -> SqliteResult<Workflow> {
        Ok(Workflow {
            workflow_id: Some(row.get(0)?),
            name: row.get(1)?,
            description: row.get(2)?,
            active: row.get(3)?,
        })
    }

    /// Convert a database row to a WorkflowState
    ///
    /// # Arguments
    ///
    /// * `row` - The database row
    ///
    /// # Returns
    ///
    /// A WorkflowState instance
    ///
    /// # Errors
    ///
    /// Returns a SqliteError if the row could not be converted
    fn row_to_workflow_state(&self, row: &Row) -> SqliteResult<WorkflowState> {
        Ok(WorkflowState {
            state_id: Some(row.get(0)?),
            workflow_id: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            is_initial: row.get(4)?,
            is_terminal: row.get(5)?,
        })
    }

    /// Convert a database row to a WorkflowTransition
    ///
    /// # Arguments
    ///
    /// * `row` - The database row
    ///
    /// # Returns
    ///
    /// A WorkflowTransition instance
    ///
    /// # Errors
    ///
    /// Returns a SqliteError if the row could not be converted
    fn row_to_workflow_transition(&self, row: &Row) -> SqliteResult<WorkflowTransition> {
        Ok(WorkflowTransition {
            transition_id: Some(row.get(0)?),
            workflow_id: row.get(1)?,
            from_state_id: row.get(2)?,
            to_state_id: row.get(3)?,
            name: row.get(4)?,
            description: row.get(5)?,
            requires_approval: row.get(6)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::schema::DatabaseManager;
    use tempfile::tempdir;

    #[test]
    fn test_workflow_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let mut db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create a workflow manager
        let workflow_manager = WorkflowManager::new(db_manager.connection());

        // Create a new workflow
        let workflow = Workflow::new(
            "Test Workflow".to_string(),
            Some("A test workflow".to_string()),
            true,
        );

        // Save the workflow to the database
        let workflow_id = workflow_manager.create_workflow(&workflow).unwrap();

        // Retrieve the workflow from the database
        let retrieved_workflow = workflow_manager.get_workflow(workflow_id).unwrap();

        // Check that the retrieved workflow matches the original
        assert_eq!(retrieved_workflow.name, workflow.name);
        assert_eq!(retrieved_workflow.description, workflow.description);
        assert_eq!(retrieved_workflow.active, workflow.active);
    }

    #[test]
    fn test_workflow_state_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let mut db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create a workflow manager
        let workflow_manager = WorkflowManager::new(db_manager.connection());

        // Create a new workflow
        let workflow = Workflow::new(
            "Test Workflow".to_string(),
            Some("A test workflow".to_string()),
            true,
        );

        // Save the workflow to the database
        let workflow_id = workflow_manager.create_workflow(&workflow).unwrap();

        // Create a new workflow state
        let state = WorkflowState::new(
            workflow_id,
            "Draft".to_string(),
            Some("Initial state for new items".to_string()),
            true,
            false,
        );

        // Save the workflow state to the database
        let state_id = workflow_manager.create_workflow_state(&state).unwrap();

        // Retrieve the workflow state from the database
        let retrieved_state = workflow_manager.get_workflow_state(state_id).unwrap();

        // Check that the retrieved workflow state matches the original
        assert_eq!(retrieved_state.workflow_id, state.workflow_id);
        assert_eq!(retrieved_state.name, state.name);
        assert_eq!(retrieved_state.description, state.description);
        assert_eq!(retrieved_state.is_initial, state.is_initial);
        assert_eq!(retrieved_state.is_terminal, state.is_terminal);
    }

    #[test]
    fn test_workflow_transition_creation_and_retrieval() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let mut db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create a workflow manager
        let workflow_manager = WorkflowManager::new(db_manager.connection());

        // Create a new workflow
        let workflow = Workflow::new(
            "Test Workflow".to_string(),
            Some("A test workflow".to_string()),
            true,
        );

        // Save the workflow to the database
        let workflow_id = workflow_manager.create_workflow(&workflow).unwrap();

        // Create workflow states
        let draft_state = WorkflowState::new(
            workflow_id,
            "Draft".to_string(),
            Some("Initial state for new items".to_string()),
            true,
            false,
        );
        let draft_state_id = workflow_manager.create_workflow_state(&draft_state).unwrap();

        let review_state = WorkflowState::new(
            workflow_id,
            "In Review".to_string(),
            Some("Item is being reviewed".to_string()),
            false,
            false,
        );
        let review_state_id = workflow_manager.create_workflow_state(&review_state).unwrap();

        // Create a workflow transition
        let transition = WorkflowTransition::new(
            workflow_id,
            draft_state_id,
            review_state_id,
            "Submit for Review".to_string(),
            Some("Submit the item for review".to_string()),
            false,
        );

        // Save the workflow transition to the database
        let transition_id = workflow_manager.create_workflow_transition(&transition).unwrap();

        // Retrieve the workflow transition from the database
        let retrieved_transition = workflow_manager.get_workflow_transition(transition_id).unwrap();

        // Check that the retrieved workflow transition matches the original
        assert_eq!(retrieved_transition.workflow_id, transition.workflow_id);
        assert_eq!(retrieved_transition.from_state_id, transition.from_state_id);
        assert_eq!(retrieved_transition.to_state_id, transition.to_state_id);
        assert_eq!(retrieved_transition.name, transition.name);
        assert_eq!(retrieved_transition.description, transition.description);
        assert_eq!(retrieved_transition.requires_approval, transition.requires_approval);
    }

    #[test]
    fn test_default_part_workflow_creation() {
        // Create a temporary directory for the test database
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");

        // Create a new database manager and initialize the schema
        let mut db_manager = DatabaseManager::new(&db_path).unwrap();
        db_manager.initialize_schema().unwrap();

        // Create a workflow manager
        let workflow_manager = WorkflowManager::new(db_manager.connection());

        // Create the default part workflow
        let workflow_id = workflow_manager.create_default_part_workflow().unwrap();

        // Retrieve the workflow
        let workflow = workflow_manager.get_workflow(workflow_id).unwrap();
        assert_eq!(workflow.name, "Part Workflow");

        // Retrieve the states
        let states = workflow_manager.get_workflow_states(workflow_id).unwrap();
        assert_eq!(states.len(), 4);

        // Verify state names
        let state_names: Vec<String> = states.iter().map(|s| s.name.clone()).collect();
        assert!(state_names.contains(&"Draft".to_string()));
        assert!(state_names.contains(&"In Review".to_string()));
        assert!(state_names.contains(&"Released".to_string()));
        assert!(state_names.contains(&"Obsolete".to_string()));

        // Retrieve the transitions
        let transitions = workflow_manager.get_workflow_transitions(workflow_id).unwrap();
        assert_eq!(transitions.len(), 5);

        // Verify transition names
        let transition_names: Vec<String> = transitions.iter().map(|t| t.name.clone()).collect();
        assert!(transition_names.contains(&"Submit for Review".to_string()));
        assert!(transition_names.contains(&"Approve".to_string()));
        assert!(transition_names.contains(&"Reject".to_string()));
        assert!(transition_names.contains(&"Obsolete".to_string()));
        assert!(transition_names.contains(&"Revise".to_string()));
    }
}