# Part Management Workflow

## Overview

The Part Management Workflow defines how parts are created, modified, reviewed, approved, and released within Implexa. This document outlines the workflow architecture, including states, transitions, approvals, and integration with other components, with a focus on simplicity and usability for small teams.

## Core Principles

1. **Traceability**: Every change to a part must be traceable to a specific user and point in time
2. **Simplicity**: The workflow should be intuitive and not add unnecessary complexity
3. **Flexibility**: Support different working styles while maintaining data integrity
4. **Integration**: Seamless integration with Git, CAD tools, and the database
5. **Collaboration**: Enable small teams to work effectively together

## Part Lifecycle

A part in Implexa goes through the following high-level lifecycle:

```
┌─────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│  Draft  │────►│ In Review│────►│ Released │────►│ Obsolete │
└─────────┘     └──────────┘     └──────────┘     └──────────┘
     ▲                │              │
     │                │              │
     └────────────────┘              │
          (Rejected)                 │
                                     │
                                     ▼
                               ┌──────────┐
                               │ Revision │
                               └──────────┘
```

### Part States

1. **Draft**: Initial state for a new part or revision
   - Part is being designed and defined
   - Changes can be made freely
   - Stored in a feature branch in Git

2. **In Review**: Part is ready for review
   - Changes are frozen pending review
   - Team members can comment and approve/reject
   - Stored in a review branch in Git

3. **Released**: Part is approved and released
   - Changes are locked
   - Part is visible to all users
   - Can be used in other assemblies
   - Stored in the main branch in Git

4. **Revision**: Part is being revised
   - Based on a released version
   - Changes can be made freely
   - Previous released version remains available
   - Stored in a feature branch in Git

5. **Obsolete**: Part is no longer active
   - Cannot be used in new designs
   - Remains visible for historical purposes
   - Existing usages are flagged for replacement
   - Remains in the main branch in Git with obsolete flag

## Working with Multiple Parts

### Parallel Development

Implexa supports working on multiple parts simultaneously:

1. **Independent Git Branches**: Each part in draft state exists in its own Git branch
2. **Workspace Management**: The system maintains separate working directories for each part
3. **CAD Integration**: Multiple parts can be opened in CAD simultaneously through:
   - Separate CAD instances pointing to different working directories
   - A single CAD instance with multiple projects open
   - Sparse checkout to only load relevant files for each part

### Multi-Part Workflow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Part A      │    │ Part B      │    │ Part C      │
│ (Draft)     │    │ (Draft)     │    │ (Draft)     │
└──────┬──────┘    └──────┬──────┘    └──────┬──────┘
       │                  │                  │
       │                  │                  │
       ▼                  ▼                  ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Working     │    │ Working     │    │ Working     │
│ Directory A │    │ Directory B │    │ Directory C │
└──────┬──────┘    └──────┬──────┘    └──────┬──────┘
       │                  │                  │
       │                  │                  │
       ▼                  ▼                  ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ CAD         │    │ CAD         │    │ CAD         │
