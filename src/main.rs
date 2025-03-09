// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
use tauri::{Manager};
use std::sync::Mutex;

// Direct imports for modules used in wrapper functions
use implexa::commands::repository;
use implexa::commands::parts;
use implexa::commands::workspace;
use implexa::commands::workflow;
use implexa::commands::revision;
use implexa::commands::relationship;
use implexa::commands::approval;
use implexa::commands::file;
use implexa::commands::manufacturer_part;
use implexa::commands::property;


// Import only the necessary state and initialization functions from the library crate
use implexa::commands::repository::GitBackendState;
use implexa::commands::repository::init_git_backend;
use implexa::commands::parts::DatabaseState;
use implexa::commands::parts::init_database_state;
use implexa::commands::workspace::WorkspaceState;
use implexa::commands::workspace::init_workspace_state;
use implexa::commands::workflow::WorkflowState;
use implexa::commands::workflow::init_workflow_state;
use implexa::commands::approval::ApprovalState;
use implexa::commands::approval::init_approval_state;
use implexa::commands::manufacturer_part::ManufacturerPartState;
use implexa::commands::manufacturer_part::init_manufacturer_part_state;
use implexa::commands::property::PropertyState;
use implexa::commands::property::init_property_state;
use implexa::commands::file::FileState;
use implexa::commands::file::init_file_state;
use implexa::commands::relationship::RelationshipState;
use implexa::commands::relationship::init_relationship_state;
use implexa::commands::revision::RevisionState;
use implexa::commands::revision::init_revision_state;
// Define a simple state struct for our application
struct AppState {
    counter: Mutex<i32>,
}

// Define a command to increment the counter
#[tauri::command]
fn increment_counter(state: tauri::State<AppState>) -> Result<i32, String> {
    let mut counter = state.counter.lock().map_err(|_| "Failed to lock counter".to_string())?;
    *counter += 1;
    Ok(*counter)
}

// Define a command to get the current counter value
#[tauri::command]
fn get_counter(state: tauri::State<AppState>) -> Result<i32, String> {
    let counter = state.counter.lock().map_err(|_| "Failed to lock counter".to_string())?;
    Ok(*counter)
}

// Repository command wrappers
#[tauri::command]
async fn create_repository(
    path: String,
    template_type: String,
    git_state: tauri::State<'_, GitBackendState>,
    db_state: tauri::State<'_, DatabaseState>,
) -> Result<repository::RepositoryDto, String> {
    repository::create_repository(path, template_type, git_state, db_state).await
}

#[tauri::command]
async fn open_repository(
    path: String,
    git_state: tauri::State<'_, GitBackendState>,
    db_state: tauri::State<'_, DatabaseState>,
) -> Result<repository::RepositoryDto, String> {
    repository::open_repository(path, git_state, db_state).await
}

#[tauri::command]
async fn close_repository(
    path: String,
    git_state: tauri::State<'_, GitBackendState>,
) -> Result<(), String> {
    repository::close_repository(path, git_state).await
}

#[tauri::command]
async fn get_repository_info(
    path: String,
    git_state: tauri::State<'_, GitBackendState>,
    db_state: tauri::State<'_, DatabaseState>,
) -> Result<repository::RepositoryDto, String> {
    repository::get_repository_info(path, git_state, db_state).await
}

// Part command wrappers
#[tauri::command]
async fn get_parts(
    db_state: tauri::State<'_, DatabaseState>,
) -> Result<Vec<parts::PartDto>, String> {
    parts::get_parts(db_state).await
}

#[tauri::command]
async fn get_part(
    part_id: i64,
    db_state: tauri::State<'_, DatabaseState>,
) -> Result<parts::PartDto, String> {
    parts::get_part(part_id, db_state).await
}

#[tauri::command]
async fn create_part(
    part_data: parts::PartCreateDto,
    db_state: tauri::State<'_, DatabaseState>,
) -> Result<parts::PartDto, String> {
    parts::create_part(part_data, db_state).await
}

#[tauri::command]
async fn update_part(
    part_id: i64,
    part_data: parts::PartCreateDto,
    db_state: tauri::State<'_, DatabaseState>,
) -> Result<parts::PartDto, String> {
    parts::update_part(part_id, part_data, db_state).await
}

