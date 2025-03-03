# Implexa: Unit Testing Approach

**Navigation:** [productContext](./productContext.md) | [activeContext](./activeContext.md) | [progress](./progress.md) | [decisionLog](./decisionLog.md) | [Memory Bank Index](./memory-bank-index.md)

## Overview

This document outlines the unit testing approach for the Implexa project. It defines the testing philosophy, tools, patterns, and practices to ensure high-quality, maintainable code that meets the project requirements. This approach is designed to be consistent with the project's architecture and coding standards while following Rust ecosystem best practices.

## Testing Philosophy

### Core Principles

1. **Test-Driven Development (TDD)** - Where appropriate, write tests before implementing features to ensure code is designed with testability in mind.
2. **Comprehensive Coverage** - Aim for high test coverage, especially for critical components like the Git Backend Manager and Database modules.
3. **Isolation** - Unit tests should test components in isolation, using mocks or stubs for dependencies.
4. **Readability** - Tests should be clear, concise, and serve as documentation for how components should be used.
5. **Maintainability** - Tests should be easy to maintain and update as the codebase evolves.
6. **Fast Execution** - Unit tests should run quickly to encourage frequent execution during development.

### Testing Pyramid

The testing strategy follows the testing pyramid approach:

1. **Unit Tests** (This document) - Testing individual functions, methods, and classes in isolation
2. **Integration Tests** - Testing interactions between components
3. **End-to-End Tests** - Testing complete workflows from UI to filesystem

## Testing Tools and Framework

### Primary Testing Tools

1. **Rust's Built-in Testing Framework** - Utilize Rust's native testing capabilities with `#[test]` attributes
2. **Cargo Test** - Use `cargo test` as the primary test runner
3. **tempfile** - For creating temporary files and directories during tests (already included in dev-dependencies)
4. **mockall** (to be added) - For creating mock objects to isolate components during testing
5. **rstest** (to be added) - For parameterized testing to reduce test code duplication
6. **test-case** (to be added) - For data-driven testing with multiple test cases
7. **proptest** (to be added) - For property-based testing to find edge cases

### Recommended Additions to Cargo.toml

```toml
[dev-dependencies]
# Existing
tempfile = "3.10.0"
# New additions
mockall = "0.12.1"
rstest = "0.18.2"
test-case = "3.3.1"
proptest = "1.4.0"
criterion = "0.5.1"  # For benchmarking
```

## Testing Structure

### Test Organization

