//! Command modules for the Implexa application
//!
//! This module contains all command implementations exposed to the frontend.
//! All commands are implemented in the library crate and exposed through this module.

// Re-export all command submodules
pub mod repository;
pub mod parts;
pub mod workspace;
pub mod workflow;
pub mod approval;
pub mod manufacturer_part;
pub mod property;
pub mod file;
pub mod relationship;
pub mod revision;

// Re-export common types from command modules
pub use repository::{
    GitBackendState,
    RepositoryDto,
    create_repository,
    open_repository,
    close_repository,
    get_repository_info,
    init_git_backend,
};

pub use parts::{
    DatabaseState,
    PartDto,
    get_parts,
    get_part,
    create_part,
    update_part,
    change_part_status,
    delete_part,
    init_database_state,
};

pub use workspace::{
    WorkspaceState,
    WorkspaceDto,
    get_workspaces,
    get_workspace,
    create_workspace,
    update_workspace,
    delete_workspace,
    add_part_to_workspace,
    remove_part_from_workspace,
    init_workspace_state,
};

pub use workflow::{
    WorkflowState,
    WorkflowDto,
    WorkflowStateDto,
    WorkflowTransitionDto,
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
};

pub use approval::{
    ApprovalState,
    ApprovalDto,
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
};

pub use manufacturer_part::{
    ManufacturerPartState,
    ManufacturerPartDto,
    get_manufacturer_part,
    get_manufacturer_parts_for_part,
    get_manufacturer_parts_by_mpn,
    create_manufacturer_part,
    update_manufacturer_part,
    delete_manufacturer_part,
    search_manufacturer_parts,
    init_manufacturer_part_state,
};

pub use property::{
    PropertyState,
    PropertyDto,
    get_property,
    get_part_properties,
    get_revision_properties,
    get_part_property,
    get_revision_property,
    create_property,
    update_property,
    delete_property,
    init_property_state,
};

pub use file::{
    FileState,
    FileDto,
    get_file,
    get_part_files,
    get_revision_files,
    get_part_files_by_type,
    get_revision_files_by_type,
    create_file,
    update_file,
    delete_file,
    init_file_state,
};

pub use relationship::{
    RelationshipState,
    RelationshipDto,
    get_relationship,
    get_parent_relationships,
    get_child_relationships,
    create_relationship,
    update_relationship,
    delete_relationship,
    init_relationship_state,
};

pub use revision::{
    RevisionState,
    RevisionDto,
    get_revision,
    get_part_revisions,
    get_latest_revision,
    get_latest_released_revision,
    create_revision,
    update_revision,
    update_revision_status,
    delete_revision,
    init_revision_state,
};