│ Instance A  │    │ Instance B  │    │ Instance C  │
└─────────────┘    └─────────────┘    └─────────────┘
```

### Assembly Integration

When working with assemblies that include multiple parts:

1. **Reference Released Parts**: Assemblies typically reference released versions of parts
2. **Draft Assembly with Released Parts**: A draft assembly can include released parts
3. **Draft Assembly with Draft Parts**: For coordinated changes, a draft assembly can reference specific draft versions of parts
4. **Workspace Synchronization**: The system can synchronize multiple workspaces to ensure consistent references

## Simplified Roles and Permissions

Implexa uses a simple role-based permission system designed for small teams:

### Core Roles

1. **Designer**: Can create, edit, and submit parts for review
   - Create and modify parts
   - Submit parts for review
   - Comment on parts
   - Approve or reject parts (when not the author)

2. **Viewer**: Can view and comment on parts but not modify them
   - View all parts
   - Comment on parts
   - Cannot modify parts or approve/reject

3. **Admin**: Has full control over the system
   - All Designer permissions
   - Manage users and roles
   - Configure system settings
   - Override approvals if necessary

### Permissions Matrix

| Action                   | Designer | Viewer | Admin |
|--------------------------|----------|--------|-------|
| View parts               | ✓        | ✓      | ✓     |
| Create parts             | ✓        | -      | ✓     |
| Edit own draft parts     | ✓        | -      | ✓     |
| Edit others' draft parts | -        | -      | ✓     |
| Submit for review        | ✓        | -      | ✓     |
| Comment on parts         | ✓        | ✓      | ✓     |
| Approve/reject           | ✓*       | -      | ✓     |
| Release parts            | ✓*       | -      | ✓     |
| Mark as obsolete         | ✓*       | -      | ✓     |
| Manage users             | -        | -      | ✓     |

*Designers cannot approve/reject/release their own parts

## Simplified Approval Process

Implexa uses a straightforward approval process similar to Git merge/pull requests:

### Review Process

```
┌─────────────┐
│ Part in     │
│ Draft State │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Submit for  │
│ Review      │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Notify      │
│ Team        │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Team        │
│ Reviews     │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Approved?   │
└──────┬──────┘
       │
      ┌┴┐
     Yes No
      │  │
      │  ▼
      │ ┌─────────────┐
      │ │ Return to   │
      │ │ Draft       │
      │ └─────────────┘
      │
      ▼
┌─────────────┐
│ Release     │
└─────────────┘
```

1. Designer submits part for review
2. Team members are notified
3. Team members review the part and provide comments
4. At least one team member (not the author) must approve
5. If rejected, the part returns to Draft state for revision
6. If approved, the part can be released

### Review Features

1. **Simple Tagging**: Designer can tag specific team members for review
2. **Comments**: Reviewers can add comments to specific aspects of the part
3. **Approval**: Reviewers can approve the part with a single click
4. **Rejection**: Reviewers can reject the part with comments
5. **Notifications**: Team members receive notifications for review requests and comments

## Detailed Workflow

### Part Creation

```
┌─────────────┐
│ Start       │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Create Part │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Assign IPN  │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Set Initial │
│ Metadata    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Create Git  │
│ Feature     │
│ Branch      │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Save to     │
│ Database    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Part in     │
│ Draft State │
└─────────────┘
```

1. Designer initiates part creation through the UI
2. System assigns a unique Internal Part Number (IPN) based on the category-subcategory-sequential schema
3. Designer sets initial metadata (name, description, category, subcategory)
4. System creates a Git feature branch for the part
5. System saves the part to the database in Draft state
6. Designer can now add files, properties, and relationships to the part

### Part Modification (Draft State)

```
┌─────────────┐
│ Part in     │
│ Draft State │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Modify in   │
│ CAD or UI   │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Commit      │
│ Changes     │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Update      │
│ Database    │
└─────────────┘
```

1. Designer modifies the part in CAD or through the UI
2. Designer commits changes with a message
3. System commits changes to the Git feature branch
4. System updates the database with the changes
5. Designer can continue making changes or submit for review

### Review and Release

```
┌─────────────┐
│ Part in     │
│ Draft State │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Submit for  │
│ Review      │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Create      │
│ Review      │
│ Branch      │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Notify      │
│ Team        │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Team        │
│ Reviews     │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Approved?   │
└──────┬──────┘
       │
      ┌┴┐
     Yes No
      │  │
      │  ▼
      │ ┌─────────────┐
      │ │ Return to   │
      │ │ Draft       │
      │ └─────────────┘
      │
      ▼