1. **In-Module Tests**
   - Simple unit tests should be placed in a `tests` module at the bottom of the same file as the code being tested
   - Use `#[cfg(test)]` to ensure test code is only compiled during testing

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_function_name() {
           // Test implementation
       }
   }
   ```

2. **Separate Test Files**
   - More complex tests should be placed in a separate file in a `tests` directory
   - Follow the same module structure as the main code
   - Example: `src/git_backend/repository.rs` would have tests in `tests/git_backend/repository_tests.rs`

3. **Integration Tests**
   - Place integration tests in the `tests` directory at the project root
   - Each file in this directory is compiled as a separate crate

### Naming Conventions

1. **Test Functions**
   - Prefix with `test_`
   - Use descriptive names that explain what is being tested
   - Include the expected outcome in the name
   - Example: `test_commit_with_valid_message_succeeds`

2. **Test Modules**
   - For separate test files, use the name of the module being tested with a `_tests` suffix
   - Example: `repository_tests.rs`

## Testing Patterns and Practices

### Unit Testing Patterns

1. **Arrange-Act-Assert (AAA)**
   - Arrange: Set up the test environment and inputs
   - Act: Call the function or method being tested
   - Assert: Verify the expected outcome

   ```rust
   #[test]
   fn test_resolve_conflict_with_ours_strategy() {
       // Arrange
       let repo = setup_test_repository();
       let file_path = Path::new("conflicted_file.txt");
       create_conflict(repo, file_path);
       
       // Act
       let result = repo.resolve_conflict(file_path, ConflictStrategy::Ours);
       
       // Assert
       assert!(result.is_ok());
       assert_eq!(read_file_content(file_path), "our content");
   }
   ```

2. **Given-When-Then (BDD Style)**
   - Given: The initial context
   - When: The action is performed
   - Then: The expected outcome

   ```rust
   #[test]
   fn test_part_creation() {
       // Given
       let db = setup_test_database();
       let part_data = PartData {
           category: "EL",
           subcategory: "RES",
           name: "Test Resistor",
           // ...
       };
       
       // When
       let result = db.create_part(part_data);
       
       // Then
       assert!(result.is_ok());
       let part_id = result.unwrap();
       let part = db.get_part(part_id).unwrap();
       assert_eq!(part.name, "Test Resistor");
   }
   ```

### Mocking and Test Doubles

1. **Mocking External Dependencies**
   - Use `mockall` to create mock objects for external dependencies
   - Define expectations for method calls
   - Verify interactions with dependencies

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       use mockall::predicate::*;
       use mockall::mock;
       
       mock! {
           GitRepository {
               fn commit(&self, message: &str) -> Result<String>;
           }
       }
       
       #[test]
       fn test_workflow_transition_commits_changes() {
           // Arrange
           let mut mock_repo = MockGitRepository::new();
           mock_repo
               .expect_commit()
               .with(eq("Transition part to Released state"))
               .times(1)
               .returning(|_| Ok("abc123".to_string()));
           
           let workflow = Workflow::new(mock_repo);
           
           // Act
           let result = workflow.transition_part("EL-RES-100001", State::Draft, State::Released);
           
           // Assert
           assert!(result.is_ok());
       }
   }
   ```

