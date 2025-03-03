# Implexa: Active Context

## Current Session Focus
- Directory Structure definition
- Project architecture documentation
- Development environment setup
- CI/CD pipeline configuration

## Recent Activities
- Created Memory Bank structure
- Documented project overview in productContext.md
- Defined detailed Git Backend Manager architecture
- Designed comprehensive SQLite database schema
- Defined Part Management Workflow with focus on small teams
- Designed User Interface Architecture with React and TailwindCSS
- Defined standardized Directory Structure for parts and libraries
- Analyzing project requirements from projectBrief.md and implexa-prd.md
- Created comprehensive development environment setup documentation for Windows, macOS, and Linux
- Created detailed CI/CD setup guide for GitHub Actions
- Created .gitignore file for the project

## Current Phase
Phase 1 (Core Infrastructure): Git backend, metadata storage, basic UI

## Key Insights
- The project uses Tauri and Rust for better performance and security compared to Electron
- KiCad integration is a priority for the initial CAD tool support
- Git-based version control is a core differentiator from other PLM/PDM solutions
- The part numbering system uses a category-subcategory-sequential approach
- The Git Backend Manager needs a modular design with clear interfaces for testability
- Git-LFS is essential for handling binary files and large assets in hardware design
- The database schema needs to support flexible metadata through key-value properties
- Workflow states and transitions need to be configurable in the database
- Integration between Git commits and database revisions is critical for traceability
- The Part Management Workflow should be simple and similar to Git MRs/PRs
- Multiple parts in draft state need to be supported with separate workspaces
- The system is designed for small teams (1-2 people) with simple role-based permissions
- The UI architecture uses React with TypeScript and TailwindCSS
- State management is handled with React Context API and custom hooks
- The UI is organized into pages, layouts, and shared components
- Tauri Commands API is used for backend integration
- The directory structure is configurable with minimal, standard, and extended templates
- Git-LFS is used for binary files and large assets
- Each part has a consistent directory structure with design, manufacturing, documentation, and tests
- The library structure supports multiple CAD tools with separate directories (kicad-library, etc.)
- Users can select which directories they need to avoid empty directories
- The development environment requires Rust, Node.js, Git with Git-LFS, and platform-specific dependencies
- Windows development requires additional components like WebView2 Runtime and C++ Build Tools
- Cross-platform development is supported with platform-specific setup instructions
- The CI/CD pipeline uses GitHub Actions for automated building, testing, and releasing
- Three workflow types are implemented: CI (for testing), Release (for production builds), and Development Builds (for previews)
- Code signing is configured for both macOS and Windows to improve security and user experience
- The project follows Semantic Versioning for version management

## Open Questions
- Current progress on Phase 1 implementation
- Specific technical challenges encountered so far
- Immediate next steps for development
- Integration approach between Git Backend Manager and Metadata Manager
- Performance considerations for SQLite with large part libraries
- Best approach for CAD integration with multiple workspaces
- Approach for handling offline scenarios in the UI
- Strategy for handling custom directory structures for specific part types