#[tauri::command]
async fn change_part_status(
    part_id: i64,
    status: String,
    db_state: tauri::State<'_, DatabaseState>,
) -> Result<(), String> {
    parts::change_part_status(part_id, status, db_state).await
}

#[tauri::command]
async fn delete_part(
    part_id: i64,
    db_state: tauri::State<'_, DatabaseState>,
) -> Result<(), String> {
    parts::delete_part(part_id, db_state).await
}

// Workspace command wrappers
#[tauri::command]
async fn get_workspaces(
    workspace_state: tauri::State<'_, WorkspaceState>,
) -> Result<Vec<workspace::WorkspaceDto>, String> {
    workspace::get_workspaces(workspace_state).await
}

#[tauri::command]
async fn get_workspace(
    workspace_id: String,
    workspace_state: tauri::State<'_, WorkspaceState>,
) -> Result<workspace::WorkspaceDto, String> {
    workspace::get_workspace(workspace_id, workspace_state).await
}

#[tauri::command]
async fn create_workspace(
    workspace_data: workspace::WorkspaceCreationData,
    workspace_state: tauri::State<'_, WorkspaceState>,
) -> Result<workspace::WorkspaceDto, String> {
    workspace::create_workspace(workspace_data, workspace_state).await
}

#[tauri::command]
async fn update_workspace(
    workspace_id: String,
    workspace_data: workspace::WorkspaceUpdateData,
    workspace_state: tauri::State<'_, WorkspaceState>,
) -> Result<workspace::WorkspaceDto, String> {
    workspace::update_workspace(workspace_id, workspace_data, workspace_state).await
}

#[tauri::command]
async fn delete_workspace(
    workspace_id: String,
    workspace_state: tauri::State<'_, WorkspaceState>,
) -> Result<(), String> {
    workspace::delete_workspace(workspace_id, workspace_state).await
}

#[tauri::command]
async fn add_part_to_workspace(
    workspace_id: String,
    part_id: i64,
    workspace_state: tauri::State<'_, WorkspaceState>,
) -> Result<(), String> {
    workspace::add_part_to_workspace(workspace_id, part_id, workspace_state).await
}

#[tauri::command]
async fn remove_part_from_workspace(
    workspace_id: String,
    part_id: i64,
    workspace_state: tauri::State<'_, WorkspaceState>,
) -> Result<(), String> {
    workspace::remove_part_from_workspace(workspace_id, part_id, workspace_state).await
}

// Workflow command wrappers
#[tauri::command]
async fn get_workflows(
    workflow_state: tauri::State<'_, WorkflowState>,
) -> Result<Vec<workflow::WorkflowDto>, String> {
    workflow::get_workflows(workflow_state).await
}

#[tauri::command]
async fn get_active_workflows(
    workflow_state: tauri::State<'_, WorkflowState>,
) -> Result<Vec<workflow::WorkflowDto>, String> {
    workflow::get_active_workflows(workflow_state).await
}

#[tauri::command]
async fn get_workflow(
    workflow_id: i64,
    workflow_state: tauri::State<'_, WorkflowState>,
) -> Result<workflow::WorkflowDto, String> {
    workflow::get_workflow(workflow_id, workflow_state).await
}

#[tauri::command]
async fn get_workflow_by_name(
    name: String,
    workflow_state: tauri::State<'_, WorkflowState>,
) -> Result<workflow::WorkflowDto, String> {
    workflow::get_workflow_by_name(name, workflow_state).await
}

#[tauri::command]
async fn create_workflow(
    workflow_data: workflow::WorkflowCreationData,
    workflow_state: tauri::State<'_, WorkflowState>,
) -> Result<workflow::WorkflowDto, String> {
    workflow::create_workflow(workflow_data, workflow_state).await
}

#[tauri::command]
async fn update_workflow(
    workflow_id: i64,
    workflow_data: workflow::WorkflowCreationData,
    workflow_state: tauri::State<'_, WorkflowState>,
) -> Result<workflow::WorkflowDto, String> {
    workflow::update_workflow(workflow_id, workflow_data, workflow_state).await
}

#[tauri::command]
async fn delete_workflow(
    workflow_id: i64,
    workflow_state: tauri::State<'_, WorkflowState>,
) -> Result<(), String> {
    workflow::delete_workflow(workflow_id, workflow_state).await
}