2. **Test Fixtures and Factories**
   - Create helper functions to set up common test scenarios
   - Use factory functions to create test objects with sensible defaults

   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       fn create_test_part() -> Part {
           Part {
               id: "EL-RES-100001".to_string(),
               category: "EL".to_string(),
               subcategory: "RES".to_string(),
               name: "Test Resistor".to_string(),
               state: State::Draft,
               // ...
           }
       }
       
       #[test]
       fn test_part_validation() {
           let part = create_test_part();
           assert!(part.validate().is_ok());
       }
   }
   ```

### Testing Error Handling

1. **Testing Success Cases**
   - Verify that functions return the expected result for valid inputs

   ```rust
   #[test]
   fn test_create_repository_success() {
       let path = tempfile::tempdir().unwrap().path().to_path_buf();
       let result = GitBackend::create_repository(&path);
       assert!(result.is_ok());
       assert!(path.join(".git").exists());
   }
   ```

2. **Testing Error Cases**
   - Verify that functions return the expected error for invalid inputs
   - Test all error variants defined in custom error types

   ```rust
   #[test]
   fn test_create_repository_already_exists() {
       let path = tempfile::tempdir().unwrap().path().to_path_buf();
       // Create the repository first
       GitBackend::create_repository(&path).unwrap();
       
       // Try to create it again
       let result = GitBackend::create_repository(&path);
       assert!(result.is_err());
       match result {
           Err(GitBackendError::RepositoryError(msg)) => {
               assert!(msg.contains("already exists"));
           }
           _ => panic!("Expected RepositoryError"),
       }
   }
   ```

### Parameterized Testing

1. **Using rstest for Parameterized Tests**
   - Test the same function with multiple inputs
   - Reduce code duplication

   ```rust
   use rstest::rstest;
   
   #[cfg(test)]
   mod tests {
       use super::*;
       use rstest::rstest;
       
       #[rstest]
       #[case("EL", "RES", true)]
       #[case("EL", "INVALID", false)]
       #[case("INVALID", "RES", false)]
       fn test_validate_category_subcategory(
           #[case] category: &str,
           #[case] subcategory: &str,
           #[case] expected: bool,
       ) {
           let result = validate_category_subcategory(category, subcategory);
           assert_eq!(result, expected);
       }
   }
   ```

2. **Using test-case for Data-Driven Tests**
   - Similar to rstest but with a different syntax

   ```rust
   use test_case::test_case;
   
   #[test_case("EL-RES-100001", true ; "valid part number")]
   #[test_case("EL-INVALID-100001", false ; "invalid subcategory")]
   #[test_case("INVALID-RES-100001", false ; "invalid category")]
   #[test_case("EL-RES-ABC", false ; "invalid sequence number")]
   fn test_validate_part_number(part_number: &str, expected: bool) {
       let result = validate_part_number(part_number);
       assert_eq!(result, expected);
   }
   ```

### Property-Based Testing

1. **Using proptest for Property-Based Tests**
   - Generate random inputs to find edge cases
   - Define properties that should hold for all inputs

   ```rust
   use proptest::prelude::*;
   
   proptest! {
       #[test]
       fn test_part_number_roundtrip(
           category in "[A-Z]{2}",
           subcategory in "[A-Z]{3}",
           number in 100000..999999u32,
       ) {
           let part_number = format!("{}-{}-{}", category, subcategory, number);
           if is_valid_category(&category) && is_valid_subcategory(&subcategory) {
               let parsed = parse_part_number(&part_number).unwrap();
               assert_eq!(parsed.category, category);
               assert_eq!(parsed.subcategory, subcategory);
               assert_eq!(parsed.number, number);
           } else {
               assert!(parse_part_number(&part_number).is_err());
           }
       }
   }
   ```

## Component-Specific Testing Strategies

### Git Backend Manager Testing

1. **Repository Operations**
   - Test repository creation, cloning, and initialization
   - Test commit, branch, and merge operations
   - Test conflict resolution strategies
   - Test authentication and authorization

2. **Git-LFS Integration**
   - Test large file handling
   - Test LFS configuration and setup
   - Test LFS operations (track, untrack, etc.)

3. **Hook Management**
   - Test hook installation and execution
   - Test pre-commit and post-commit hooks
   - Test custom hooks for PLM metadata

4. **Error Handling**
   - Test error cases for all Git operations
   - Verify appropriate error types and messages

### Database Module Testing

1. **Schema Management**
   - Test schema creation and migration
   - Test schema validation

2. **CRUD Operations**
   - Test create, read, update, and delete operations for all entities
   - Test transaction handling and rollback

3. **Relationship Management**
   - Test parent-child relationships
   - Test many-to-many relationships
   - Test relationship constraints

4. **Query Performance**
   - Test query execution time for common operations
   - Test query optimization strategies

### Workflow Engine Testing

1. **State Transitions**
   - Test valid state transitions
   - Test invalid state transitions
   - Test transition conditions and guards

2. **Approval Process**
   - Test approval creation and validation
   - Test approval workflow
   - Test approval notifications

3. **Integration with Git**
   - Test workflow integration with Git operations
   - Test branch strategy enforcement

### Part Management Testing

1. **Part Creation**
   - Test part creation with valid data
   - Test part creation with invalid data
   - Test part numbering rules

2. **Part Relationships**
   - Test parent-child relationships
   - Test assembly structure
   - Test relationship constraints

3. **Part Properties**
   - Test property creation and validation
   - Test property inheritance
   - Test property constraints

## Test Coverage

### Coverage Goals

1. **Line Coverage**
   - Aim for at least 80% line coverage for critical components
   - Minimum 70% line coverage for all components

2. **Branch Coverage**
   - Aim for at least 75% branch coverage for critical components
   - Minimum 65% branch coverage for all components

3. **Function Coverage**
   - Aim for 100% function coverage for public APIs
   - Minimum 90% function coverage for all functions

### Coverage Measurement

1. **Tools**
   - Use `cargo-tarpaulin` for measuring test coverage
   - Configure CI/CD pipeline to run coverage analysis

2. **Reporting**
   - Generate coverage reports for each build
   - Track coverage trends over time
   - Identify areas with low coverage for improvement

## Test Data Management

### Test Data Principles

1. **Isolation**
   - Tests should not depend on external data
   - Each test should create its own data
   - Tests should clean up after themselves

2. **Reproducibility**
   - Tests should be reproducible
   - Use fixed seeds for random data generation
   - Avoid dependencies on system state

### Test Data Strategies

1. **In-Memory Databases**
   - Use in-memory SQLite databases for testing
   - Initialize schema for each test
   - Populate with test data

2. **Temporary Files and Directories**
   - Use the `tempfile` crate to create temporary files and directories
   - Clean up automatically after tests

3. **Test Fixtures**
   - Create reusable test fixtures for common scenarios
   - Use factory functions to create test objects

## Continuous Integration

### CI/CD Integration

1. **Automated Testing**
   - Run tests on every pull request
   - Run tests on every push to main branch
   - Block merges if tests fail

2. **Coverage Analysis**
   - Run coverage analysis on every pull request
   - Track coverage trends over time
   - Set minimum coverage thresholds

3. **Performance Testing**
   - Run benchmarks on performance-critical components
   - Track performance trends over time
   - Alert on performance regressions

### GitHub Actions Configuration

```yaml
name: Rust Tests

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
          override: true
      - name: Run tests
        uses: actions-rs/cargo@v1
        with:
          command: test
      - name: Run coverage
        uses: actions-rs/tarpaulin@v0.1
        with:
          version: '0.15.0'
          args: '--out Xml'
      - name: Upload coverage to Codecov
        uses: codecov/codecov-action@v1
