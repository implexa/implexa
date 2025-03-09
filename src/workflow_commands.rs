//! Tauri command handlers for workflow operations
//!
//! This module contains the command handlers for workflow-related operations in the Tauri application.
//! These commands are exposed to the frontend and allow it to interact with the workflow management system.

use std::sync::Mutex;
use tauri::{command, State};
use serde::{Serialize, Deserialize};
use implexa::database::workflow::{WorkflowManager, Workflow, WorkflowState as DbWorkflowState, WorkflowTransition};
use implexa::database::connection_manager::ConnectionManager;
use implexa::database::schema::DatabaseResult;

/// Workflow data structure for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowDto {
    /// Workflow ID
    pub workflow_id: i64,
    /// Workflow name
    pub name: String,
    /// Description of the workflow
    pub description: Option<String>,
    /// Whether this workflow is the default for parts (not in database model, for UI only)
    pub is_default: bool,
    /// Whether this workflow is active
    pub is_active: bool,
}

/// Workflow state data structure for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStateDto {
    /// State ID
    pub state_id: i64,
    /// Workflow ID this state belongs to
    pub workflow_id: i64,
    /// State name
    pub name: String,
    /// Description of the state
    pub description: Option<String>,
    /// Whether this is the initial state in the workflow
    pub is_initial: bool,
    /// Whether this is a terminal state in the workflow
    pub is_terminal: bool,
}

/// Workflow transition data structure for the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTransitionDto {
    /// Transition ID
    pub transition_id: i64,
    /// Workflow ID this transition belongs to
    pub workflow_id: i64,
    /// From state ID
    pub from_state_id: i64,
    /// To state ID
    pub to_state_id: i64,
    /// Transition name
    pub name: String,
    /// Description of the transition
    pub description: Option<String>,
    /// Whether approval is required for this transition
    pub requires_approval: bool,
}

/// Workflow creation data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowCreationData {
    /// Workflow name
    pub name: String,
    /// Description of the workflow
    pub description: Option<String>,
    /// Whether this workflow is the default for parts
    pub is_default: bool,
    /// Whether this workflow is active
    pub is_active: bool,
}

/// Workflow state creation data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStateCreationData {
    /// Workflow ID this state belongs to
    pub workflow_id: i64,
    /// State name
    pub name: String,
    /// Description of the state
    pub description: Option<String>,
    /// Whether this is the initial state in the workflow
    pub is_initial: bool,
    /// Whether this is a terminal state in the workflow
    pub is_terminal: bool,
}

/// Workflow transition creation data from the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTransitionCreationData {
    /// Workflow ID this transition belongs to
    pub workflow_id: i64,
    /// From state ID
    pub from_state_id: i64,
    /// To state ID
    pub to_state_id: i64,
    /// Transition name
    pub name: String,
    /// Description of the transition
    pub description: Option<String>,
    /// Whether approval is required for this transition
    pub requires_approval: bool,
}

/// Workflow state for workflow operations
pub struct WorkflowState {
    /// Connection manager for the database
    pub connection_manager: ConnectionManager,
    /// Workflow manager for workflow operations
    pub workflow_manager: Mutex<WorkflowManager<'static>>,
}

impl From<Workflow> for WorkflowDto {
    fn from(workflow: Workflow) -> Self {
        Self {
            workflow_id: workflow.workflow_id.unwrap_or_default(),
            name: workflow.name,
            description: workflow.description,
            is_default: false, // Not in Workflow struct, default to false
            is_active: workflow.active,
        }
    }
}

impl From<DbWorkflowState> for WorkflowStateDto {
    fn from(state: DbWorkflowState) -> Self {
        Self {
            state_id: state.state_id.unwrap_or_default(),
            workflow_id: state.workflow_id,
            name: state.name,
            description: state.description,
            is_initial: state.is_initial,
            is_terminal: state.is_terminal,
        }
    }
}

impl From<WorkflowTransition> for WorkflowTransitionDto {
    fn from(transition: WorkflowTransition) -> Self {
        Self {
            transition_id: transition.transition_id.unwrap_or_default(),
            workflow_id: transition.workflow_id,
            from_state_id: transition.from_state_id,
            to_state_id: transition.to_state_id,
            name: transition.name,
            description: transition.description,
            requires_approval: transition.requires_approval,
        }
    }
}

/// Initialize the workflow state
pub fn init_workflow_state(connection_manager: ConnectionManager) -> WorkflowState {
    // Create a workflow manager with 'static lifetime using a leak (safe in this context)
    let static_connection_manager: &'static ConnectionManager = Box::leak(Box::new(connection_manager.clone()));
    let workflow_manager = WorkflowManager::new(static_connection_manager);
    
    WorkflowState {
        connection_manager,
        workflow_manager: Mutex::new(workflow_manager),
    }
}

