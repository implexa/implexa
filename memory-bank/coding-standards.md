# Implexa: Coding Standards

This document outlines the coding standards and style guidelines for the Implexa project. Following these standards ensures consistency across the codebase and makes it easier for developers to understand and maintain the code.

## Rust Coding Standards

### Module Organization

- **Filename-as-Module Pattern**: We use the filename-as-module pattern for organizing Rust modules.
  - Each module is a file named after the module (e.g., `src/git_backend/conflict.rs`)
  - This is preferred over the older mod.rs pattern (e.g., `src/git_backend/conflict/mod.rs`)
  - See DEC-010 in the Decision Log for the rationale behind this choice

### Naming Conventions

- **Snake Case**: Use snake_case for variables, functions, methods, modules, and file names
  ```rust
  let user_id = get_user_id();
  ```

- **Camel Case**: Use CamelCase for types, traits, and enum variants
  ```rust
  struct UserAccount { ... }
  enum FileType { Text, Binary, Directory }
  ```

- **Screaming Snake Case**: Use SCREAMING_SNAKE_CASE for constants and static variables
  ```rust
  const MAX_CONNECTIONS: u32 = 100;
  ```

### Documentation

- All public items (functions, structs, traits, etc.) should have documentation comments
- Use `///` for documenting items and `//!` for module-level documentation
- Include examples in documentation where appropriate
- Document parameters, return values, and potential errors

```rust
/// Resolves a conflict in a Git repository.
///
/// # Arguments
///
/// * `file` - The path to the conflicted file
/// * `strategy` - The strategy to use for resolving the conflict
///
/// # Returns
///
/// A Result indicating success or failure
///
/// # Errors
///
/// Returns an error if the file doesn't exist or if there's no conflict
pub fn resolve_conflict(&self, file: &Path, strategy: ConflictStrategy) -> Result<()> {
    // ...
}
```

### Error Handling

- Use the `Result` type for functions that can fail
- Create custom error types using `thiserror` for domain-specific errors
- Provide meaningful error messages
- Use `?` operator for error propagation

```rust
#[derive(Error, Debug)]
pub enum GitBackendError {
    #[error("Git error: {0}")]
    GitError(#[from] git2::Error),
    
    #[error("Repository error: {0}")]
    RepositoryError(String),
    
    // ...
}

pub type Result<T> = std::result::Result<T, GitBackendError>;
```

### Code Organization

- Group related functionality into modules
- Use clear and descriptive names for modules
- Keep files to a reasonable size (aim for under 500 lines)
- Separate interface from implementation where appropriate

### Formatting

- Use `rustfmt` for consistent code formatting
- Use 4 spaces for indentation
- Maximum line length of 100 characters
- Run `cargo fmt` before committing code

### Testing

- Write unit tests for all public functions
- Use the `#[test]` attribute for test functions
- Place tests in a `tests` module at the bottom of the file or in a separate file
- Use descriptive test names that explain what is being tested

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_resolve_conflict_with_ours_strategy() {
        // ...
    }
}
```

## TypeScript/JavaScript Coding Standards

### Naming Conventions

- **Camel Case**: Use camelCase for variables, functions, and methods
  ```typescript
  const userId = getUserId();
  ```

- **Pascal Case**: Use PascalCase for classes, interfaces, types, and React components
  ```typescript
  class UserAccount { ... }
  interface UserProps { ... }
  const UserProfile: React.FC = () => { ... }
  ```

### File Organization

- One component per file for React components
- Group related components in directories
- Use index.ts files for re-exporting components

### TypeScript Specifics

- Use TypeScript for all new code
- Define explicit types for function parameters and return values
- Use interfaces for object shapes
- Use type aliases for complex types
- Avoid using `any` type

### React Conventions

- Use functional components with hooks
- Use TypeScript interfaces for component props
- Use the Context API for state management
- Follow the container/presentational pattern where appropriate

## CSS/Styling Standards

- Use TailwindCSS for styling
- Follow utility-first approach
- Extract common patterns to components
- Use consistent naming for custom classes

## Version Control Practices

- Write clear, descriptive commit messages
- Use present tense in commit messages
- Reference issue numbers in commit messages when applicable
- Keep commits focused on a single change
- Use branches for feature development and bug fixes

## Documentation

- Document all APIs
- Keep documentation up-to-date with code changes
- Use Markdown for documentation
- Include examples in documentation

## Continuous Integration

- All code must pass linting and tests before merging
- Follow semantic versioning for releases
- Automate builds and testing through GitHub Actions

## Conclusion

These coding standards are designed to ensure consistency and maintainability across the Implexa codebase. They should be followed for all new code and applied to existing code during refactoring.

The standards may evolve over time as the project grows and as best practices in the industry change. Significant changes to these standards should be documented in the Decision Log.