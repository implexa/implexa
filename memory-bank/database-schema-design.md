# SQLite Database Schema Design

**Navigation:** [productContext](./productContext.md) | [activeContext](./activeContext.md) | [progress](./progress.md) | [decisionLog](./decisionLog.md) | [Memory Bank Index](./memory-bank-index.md)

## Overview

The SQLite database is a critical component of Implexa that stores metadata for parts, relationships, and properties. This document outlines the database schema design, focusing on the structure, relationships, and integration with other components.

## Design Principles

1. **Simplicity**: Keep the schema as simple as possible while meeting all requirements
2. **Flexibility**: Support extensible metadata through key-value properties
3. **Integrity**: Enforce referential integrity and constraints
4. **Performance**: Optimize for common queries and operations
5. **Versioning**: Support schema evolution over time
6. **Git Integration**: Design with Git-based version control in mind

## Database Structure

The database consists of several interconnected tables that store different aspects of the PLM data:

```
┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐
│     Parts       │       │   Revisions     │       │  Relationships  │
│                 │       │                 │       │                 │
│ - Part ID       │◄──┐   │ - Revision ID   │       │ - Relationship  │
│ - Category      │   └───┤ - Part ID       │       │   ID            │
│ - Subcategory   │       │ - Version       │       │ - Parent Part   │
│ - Name          │       │ - Status        │       │   ID            │
│ - Description   │       │ - Created Date  │       │ - Child Part    │
│ - Created Date  │       │ - Created By    │       │   ID            │
│ - Modified Date │       │ - Commit Hash   │       │ - Type          │
└─────────────────┘       └─────────────────┘       │ - Quantity      │
                                                    └─────────────────┘
                                                    
┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐
│   Properties    │       │ Manufacturer    │       │   Approvals     │
│                 │       │   Parts         │       │                 │
│ - Property ID   │       │ - MPN ID        │       │ - Approval ID   │
│ - Part ID       │       │ - Part ID       │       │ - Revision ID   │
│ - Revision ID   │       │ - Manufacturer  │       │ - Approver      │
│ - Key           │       │ - MPN           │       │ - Status        │
│ - Value         │       │ - Description   │       │ - Date          │
│ - Type          │       │ - Status        │       │ - Comments      │
└─────────────────┘       └─────────────────┘       └─────────────────┘

┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐
│    Files        │       │   Workflows     │       │ Workflow States │
│                 │       │                 │       │                 │
│ - File ID       │       │ - Workflow ID   │       │ - State ID      │
│ - Part ID       │       │ - Name          │       │ - Workflow ID   │
│ - Revision ID   │       │ - Description   │       │ - Name          │
│ - Path          │       │ - Active        │       │ - Description   │
│ - Type          │       │                 │       │ - Is Initial    │
│ - Description   │       │                 │       │ - Is Terminal   │
└─────────────────┘       └─────────────────┘       └─────────────────┘

┌─────────────────┐       ┌─────────────────┐       ┌─────────────────┐
│  Categories     │       │  Subcategories  │       │  PartSequence   │
│                 │       │                 │       │                 │
│ - Category ID   │◄──┐   │ - Subcategory   │       │ - ID            │
│ - Name          │   └───┤   ID            │       │ - Next Value    │
│ - Code          │       │ - Category ID   │       │                 │
│ - Description   │       │ - Name          │       │                 │
│                 │       │ - Code          │       │                 │
│                 │       │ - Description   │       │                 │
└─────────────────┘       └─────────────────┘       └─────────────────┘
```

## Table Definitions

### Parts

The `Parts` table is the central table that stores information about all parts in the system.

```sql
CREATE TABLE Parts (
    part_id INTEGER PRIMARY KEY,
    category TEXT NOT NULL,
    subcategory TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    modified_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(category, subcategory, name)
);

CREATE INDEX idx_parts_category ON Parts(category);
CREATE INDEX idx_parts_subcategory ON Parts(subcategory);
CREATE INDEX idx_parts_name ON Parts(name);
```