/// Get all workflows
#[command]
pub async fn get_workflows(
    workflow_state: State<'_, WorkflowState>,
) -> Result<Vec<WorkflowDto>, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all workflows
    let workflows = workflow_manager.get_all_workflows()
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let workflow_dtos = workflows.into_iter()
        .map(WorkflowDto::from)
        .collect();
    
    Ok(workflow_dtos)
}

/// Get active workflows
#[command]
pub async fn get_active_workflows(
    workflow_state: State<'_, WorkflowState>,
) -> Result<Vec<WorkflowDto>, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Get active workflows
    let workflows = workflow_manager.get_active_workflows()
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let workflow_dtos = workflows.into_iter()
        .map(WorkflowDto::from)
        .collect();
    
    Ok(workflow_dtos)
}

/// Get a specific workflow by ID
#[command]
pub async fn get_workflow(
    workflow_id: i64,
    workflow_state: State<'_, WorkflowState>,
) -> Result<WorkflowDto, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the workflow
    let workflow = workflow_manager.get_workflow(workflow_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(WorkflowDto::from(workflow))
}

/// Get a workflow by name
#[command]
pub async fn get_workflow_by_name(
    name: String,
    workflow_state: State<'_, WorkflowState>,
) -> Result<WorkflowDto, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the workflow
    let workflow = workflow_manager.get_workflow_by_name(&name)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(WorkflowDto::from(workflow))
}

/// Create a new workflow
#[command]
pub async fn create_workflow(
    workflow_data: WorkflowCreationData,
    workflow_state: State<'_, WorkflowState>,
) -> Result<WorkflowDto, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Create a new workflow
    let workflow = Workflow {
        workflow_id: None,
        name: workflow_data.name,
        description: workflow_data.description,
        active: workflow_data.is_active,
    };
    
    // Save the workflow
    let workflow_id = workflow_manager.create_workflow(&workflow)
        .map_err(|e| e.to_string())?;
    
    // Get the workflow with the ID
    let created_workflow = workflow_manager.get_workflow(workflow_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(WorkflowDto::from(created_workflow))
}

/// Update an existing workflow
#[command]
pub async fn update_workflow(
    workflow_id: i64,
    workflow_data: WorkflowCreationData,
    workflow_state: State<'_, WorkflowState>,
) -> Result<WorkflowDto, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Create an updated workflow
    let workflow = Workflow {
        workflow_id: Some(workflow_id),
        name: workflow_data.name,
        description: workflow_data.description,
        active: workflow_data.is_active,
    };
    
    // Update the workflow
    workflow_manager.update_workflow(&workflow)
        .map_err(|e| e.to_string())?;
    
    // Get the updated workflow
    let updated_workflow = workflow_manager.get_workflow(workflow_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(WorkflowDto::from(updated_workflow))
}

/// Delete a workflow
#[command]
pub async fn delete_workflow(
    workflow_id: i64,
    workflow_state: State<'_, WorkflowState>,
) -> Result<(), String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Delete the workflow
    workflow_manager.delete_workflow(workflow_id)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Get all states for a workflow
#[command]
pub async fn get_workflow_states(
    workflow_id: i64,
    workflow_state: State<'_, WorkflowState>,
) -> Result<Vec<WorkflowStateDto>, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all states for the workflow
    let states = workflow_manager.get_workflow_states(workflow_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let state_dtos = states.into_iter()
        .map(WorkflowStateDto::from)
        .collect();
    
    Ok(state_dtos)
}

/// Get the initial state for a workflow
#[command]
pub async fn get_initial_state(
    workflow_id: i64,
    workflow_state: State<'_, WorkflowState>,
) -> Result<WorkflowStateDto, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the initial state
    let state = workflow_manager.get_initial_state(workflow_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(WorkflowStateDto::from(state))
}

/// Get a specific workflow state
#[command]
pub async fn get_workflow_state(
    state_id: i64,
    workflow_state: State<'_, WorkflowState>,
) -> Result<WorkflowStateDto, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the state
    let state = workflow_manager.get_workflow_state(state_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(WorkflowStateDto::from(state))
}

/// Create a new workflow state
#[command]
pub async fn create_workflow_state(
    state_data: WorkflowStateCreationData,
    workflow_state: State<'_, WorkflowState>,
) -> Result<WorkflowStateDto, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Create a new state
    let state = DbWorkflowState {
        state_id: None,
        workflow_id: state_data.workflow_id,
        name: state_data.name,
        description: state_data.description,
        is_initial: state_data.is_initial,
        is_terminal: state_data.is_terminal,
    };
    
    // Save the state
    let state_id = workflow_manager.create_workflow_state(&state)
        .map_err(|e| e.to_string())?;
    
    // Get the state with the ID
    let created_state = workflow_manager.get_workflow_state(state_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(WorkflowStateDto::from(created_state))
}