#[tauri::command]
async fn get_workflow_states(
    workflow_id: i64,
    workflow_state: tauri::State<'_, WorkflowState>,
) -> Result<Vec<workflow::WorkflowStateDto>, String> {
    workflow::get_workflow_states(workflow_id, workflow_state).await
}

#[tauri::command]
async fn get_initial_state(
    workflow_id: i64,
    workflow_state: tauri::State<'_, WorkflowState>,
) -> Result<workflow::WorkflowStateDto, String> {
    workflow::get_initial_state(workflow_id, workflow_state).await
}

#[tauri::command]
async fn create_default_part_workflow(
    workflow_state: tauri::State<'_, WorkflowState>,
) -> Result<workflow::WorkflowDto, String> {
    workflow::create_default_part_workflow(workflow_state).await
}

// Revision command wrappers
#[tauri::command]
async fn get_revision(
    revision_id: i64,
    revision_state: tauri::State<'_, RevisionState>,
) -> Result<revision::RevisionDto, String> {
    revision::get_revision(revision_id, revision_state).await
}

#[tauri::command]
async fn get_part_revisions(
    part_id: i64,
    revision_state: tauri::State<'_, RevisionState>,
) -> Result<Vec<revision::RevisionDto>, String> {
    revision::get_part_revisions(part_id, revision_state).await
}

#[tauri::command]
async fn get_latest_revision(
    part_id: i64,
    revision_state: tauri::State<'_, RevisionState>,
) -> Result<revision::RevisionDto, String> {
    revision::get_latest_revision(part_id, revision_state).await
}

#[tauri::command]
async fn get_latest_released_revision(
    part_id: i64,
    revision_state: tauri::State<'_, RevisionState>,
) -> Result<revision::RevisionDto, String> {
    revision::get_latest_released_revision(part_id, revision_state).await
}

#[tauri::command]
async fn create_revision(
    revision_data: revision::RevisionCreationData,
    revision_state: tauri::State<'_, RevisionState>,
) -> Result<revision::RevisionDto, String> {
    revision::create_revision(revision_data, revision_state).await
}

#[tauri::command]
async fn update_revision(
    revision_id: i64,
    revision_data: revision::RevisionCreationData,
    revision_state: tauri::State<'_, RevisionState>,
) -> Result<revision::RevisionDto, String> {
    revision::update_revision(revision_id, revision_data, revision_state).await
}

#[tauri::command]
async fn update_revision_status(
    revision_id: i64,
    status: String,
    revision_state: tauri::State<'_, RevisionState>,
) -> Result<revision::RevisionDto, String> {
    revision::update_revision_status(revision_id, status, revision_state).await
}

#[tauri::command]
async fn delete_revision(
    revision_id: i64,
    revision_state: tauri::State<'_, RevisionState>,
) -> Result<(), String> {
    revision::delete_revision(revision_id, revision_state).await
}

// Relationship command wrappers
#[tauri::command]
async fn get_relationship(
    relationship_id: i64,
    relationship_state: tauri::State<'_, RelationshipState>,
) -> Result<relationship::RelationshipDto, String> {
    relationship::get_relationship(relationship_id, relationship_state).await
}

#[tauri::command]
async fn get_parent_relationships(
    part_id: i64,
    relationship_state: tauri::State<'_, RelationshipState>,
) -> Result<Vec<relationship::RelationshipDto>, String> {
    relationship::get_parent_relationships(part_id, relationship_state).await
}

#[tauri::command]
async fn get_child_relationships(
    part_id: i64,
    relationship_state: tauri::State<'_, RelationshipState>,
) -> Result<Vec<relationship::RelationshipDto>, String> {
    relationship::get_child_relationships(part_id, relationship_state).await
}

#[tauri::command]
async fn create_relationship(
    relationship_data: relationship::RelationshipCreationData,
    relationship_state: tauri::State<'_, RelationshipState>,
) -> Result<relationship::RelationshipDto, String> {
    relationship::create_relationship(relationship_data, relationship_state).await
}

#[tauri::command]
async fn update_relationship(
    relationship_id: i64,
    relationship_data: relationship::RelationshipCreationData,
    relationship_state: tauri::State<'_, RelationshipState>,
) -> Result<relationship::RelationshipDto, String> {
    relationship::update_relationship(relationship_id, relationship_data, relationship_state).await
}

#[tauri::command]
async fn delete_relationship(
    relationship_id: i64,
    relationship_state: tauri::State<'_, RelationshipState>,
) -> Result<(), String> {
    relationship::delete_relationship(relationship_id, relationship_state).await
}