### PartSequence

The `PartSequence` table manages the sequential numbering for parts.

```sql
CREATE TABLE PartSequence (
    id INTEGER PRIMARY KEY CHECK (id = 1), -- Only one row allowed
    next_value INTEGER NOT NULL DEFAULT 10000
);

-- Initialize the sequence with starting value 10000
INSERT OR IGNORE INTO PartSequence (id, next_value) VALUES (1, 10000);
```

### Categories

The `Categories` table stores configurable categories for parts.

```sql
CREATE TABLE Categories (
    category_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    description TEXT,
    UNIQUE(name),
    UNIQUE(code)
);

CREATE INDEX idx_categories_code ON Categories(code);
```

### Subcategories

The `Subcategories` table stores configurable subcategories for parts.

```sql
CREATE TABLE Subcategories (
    subcategory_id INTEGER PRIMARY KEY AUTOINCREMENT,
    category_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    code TEXT NOT NULL,
    description TEXT,
    FOREIGN KEY (category_id) REFERENCES Categories(category_id) ON DELETE CASCADE,
    UNIQUE(category_id, name),
    UNIQUE(category_id, code)
);

CREATE INDEX idx_subcategories_code ON Subcategories(code);
CREATE INDEX idx_subcategories_category_id ON Subcategories(category_id);
```

### Revisions

The `Revisions` table stores information about each revision of a part.

```sql
CREATE TABLE Revisions (
    revision_id INTEGER PRIMARY KEY AUTOINCREMENT,
    part_id INTEGER NOT NULL,
    version TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('Draft', 'In Review', 'Released', 'Obsolete')),
    created_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT NOT NULL,
    commit_hash TEXT,
    FOREIGN KEY (part_id) REFERENCES Parts(part_id) ON DELETE CASCADE,
    UNIQUE(part_id, version)
);

CREATE INDEX idx_revisions_part_id ON Revisions(part_id);
CREATE INDEX idx_revisions_status ON Revisions(status);
CREATE INDEX idx_revisions_commit_hash ON Revisions(commit_hash);
```

### Relationships

The `Relationships` table stores parent-child relationships between parts.

```sql
CREATE TABLE Relationships (
    relationship_id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_part_id INTEGER NOT NULL,
    child_part_id INTEGER NOT NULL,
    type TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY (parent_part_id) REFERENCES Parts(part_id) ON DELETE CASCADE,
    FOREIGN KEY (child_part_id) REFERENCES Parts(part_id) ON DELETE CASCADE,
    UNIQUE(parent_part_id, child_part_id, type)
);

CREATE INDEX idx_relationships_parent ON Relationships(parent_part_id);
CREATE INDEX idx_relationships_child ON Relationships(child_part_id);
CREATE INDEX idx_relationships_type ON Relationships(type);
```

### Properties

The `Properties` table stores key-value properties for parts and revisions.

```sql
CREATE TABLE Properties (
    property_id INTEGER PRIMARY KEY AUTOINCREMENT,
    part_id INTEGER,
    revision_id INTEGER,
    key TEXT NOT NULL,
    value TEXT,
    type TEXT NOT NULL DEFAULT 'string',
    FOREIGN KEY (part_id) REFERENCES Parts(part_id) ON DELETE CASCADE,
    FOREIGN KEY (revision_id) REFERENCES Revisions(revision_id) ON DELETE CASCADE,
    CHECK ((part_id IS NOT NULL AND revision_id IS NULL) OR (part_id IS NULL AND revision_id IS NOT NULL)),
    UNIQUE(part_id, revision_id, key)
);

CREATE INDEX idx_properties_part_id ON Properties(part_id);
CREATE INDEX idx_properties_revision_id ON Properties(revision_id);
CREATE INDEX idx_properties_key ON Properties(key);
```

### Manufacturer Parts

The `ManufacturerParts` table stores information about manufacturer parts that correspond to internal parts.