```

## Implementation Plan

### Phase 1: Basic Testing Infrastructure

1. **Set Up Testing Framework**
   - Add necessary dependencies to Cargo.toml
   - Configure test runners and coverage tools
   - Create test directory structure

2. **Create Test Utilities**
   - Implement test fixtures and factories
   - Create helper functions for common test operations
   - Set up mocking infrastructure

3. **Write Initial Tests**
   - Focus on critical components (Git Backend, Database)
   - Implement tests for core functionality
   - Establish patterns for future tests

### Phase 2: Comprehensive Test Suite

1. **Expand Test Coverage**
   - Write tests for all components
   - Implement property-based tests for complex logic
   - Add parameterized tests for edge cases

2. **Integration Tests**
   - Implement tests for component interactions
   - Test end-to-end workflows
   - Verify system behavior

3. **Performance Tests**
   - Implement benchmarks for critical operations
   - Establish performance baselines
   - Set up performance regression testing

### Phase 3: Continuous Improvement

1. **Refine Test Suite**
   - Identify and fix flaky tests
   - Improve test readability and maintainability
   - Optimize test execution time

2. **Enhance Coverage**
   - Identify and fill coverage gaps
   - Add tests for edge cases and error conditions
   - Improve property-based tests

3. **Documentation and Training**
   - Document testing patterns and practices
   - Train team members on testing approach
   - Establish code review guidelines for tests

## Conclusion

This unit testing approach provides a comprehensive framework for ensuring the quality and reliability of the Implexa codebase. By following these guidelines, the team can build a robust test suite that verifies the correctness of the code, catches regressions, and supports ongoing development.

The approach is designed to be flexible and adaptable as the project evolves, with a focus on maintainability, readability, and effectiveness. Regular reviews and updates to this document will ensure that it remains relevant and useful throughout the project lifecycle.

## Related Files
- [Product Context](./productContext.md) - Project overview and high-level design
- [Active Context](./activeContext.md) - Current session focus and recent activities
- [Decision Log](./decisionLog.md) - Key architectural decisions
- [Git Backend Architecture](./git-backend-architecture.md) - Git backend component design
- [Database Schema Design](./database-schema-design.md) - SQLite database schema design
- [Coding Standards](./coding-standards.md) - Code style and practices

## Related Decisions
- [DEC-012](./decisionLog.md#dec-012---unit-testing-approach) - Unit Testing Approach

## Implementation
This testing approach will be implemented in:
- Test files alongside implementation files
- Separate test files in the tests directory
- CI/CD configuration for automated testing
- Test utilities and helpers for common testing patterns