// Approval command wrappers
#[tauri::command]
async fn get_approval(
    approval_id: i64,
    approval_state: tauri::State<'_, ApprovalState>,
) -> Result<approval::ApprovalDto, String> {
    approval::get_approval(approval_id, approval_state).await
}

#[tauri::command]
async fn get_approvals_for_revision(
    revision_id: i64,
    approval_state: tauri::State<'_, ApprovalState>,
) -> Result<Vec<approval::ApprovalDto>, String> {
    approval::get_approvals_for_revision(revision_id, approval_state).await
}

#[tauri::command]
async fn get_approval_for_revision_and_approver(
    revision_id: i64,
    approver: String,
    approval_state: tauri::State<'_, ApprovalState>,
) -> Result<approval::ApprovalDto, String> {
    approval::get_approval_for_revision_and_approver(revision_id, approver, approval_state).await
}

#[tauri::command]
async fn create_approval(
    approval_data: approval::ApprovalCreationData,
    approval_state: tauri::State<'_, ApprovalState>,
) -> Result<approval::ApprovalDto, String> {
    approval::create_approval(approval_data, approval_state).await
}

#[tauri::command]
async fn update_approval_status(
    approval_id: i64,
    update_data: approval::ApprovalUpdateData,
    approval_state: tauri::State<'_, ApprovalState>,
) -> Result<approval::ApprovalDto, String> {
    approval::update_approval_status(approval_id, update_data, approval_state).await
}

#[tauri::command]
async fn approve_revision(
    revision_id: i64,
    approver: String,
    comments: Option<String>,
    approval_state: tauri::State<'_, ApprovalState>,
) -> Result<approval::ApprovalDto, String> {
    approval::approve_revision(revision_id, approver, comments, approval_state).await
}

#[tauri::command]
async fn reject_revision(
    revision_id: i64,
    rejecter: String,
    comments: Option<String>,
    approval_state: tauri::State<'_, ApprovalState>,
) -> Result<approval::ApprovalDto, String> {
    approval::reject_revision(revision_id, rejecter, comments, approval_state).await
}

#[tauri::command]
async fn delete_approval(
    approval_id: i64,
    approval_state: tauri::State<'_, ApprovalState>,
) -> Result<(), String> {
    approval::delete_approval(approval_id, approval_state).await
}

#[tauri::command]
async fn is_revision_approved(
    revision_id: i64,
    approval_state: tauri::State<'_, ApprovalState>,
) -> Result<bool, String> {
    approval::is_revision_approved(revision_id, approval_state).await
}

#[tauri::command]
async fn submit_for_approval(
    revision_id: i64,
    approvers: Vec<String>,
    approval_state: tauri::State<'_, ApprovalState>,
) -> Result<Vec<approval::ApprovalDto>, String> {
    approval::submit_for_approval(revision_id, approvers, approval_state).await
}

// Manufacturer Part command wrappers
#[tauri::command]
async fn get_manufacturer_part(
    mpn_id: i64,
    manufacturer_part_state: tauri::State<'_, ManufacturerPartState>,
) -> Result<manufacturer_part::ManufacturerPartDto, String> {
    manufacturer_part::get_manufacturer_part(mpn_id, manufacturer_part_state).await
}

#[tauri::command]
async fn get_manufacturer_parts_for_part(
    part_id: String,
    manufacturer_part_state: tauri::State<'_, ManufacturerPartState>,
) -> Result<Vec<manufacturer_part::ManufacturerPartDto>, String> {
    manufacturer_part::get_manufacturer_parts_for_part(part_id, manufacturer_part_state).await
}

#[tauri::command]
async fn get_manufacturer_parts_by_mpn(
    manufacturer: String,
    mpn: String,
    manufacturer_part_state: tauri::State<'_, ManufacturerPartState>,
) -> Result<Vec<manufacturer_part::ManufacturerPartDto>, String> {
    manufacturer_part::get_manufacturer_parts_by_mpn(manufacturer, mpn, manufacturer_part_state).await
}

#[tauri::command]
async fn create_manufacturer_part(
    manufacturer_part_data: manufacturer_part::ManufacturerPartCreationData,
    manufacturer_part_state: tauri::State<'_, ManufacturerPartState>,
) -> Result<manufacturer_part::ManufacturerPartDto, String> {
    manufacturer_part::create_manufacturer_part(manufacturer_part_data, manufacturer_part_state).await
}

