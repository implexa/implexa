# Implexa Memory Bank Index

This index provides a categorized list of all documentation in the memory bank for the Implexa project, a hardware-focused Git-based Product Lifecycle Management (PLM) system.

## Core Documentation
- [Product Context](./productContext.md) - Project overview and high-level design
- [Active Context](./activeContext.md) - Current session context and focus
- [Progress Tracking](./progress.md) - Project status and task list
- [Decision Log](./decisionLog.md) - Architectural decisions and rationales

## Architecture Documentation
- [Git Backend Architecture](./git-backend-architecture.md) - Design of the Git backend component
- [Database Schema Design](./database-schema-design.md) - SQLite database schema design
- [Part Management Workflow](./part-management-workflow.md) - Part lifecycle workflow design
- [User Interface Architecture](./user-interface-architecture.md) - UI design and components
- [Directory Structure](./directory-structure.md) - File and directory organization

## Development Guidelines
- [Coding Standards](./coding-standards.md) - Code style and practices
- [Rust Module Refactoring Guide](./rust-module-refactoring-guide.md) - Guide for module organization
- [Unit Testing Approach](./unit-testing-approach.md) - Testing philosophy and practices

## Project Requirements
- [Implexa Project Requirement Doc](./implexa-project-requirement-doc.md) - Detailed project requirements

## Navigation Guide

### For New Contributors
If you're new to the project, we recommend reading these files in the following order:
1. [Product Context](./productContext.md) - Understand the project overview
2. [Implexa Project Requirement Doc](./implexa-project-requirement-doc.md) - Review detailed requirements
3. [Progress Tracking](./progress.md) - See current project status
4. [Active Context](./activeContext.md) - Understand current focus

### For AI Assistants
When starting a new context, AI assistants should prioritize reading:
1. [Active Context](./activeContext.md) - Current session focus and recent activities
2. [Product Context](./productContext.md) - Project overview
3. [Progress Tracking](./progress.md) - Current tasks and status
4. [Decision Log](./decisionLog.md) - Key architectural decisions

### For Specific Tasks
- **Working on Git Backend**: Read [Git Backend Architecture](./git-backend-architecture.md)
- **Working on Database**: Read [Database Schema Design](./database-schema-design.md)
- **Working on Part Management**: Read [Part Management Workflow](./part-management-workflow.md)
- **Working on UI**: Read [User Interface Architecture](./user-interface-architecture.md)
- **Setting up Directory Structure**: Read [Directory Structure](./directory-structure.md)

## Memory Bank Structure

The Memory Bank is organized to provide comprehensive documentation for the Implexa project:

- **Core Documentation**: Essential files that provide project context and status
- **Architecture Documentation**: Detailed design documents for major components
- **Development Guidelines**: Standards and practices for development
- **Project Requirements**: Detailed requirements and specifications

## Updating the Memory Bank

When adding new documentation to the Memory Bank:
1. Add the file to the appropriate section in this index
2. Include standard navigation links at the top of the file
3. Add a "Related Files" section at the end of the file
4. Update any related files to reference the new file