/// Update an existing workflow state
#[command]
pub async fn update_workflow_state(
    state_id: i64,
    state_data: WorkflowStateCreationData,
    workflow_state: State<'_, WorkflowState>,
) -> Result<WorkflowStateDto, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Create an updated state
    let state = DbWorkflowState {
        state_id: Some(state_id),
        workflow_id: state_data.workflow_id,
        name: state_data.name,
        description: state_data.description,
        is_initial: state_data.is_initial,
        is_terminal: state_data.is_terminal,
    };
    
    // Update the state
    workflow_manager.update_workflow_state(&state)
        .map_err(|e| e.to_string())?;
    
    // Get the updated state
    let updated_state = workflow_manager.get_workflow_state(state_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(WorkflowStateDto::from(updated_state))
}

/// Delete a workflow state
#[command]
pub async fn delete_workflow_state(
    state_id: i64,
    workflow_state: State<'_, WorkflowState>,
) -> Result<(), String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Delete the state
    workflow_manager.delete_workflow_state(state_id)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Get all transitions for a workflow
#[command]
pub async fn get_workflow_transitions(
    workflow_id: i64,
    workflow_state: State<'_, WorkflowState>,
) -> Result<Vec<WorkflowTransitionDto>, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all transitions for the workflow
    let transitions = workflow_manager.get_workflow_transitions(workflow_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let transition_dtos = transitions.into_iter()
        .map(WorkflowTransitionDto::from)
        .collect();
    
    Ok(transition_dtos)
}

/// Get transitions from a specific state
#[command]
pub async fn get_transitions_from_state(
    from_state_id: i64,
    workflow_state: State<'_, WorkflowState>,
) -> Result<Vec<WorkflowTransitionDto>, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Get all transitions from the state
    let transitions = workflow_manager.get_transitions_from_state(from_state_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTOs
    let transition_dtos = transitions.into_iter()
        .map(WorkflowTransitionDto::from)
        .collect();
    
    Ok(transition_dtos)
}

/// Get a specific workflow transition
#[command]
pub async fn get_workflow_transition(
    transition_id: i64,
    workflow_state: State<'_, WorkflowState>,
) -> Result<WorkflowTransitionDto, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Get the transition
    let transition = workflow_manager.get_workflow_transition(transition_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(WorkflowTransitionDto::from(transition))
}

/// Create a new workflow transition
#[command]
pub async fn create_workflow_transition(
    transition_data: WorkflowTransitionCreationData,
    workflow_state: State<'_, WorkflowState>,
) -> Result<WorkflowTransitionDto, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Create a new transition
    let transition = WorkflowTransition {
        transition_id: None,
        workflow_id: transition_data.workflow_id,
        from_state_id: transition_data.from_state_id,
        to_state_id: transition_data.to_state_id,
        name: transition_data.name,
        description: transition_data.description,
        requires_approval: transition_data.requires_approval,
    };
    
    // Save the transition
    let transition_id = workflow_manager.create_workflow_transition(&transition)
        .map_err(|e| e.to_string())?;
    
    // Get the transition with the ID
    let created_transition = workflow_manager.get_workflow_transition(transition_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(WorkflowTransitionDto::from(created_transition))
}

/// Update an existing workflow transition
#[command]
pub async fn update_workflow_transition(
    transition_id: i64,
    transition_data: WorkflowTransitionCreationData,
    workflow_state: State<'_, WorkflowState>,
) -> Result<WorkflowTransitionDto, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Create an updated transition
    let transition = WorkflowTransition {
        transition_id: Some(transition_id),
        workflow_id: transition_data.workflow_id,
        from_state_id: transition_data.from_state_id,
        to_state_id: transition_data.to_state_id,
        name: transition_data.name,
        description: transition_data.description,
        requires_approval: transition_data.requires_approval,
    };
    
    // Update the transition
    workflow_manager.update_workflow_transition(&transition)
        .map_err(|e| e.to_string())?;
    
    // Get the updated transition
    let updated_transition = workflow_manager.get_workflow_transition(transition_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(WorkflowTransitionDto::from(updated_transition))
}

/// Delete a workflow transition
#[command]
pub async fn delete_workflow_transition(
    transition_id: i64,
    workflow_state: State<'_, WorkflowState>,
) -> Result<(), String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Delete the transition
    workflow_manager.delete_workflow_transition(transition_id)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Create a default part workflow
#[command]
pub async fn create_default_part_workflow(
    workflow_state: State<'_, WorkflowState>,
) -> Result<WorkflowDto, String> {
    let workflow_manager = workflow_state.workflow_manager.lock().map_err(|e| e.to_string())?;
    
    // Create the default part workflow
    let workflow_id = workflow_manager.create_default_part_workflow()
        .map_err(|e| e.to_string())?;
    
    // Get the created workflow
    let workflow = workflow_manager.get_workflow(workflow_id)
        .map_err(|e| e.to_string())?;
    
    // Convert to DTO
    Ok(WorkflowDto::from(workflow))
}