#[tauri::command]
async fn update_manufacturer_part(
    mpn_id: i64,
    manufacturer_part_data: manufacturer_part::ManufacturerPartCreationData,
    manufacturer_part_state: tauri::State<'_, ManufacturerPartState>,
) -> Result<manufacturer_part::ManufacturerPartDto, String> {
    manufacturer_part::update_manufacturer_part(mpn_id, manufacturer_part_data, manufacturer_part_state).await
}

#[tauri::command]
async fn delete_manufacturer_part(
    mpn_id: i64,
    manufacturer_part_state: tauri::State<'_, ManufacturerPartState>,
) -> Result<(), String> {
    manufacturer_part::delete_manufacturer_part(mpn_id, manufacturer_part_state).await
}

#[tauri::command]
async fn search_manufacturer_parts(
    search_term: String,
    manufacturer_part_state: tauri::State<'_, ManufacturerPartState>,
) -> Result<Vec<manufacturer_part::ManufacturerPartDto>, String> {
    manufacturer_part::search_manufacturer_parts(search_term, manufacturer_part_state).await
}

// Property command wrappers
#[tauri::command]
async fn get_part_properties(
    part_id: String,
    property_state: tauri::State<'_, PropertyState>,
) -> Result<Vec<property::PropertyDto>, String> {
    property::get_part_properties(part_id, property_state).await
}

#[tauri::command]
async fn get_revision_properties(
    revision_id: i64,
    property_state: tauri::State<'_, PropertyState>,
) -> Result<Vec<property::PropertyDto>, String> {
    property::get_revision_properties(revision_id, property_state).await
}

#[tauri::command]
async fn get_part_property(
    part_id: String,
    key: String,
    property_state: tauri::State<'_, PropertyState>,
) -> Result<property::PropertyDto, String> {
    property::get_part_property(part_id, key, property_state).await
}

#[tauri::command]
async fn get_revision_property(
    revision_id: i64,
    key: String,
    property_state: tauri::State<'_, PropertyState>,
) -> Result<property::PropertyDto, String> {
    property::get_revision_property(revision_id, key, property_state).await
}

#[tauri::command]
async fn get_property(
    property_id: i64,
    property_state: tauri::State<'_, PropertyState>,
) -> Result<property::PropertyDto, String> {
    property::get_property(property_id, property_state).await
}

#[tauri::command]
async fn create_property(
    property_data: property::PropertyCreationData,
    property_state: tauri::State<'_, PropertyState>,
) -> Result<property::PropertyDto, String> {
    property::create_property(property_data, property_state).await
}

#[tauri::command]
async fn update_property(
    property_id: i64,
    property_data: property::PropertyCreationData,
    property_state: tauri::State<'_, PropertyState>,
) -> Result<property::PropertyDto, String> {
    property::update_property(property_id, property_data, property_state).await
}

#[tauri::command]
async fn delete_property(
    property_id: i64,
    property_state: tauri::State<'_, PropertyState>,
) -> Result<(), String> {
    property::delete_property(property_id, property_state).await
}

// File command wrappers
#[tauri::command]
async fn get_file(
    file_id: i64,
    file_state: tauri::State<'_, FileState>,
) -> Result<file::FileDto, String> {
    file::get_file(file_id, file_state).await
}

#[tauri::command]
async fn get_part_files(
    part_id: String,
    file_state: tauri::State<'_, FileState>,
) -> Result<Vec<file::FileDto>, String> {
    file::get_part_files(part_id, file_state).await
}

#[tauri::command]
async fn get_revision_files(
    revision_id: i64,
    file_state: tauri::State<'_, FileState>,
) -> Result<Vec<file::FileDto>, String> {
    file::get_revision_files(revision_id, file_state).await
}

#[tauri::command]
async fn get_part_files_by_type(
    part_id: String,
    file_type: String,
    file_state: tauri::State<'_, FileState>,
) -> Result<Vec<file::FileDto>, String> {
    file::get_part_files_by_type(part_id, file_type, file_state).await
}

#[tauri::command]
async fn get_revision_files_by_type(
    revision_id: i64,
    file_type: String,
    file_state: tauri::State<'_, FileState>,
) -> Result<Vec<file::FileDto>, String> {
    file::get_revision_files_by_type(revision_id, file_type, file_state).await
}