```sql
CREATE TABLE ManufacturerParts (
    mpn_id INTEGER PRIMARY KEY AUTOINCREMENT,
    part_id INTEGER NOT NULL,
    manufacturer TEXT NOT NULL,
    mpn TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'Active' CHECK(status IN ('Active', 'Preferred', 'Alternate', 'Obsolete')),
    FOREIGN KEY (part_id) REFERENCES Parts(part_id) ON DELETE CASCADE,
    UNIQUE(manufacturer, mpn)
);

CREATE INDEX idx_mpn_part_id ON ManufacturerParts(part_id);
CREATE INDEX idx_mpn_manufacturer ON ManufacturerParts(manufacturer);
CREATE INDEX idx_mpn_mpn ON ManufacturerParts(mpn);
CREATE INDEX idx_mpn_status ON ManufacturerParts(status);
```

### Approvals

The `Approvals` table stores approval information for revisions.

```sql
CREATE TABLE Approvals (
    approval_id INTEGER PRIMARY KEY AUTOINCREMENT,
    revision_id INTEGER NOT NULL,
    approver TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('Pending', 'Approved', 'Rejected')),
    date TIMESTAMP,
    comments TEXT,
    FOREIGN KEY (revision_id) REFERENCES Revisions(revision_id) ON DELETE CASCADE,
    UNIQUE(revision_id, approver)
);

CREATE INDEX idx_approvals_revision_id ON Approvals(revision_id);
CREATE INDEX idx_approvals_approver ON Approvals(approver);
CREATE INDEX idx_approvals_status ON Approvals(status);
```

### Files

The `Files` table stores information about files associated with parts and revisions.

```sql
CREATE TABLE Files (
    file_id INTEGER PRIMARY KEY AUTOINCREMENT,
    part_id INTEGER,
    revision_id INTEGER,
    path TEXT NOT NULL,
    type TEXT NOT NULL,
    description TEXT,
    FOREIGN KEY (part_id) REFERENCES Parts(part_id) ON DELETE CASCADE,
    FOREIGN KEY (revision_id) REFERENCES Revisions(revision_id) ON DELETE CASCADE,
    CHECK ((part_id IS NOT NULL AND revision_id IS NULL) OR (part_id IS NULL AND revision_id IS NOT NULL))
);

CREATE INDEX idx_files_part_id ON Files(part_id);
CREATE INDEX idx_files_revision_id ON Files(revision_id);
CREATE INDEX idx_files_type ON Files(type);
```

### Workflows

The `Workflows` table stores workflow definitions.

```sql
CREATE TABLE Workflows (
    workflow_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    active BOOLEAN NOT NULL DEFAULT 1,
    UNIQUE(name)
);
```

### Workflow States

The `WorkflowStates` table stores states for each workflow.

```sql
CREATE TABLE WorkflowStates (
    state_id INTEGER PRIMARY KEY AUTOINCREMENT,
    workflow_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    is_initial BOOLEAN NOT NULL DEFAULT 0,
    is_terminal BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (workflow_id) REFERENCES Workflows(workflow_id) ON DELETE CASCADE,
    UNIQUE(workflow_id, name)
);

CREATE INDEX idx_workflow_states_workflow_id ON WorkflowStates(workflow_id);
```

### Workflow Transitions

The `WorkflowTransitions` table stores allowed transitions between workflow states.

```sql
CREATE TABLE WorkflowTransitions (
    transition_id INTEGER PRIMARY KEY AUTOINCREMENT,
    workflow_id INTEGER NOT NULL,
    from_state_id INTEGER NOT NULL,
    to_state_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    requires_approval BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (workflow_id) REFERENCES Workflows(workflow_id) ON DELETE CASCADE,
    FOREIGN KEY (from_state_id) REFERENCES WorkflowStates(state_id) ON DELETE CASCADE,
    FOREIGN KEY (to_state_id) REFERENCES WorkflowStates(state_id) ON DELETE CASCADE,
    UNIQUE(workflow_id, from_state_id, to_state_id)
);

CREATE INDEX idx_workflow_transitions_workflow_id ON WorkflowTransitions(workflow_id);
CREATE INDEX idx_workflow_transitions_from_state_id ON WorkflowTransitions(from_state_id);
CREATE INDEX idx_workflow_transitions_to_state_id ON WorkflowTransitions(to_state_id);
```

