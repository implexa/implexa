// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]

use tauri::{Manager};
use std::sync::Mutex;

// Import commands from the library crate
use implexa::commands::{
    // Repository commands
    repository::{
        create_repository,
        open_repository,
        close_repository,
        get_repository_info,
        init_git_backend,
        GitBackendState
    },
    
    // Part commands
    parts::{
        get_parts,
        get_part,
        create_part,
        update_part,
        change_part_status,
        delete_part,
        init_database_state,
        DatabaseState
    },
    
    // Workspace commands
    workspace::{
        get_workspaces,
        get_workspace,
        create_workspace,
        update_workspace,
        delete_workspace,
        add_part_to_workspace,
        remove_part_from_workspace,
        init_workspace_state,
        WorkspaceState
    },
    
    // Workflow commands
    workflow::{
        get_workflows,
        get_active_workflows,
        get_workflow,
        get_workflow_by_name,
        create_workflow,
        update_workflow,
        delete_workflow,
        get_workflow_states,
        get_initial_state,
        get_workflow_state,
        create_workflow_state,
        update_workflow_state,
        delete_workflow_state,
        get_workflow_transitions,
        get_transitions_from_state,
        get_workflow_transition,
        create_workflow_transition,
        update_workflow_transition,
        delete_workflow_transition,
        create_default_part_workflow,
        init_workflow_state,
        WorkflowState
    },
    
    // Approval commands
    approval::{
        get_approval,
        get_approvals_for_revision,
        get_approval_for_revision_and_approver,
        create_approval,
        update_approval_status,
        delete_approval,
        is_revision_approved,
        submit_for_approval,
        approve_revision,
        reject_revision,
        init_approval_state,
        ApprovalState
    },
    
    // Manufacturer Part commands
    manufacturer_part::{
        get_manufacturer_part,
        get_manufacturer_parts_for_part,
        get_manufacturer_parts_by_mpn,
        create_manufacturer_part,
        update_manufacturer_part,
        delete_manufacturer_part,
        search_manufacturer_parts,
        init_manufacturer_part_state,
        ManufacturerPartState
    },
    
    // Property commands
    property::{
        get_property,
        get_part_properties,
        get_revision_properties,
        get_part_property,
        get_revision_property,
        create_property,
        update_property,
        delete_property,
        init_property_state,
        PropertyState
    },
    
    // File commands
    file::{
        get_file,
        get_part_files,
        get_revision_files,
        get_part_files_by_type,
        get_revision_files_by_type,
        create_file,
        update_file,
        delete_file,
        init_file_state,
        FileState
    },
    
    // Relationship commands
    relationship::{
        get_relationship,
        get_parent_relationships,
        get_child_relationships,
        create_relationship,
        update_relationship,
        delete_relationship,
        init_relationship_state,
        RelationshipState
    },
    
    // Revision commands
    revision::{
        get_revision,
        get_part_revisions,
        get_latest_revision,
        get_latest_released_revision,
        create_revision,
        update_revision,
        update_revision_status,
        delete_revision,
        init_revision_state,
        RevisionState
    }
};

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
            
            // Create a shared connection manager for database access
            let connection_manager = init_database_state().connection_manager.clone();
            
            // Initialize the Git backend state
            app.manage(init_git_backend());
            
            // Initialize the database state for parts
            app.manage(init_database_state());
            
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
            
            // Log that the application has started
            log::info!("Implexa application started");
            
            Ok(())
        })
        // For the initial implementation, we'll just use the counter commands
        // We'll need to properly implement Tauri command registration for our
        // library crate functions later
        .invoke_handler(tauri::generate_handler![
            // Counter commands
            increment_counter,
            get_counter
        ])
        .run(context)
        .expect("Error while running Implexa application");
}