#[tauri::command]
async fn create_file(
    file_data: file::FileCreationData,
    file_state: tauri::State<'_, FileState>,
) -> Result<file::FileDto, String> {
    file::create_file(file_data, file_state).await
}

#[tauri::command]
async fn update_file(
    file_id: i64,
    file_data: file::FileCreationData,
    file_state: tauri::State<'_, FileState>,
) -> Result<file::FileDto, String> {
    file::update_file(file_id, file_data, file_state).await
}

#[tauri::command]
async fn delete_file(
    file_id: i64,
    file_state: tauri::State<'_, FileState>,
) -> Result<(), String> {
    file::delete_file(file_id, file_state).await
}


fn main() {
    // Initialize logging
    env_logger::init();

    // Build the Tauri application
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            // Initialize the application state
            app.manage(AppState {
                counter: Mutex::new(0),
            });
            
            // Log that we're initializing application states
            println!("Initializing application states...");
            
            // Create a shared connection manager for database access
            let db_state = init_database_state();
            let connection_manager = db_state.connection_manager.clone();
            
            // Initialize the Git backend state
            app.manage(init_git_backend());
            
            // Initialize the database state for parts
            app.manage(db_state);
            
            // Initialize the workspace state
            app.manage(init_workspace_state());
            
            // Initialize the workflow state
            app.manage(init_workflow_state(connection_manager.clone()));
            
            // Initialize the approval state
            app.manage(init_approval_state(connection_manager.clone()));
            
            // Initialize the manufacturer part state
            app.manage(init_manufacturer_part_state(connection_manager.clone()));
            
            // Initialize the property state
            app.manage(init_property_state(connection_manager.clone()));
            
            // Initialize the file state
            app.manage(init_file_state(connection_manager.clone()));
            
            // Initialize the relationship state
            app.manage(init_relationship_state(connection_manager.clone()));
            
            // Initialize the revision state
            app.manage(init_revision_state(connection_manager.clone()));
            
            println!("Application states initialized successfully");
            
            // Log that the application has started
            log::info!("Implexa application started");
            
            Ok(())
        })
        // Register our wrapper commands to expose the library functionality
        .invoke_handler(tauri::generate_handler![
            // Counter commands (for demonstration)
            increment_counter,
            get_counter,
            
            // Repository commands
            create_repository,
            open_repository,
            close_repository,
            get_repository_info,
            
            // Part commands
            get_parts,
            get_part,
            create_part,
            update_part,
            change_part_status,
            delete_part,
            
            // Workspace commands
            get_workspaces,
            get_workspace,
            create_workspace,
            update_workspace,
            delete_workspace,
            add_part_to_workspace,
            remove_part_from_workspace,
            
            // Workflow commands
            get_workflows,
            get_active_workflows,
            get_workflow,
            get_workflow_by_name,
            create_workflow,
            update_workflow,
            delete_workflow,
            get_workflow_states,
            get_initial_state,
            create_default_part_workflow,
            
            // Revision commands
            get_revision,
            get_part_revisions,
            get_latest_revision,
            get_latest_released_revision,
            create_revision,
            update_revision,
            update_revision_status,
            delete_revision,
            
            // Relationship commands
            get_relationship,
            get_parent_relationships,
            get_child_relationships,
            create_relationship,
            update_relationship,
            delete_relationship,
            // Approval commands
            get_approval,
            get_approvals_for_revision,
            get_approval_for_revision_and_approver,
            create_approval,
            update_approval_status,
            approve_revision,
            reject_revision,
            delete_approval,
            is_revision_approved,
            submit_for_approval,
            
            // Manufacturer Part commands
            get_manufacturer_part,
            get_manufacturer_parts_for_part,
            get_manufacturer_parts_by_mpn,
            create_manufacturer_part,
            update_manufacturer_part,
            delete_manufacturer_part,
            search_manufacturer_parts,
            
            // Property commands
            get_property,
            get_part_properties,
            get_revision_properties,
            get_part_property,
            get_revision_property,
            create_property,
            update_property,
            delete_property,
            
            // File commands
            get_file,
            get_part_files,
            get_revision_files,
            get_part_files_by_type,
            get_revision_files_by_type,
            create_file,
            update_file,
            delete_file
        ])
        .run(context)
        .expect("Error while running Implexa application");
}