## Key Relationships

1. **Parts to Revisions**: One-to-many relationship. Each part can have multiple revisions.
2. **Parts to Relationships**: One-to-many relationship in both directions (as parent and as child). Parts can be used in multiple assemblies and can contain multiple child parts.
3. **Parts/Revisions to Properties**: One-to-many relationship. Each part or revision can have multiple properties.
4. **Parts to Manufacturer Parts**: One-to-many relationship. Each part can have multiple manufacturer parts.
5. **Revisions to Approvals**: One-to-many relationship. Each revision can have multiple approvals.
6. **Parts/Revisions to Files**: One-to-many relationship. Each part or revision can have multiple associated files.
7. **Workflows to States**: One-to-many relationship. Each workflow can have multiple states.
8. **Workflows to Transitions**: One-to-many relationship. Each workflow can have multiple transitions between states.

## Data Types

1. **TEXT**: Used for strings, including part IDs, names, descriptions, etc.
2. **INTEGER**: Used for numeric values, auto-incrementing IDs, and boolean values (0 or 1).
3. **TIMESTAMP**: Used for date and time values.
4. **BLOB**: Used for binary data (if needed).

## Constraints

1. **Primary Keys**: Each table has a primary key to uniquely identify each row.
2. **Foreign Keys**: Used to enforce referential integrity between tables.
3. **Unique Constraints**: Used to prevent duplicate entries.
4. **Check Constraints**: Used to enforce valid values for certain fields.
5. **Not Null Constraints**: Used to ensure required fields are provided.

## Indexes

Indexes are created on columns that are frequently used in WHERE clauses, JOIN conditions, and ORDER BY clauses to improve query performance.

## Schema Evolution Strategy

The database schema will evolve over time as new features are added and requirements change. To manage this evolution:

1. **Version Table**: A `SchemaVersion` table will track the current schema version.

```sql
CREATE TABLE SchemaVersion (
    version INTEGER PRIMARY KEY,
    applied_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    description TEXT
);
```

2. **Migration Scripts**: Each schema change will be implemented as a migration script that updates the schema and the version number.
3. **Backward Compatibility**: Migrations will be designed to maintain backward compatibility where possible.
4. **Data Migration**: When schema changes require data migration, the migration scripts will handle this.

## Integration with Git Backend Manager

The database schema is designed to work seamlessly with the Git Backend Manager:

1. **Commit Hash References**: The `Revisions` table includes a `commit_hash` field that links each revision to a specific Git commit.
2. **File Path References**: The `Files` table includes a `path` field that references files in the Git repository.
3. **Metadata Storage**: The database stores metadata that complements the file storage in Git.
4. **Transaction Support**: SQLite's transaction support ensures that database changes are atomic, which is important when coordinating with Git operations.

## Example Queries

### Get all revisions of a part

```sql
SELECT r.*
FROM Revisions r
WHERE r.part_id = ?
ORDER BY r.created_date DESC;
```

### Get the BOM for a part

```sql
SELECT p.part_id, p.name, p.category, p.subcategory, r.quantity
FROM Parts p
JOIN Relationships r ON p.part_id = r.child_part_id
WHERE r.parent_part_id = ?;
```

### Get all properties of a part

```sql
SELECT p.key, p.value, p.type
FROM Properties p
WHERE p.part_id = ?;
```

### Get all manufacturer parts for a part

```sql
SELECT mp.manufacturer, mp.mpn, mp.description, mp.status
FROM ManufacturerParts mp
WHERE mp.part_id = ?;
```

### Get approval status for a revision

```sql
SELECT a.approver, a.status, a.date, a.comments
FROM Approvals a
WHERE a.revision_id = ?;
```

### Get all files for a part