┌─────────────┐
│ Merge to    │
│ Main Branch │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Part in     │
│ Released    │
│ State       │
└─────────────┘
```

1. Designer submits part for review
2. System creates a review branch from the feature branch
3. System changes the part state to In Review in the database
4. System notifies team members
5. Team members review the part and add comments
6. At least one team member (not the author) approves the part
7. If rejected:
   - System changes the part state back to Draft
   - Designer addresses comments and resubmits
8. If approved:
   - System merges the review branch to the main branch
   - System changes the part state to Released in the database

### Revision Process

```
┌─────────────┐
│ Part in     │
│ Released    │
│ State       │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Create      │
│ Revision    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Increment   │
│ Version     │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Create      │
│ Feature     │
│ Branch      │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Copy        │
│ Metadata    │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Part in     │
│ Draft State │
└─────────────┘
```

1. Designer initiates a revision of a released part
2. System increments the version number
3. System creates a new feature branch from the main branch
4. System copies the metadata from the released version
5. System creates a new revision in the database in Draft state
6. Designer can now modify the part as needed
7. The revision follows the same workflow as a new part from this point

## Integration with Git

The Part Management Workflow integrates closely with Git:

### Branch Strategy

1. **Main Branch**: Contains all released and obsolete parts
2. **Feature Branches**: Used for drafting new parts and revisions
   - Named with pattern: `part/[IPN]/draft`
   - One branch per part in draft state
3. **Review Branches**: Used for reviewing parts before release
   - Named with pattern: `part/[IPN]/review`

### Workspace Management

1. **Multiple Working Directories**: System maintains separate working directories for each part in draft state
2. **Sparse Checkout**: Only relevant files are checked out for each part
3. **Workspace Switching**: UI provides easy switching between workspaces

### CAD Integration

1. **File Locking**: Prevent concurrent edits to the same CAD file
2. **CAD Workspace Configuration**: Automatically configure CAD to use the correct working directory
3. **Multi-Instance Support**: Launch multiple CAD instances for different parts

## Integration with Database

The Part Management Workflow integrates with the SQLite database:

### State Tracking

1. **Part State**: Stored in the `Revisions` table
2. **Approval Status**: Stored in the `Approvals` table
3. **Workflow History**: Stored in a new `WorkflowHistory` table

### Transaction Management

1. **Atomic Transactions**: Database changes are wrapped in transactions
2. **Rollback on Error**: Transactions are rolled back if an error occurs
3. **Commit on Success**: Transactions are committed only when all operations succeed

## User Interface Considerations

The Part Management Workflow has several implications for the user interface:

### State Visualization

1. **State Indicators**: Clear visual indicators of part state
2. **State Transitions**: Visual representation of available transitions
3. **State History**: Timeline view of state changes

### Workspace Management

1. **Workspace Selector**: Easily switch between different parts in draft state
2. **Workspace Status**: Show status of each workspace (modified, clean, etc.)
3. **Workspace Actions**: Actions specific to the current workspace

### Review Interface

1. **Review Dashboard**: Show all parts in review
2. **Comment Interface**: Add comments to specific aspects of the part
3. **Approval Interface**: Simple approve/reject buttons

## Implementation Approach

The Part Management Workflow will be implemented in phases:

### Phase 1: Basic Workflow

1. Implement the core states (Draft, In Review, Released, Obsolete)
2. Implement basic transitions between states
3. Implement simple approval process
4. Integrate with Git for basic version control

### Phase 2: Multi-Part Workflow

1. Implement workspace management for multiple parts
2. Implement CAD integration for multiple parts
3. Enhance Git integration with sparse checkout
4. Implement the Revision state and process

## Error Handling

The Part Management Workflow includes comprehensive error handling:

### Validation Errors

1. **State Transition Validation**: Ensure transitions are valid
2. **Approval Validation**: Ensure approvals are valid
3. **Data Validation**: Ensure required data is present and valid

### Conflict Resolution

1. **Merge Conflicts**: Provide tools to resolve Git merge conflicts
2. **Concurrent Edit Conflicts**: Detect and resolve concurrent edits
3. **Dependency Conflicts**: Detect and resolve conflicts in part dependencies

## Security Considerations

The Part Management Workflow includes several security considerations:

### Access Control

1. **Role-Based Access**: Different roles have different permissions
2. **State-Based Access**: Access to parts depends on their state
3. **Owner-Based Access**: Creators have special permissions on their parts

### Audit Trail

1. **State Changes**: Log all state changes with user and timestamp
2. **Approvals**: Log all approvals with user and timestamp
3. **Content Changes**: Log all content changes with user and timestamp

## Conclusion

The Part Management Workflow provides a simple yet robust foundation for managing parts throughout their lifecycle, with a focus on usability for small teams. Its integration with Git and the database ensures that all changes are tracked and traceable, while its support for multiple parts in draft state enables efficient parallel development. The simplified approval process, similar to Git merge/pull requests, makes it easy for team members to collaborate on part design and review.