```sql
SELECT f.path, f.type, f.description
FROM Files f
WHERE f.part_id = ?;
```

### Get workflow state for a part

```sql
SELECT ws.name, ws.description
FROM WorkflowStates ws
JOIN Revisions r ON r.status = ws.name
WHERE r.part_id = ? AND r.version = (
    SELECT MAX(version)
    FROM Revisions
    WHERE part_id = ?
);
```

## Performance Considerations

1. **Indexing Strategy**: Indexes are created on columns that are frequently used in queries to improve performance.
2. **Denormalization**: Where appropriate, some data may be denormalized to improve query performance.
3. **Query Optimization**: Complex queries will be optimized to minimize the use of temporary tables and subqueries.
4. **Connection Pooling**: The application will use connection pooling to minimize the overhead of creating new database connections.
5. **Transaction Management**: Transactions will be used to ensure data consistency and improve performance for batch operations.

## Security Considerations

1. **Input Validation**: All user input will be validated before being used in SQL queries to prevent SQL injection attacks.
2. **Prepared Statements**: Prepared statements will be used for all SQL queries to prevent SQL injection.
3. **Least Privilege**: The application will use a database user with the minimum necessary privileges.
4. **Encryption**: Sensitive data will be encrypted before being stored in the database.
5. **Audit Trail**: Changes to critical data will be logged in an audit trail.

## Backup and Recovery

1. **Regular Backups**: The database will be backed up regularly.
2. **Point-in-Time Recovery**: The backup strategy will support point-in-time recovery.
3. **Transaction Logs**: Transaction logs will be maintained to support recovery in case of failure.
4. **Integrity Checks**: Regular integrity checks will be performed to ensure database consistency.

## Conclusion

This database schema design provides a solid foundation for the Implexa PLM/PDM system. It supports the core requirements of part management, revision control, relationship tracking, and workflow management while providing flexibility for future extensions. The integration with the Git Backend Manager ensures that file storage and metadata are coordinated, providing a comprehensive solution for hardware product lifecycle management.

## Related Files
- [Product Context](./productContext.md) - Project overview and high-level design
- [Active Context](./activeContext.md) - Current session focus and recent activities
- [Decision Log](./decisionLog.md) - Key architectural decisions
- [Git Backend Architecture](./git-backend-architecture.md) - Git backend component design
- [Part Management Workflow](./part-management-workflow.md) - Part lifecycle workflow design
- [Coding Standards](./coding-standards.md) - Code style and practices
- [Unit Testing Approach](./unit-testing-approach.md) - Testing philosophy and practices

## Related Decisions
- [DEC-005](./decisionLog.md#dec-005---sqlite-database-schema-design) - SQLite Database Schema Design
- [DEC-011](./decisionLog.md#dec-011---database-schema-implementation) - Database Schema Implementation
- [DEC-013](./decisionLog.md#dec-013---enhanced-part-numbering-system) - Enhanced Part Numbering System

## Implementation
This schema is implemented in the following files:
- [src/database.rs](../src/database.rs) - Main Database module
- [src/database/schema.rs](../src/database/schema.rs) - Schema creation and initialization
- [src/database/part.rs](../src/database/part.rs) - Part entity and manager
- [src/database/revision.rs](../src/database/revision.rs) - Revision entity and manager
- [src/database/relationship.rs](../src/database/relationship.rs) - Relationship entity and manager
- [src/database/property.rs](../src/database/property.rs) - Property entity and manager
- [src/database/manufacturer_part.rs](../src/database/manufacturer_part.rs) - Manufacturer Part entity and manager
- [src/database/approval.rs](../src/database/approval.rs) - Approval entity and manager
- [src/database/file.rs](../src/database/file.rs) - File entity and manager
- [src/database/workflow.rs](../src/database/workflow.rs) - Workflow entities and manager
- [src/database/category.rs](../src/database/category.rs) - Category and Subcategory entities and manager
- [src/database/part_management.rs](../src/database/part_management.rs) - Part Management implementation