# zed-leetcode Development Guidelines

**AI Agent Exclusive Document - Project-Specific Rules Only**

## Project Architecture

### Core Structure

- **Main Entry**: `src/lib.rs` (Extension trait implementation)
- **Command Layer**: `src/commands.rs` (slash command handlers)
- **API Layer**: `src/api.rs` (LeetCode API communication via curl)
- **Auth Layer**: `src/auth.rs` (session management)
- **File Layer**: `src/file_manager.rs` (local data management)
- **Data Layer**: `src/models.rs` (data structures)

### Zed Extension Constraints

- **NEVER use src/main.rs** - Extensions use lib.rs only
- **NEVER use direct HTTP clients** - Use curl system commands only
- **NEVER attempt UI components** - Text-only output via SlashCommandOutput
- **ALWAYS use Worktree API** for file system operations

## TDD Implementation Standards

### Test Structure Requirements

- **ALWAYS write tests first** before implementing functionality
- **ALWAYS create tests module** for each src/\*.rs file
- **ALWAYS use integration tests** in tests/ directory for end-to-end flows

### Test File Patterns

```rust
// In each src/*.rs file:
#[cfg(test)]
mod tests {
    use super::*;
    // Unit tests here
}
```

### Test Execution Order

1. **Unit tests** - Individual function testing
2. **Integration tests** - Multi-module interaction
3. **Command tests** - Full slash command workflows

### Mock Patterns for External Calls

- **Mock curl commands** using test doubles
- **Mock file system operations** using temporary directories
- **Mock API responses** using fixture data

## Slash Command Implementation Rules

### Adding New Commands

**MUST modify these files simultaneously:**

1. `extension.toml` - Add command definition
2. `src/lib.rs` - Add route in run_slash_command
3. `src/commands.rs` - Add handler implementation

### Command Handler Pattern

```rust
pub fn handle_command_name(args: Vec<String>) -> Result<SlashCommandOutput, String> {
    // 1. Parse arguments
    // 2. Validate authentication (if required)
    // 3. Execute business logic
    // 4. Format output
}
```

### Error Handling Requirements

- **ALWAYS return Result<SlashCommandOutput, String>**
- **ALWAYS provide user-friendly error messages**
- **NEVER expose internal implementation details in errors**

## LeetCode API Communication Rules

### HTTP Request Pattern

- **ALWAYS use curl system commands** via std::process::Command
- **NEVER use reqwest, hyper, or other HTTP crates**
- **ALWAYS handle curl exit codes** and parse stderr for errors

### API Endpoint Management

- **Base URL**: `https://leetcode.com/graphql/`
- **Authentication**: Session cookie in request headers
- **Query Format**: GraphQL queries only

### Example curl Pattern

```rust
let output = std::process::Command::new("curl")
    .args(["-s", "-X", "POST", url])
    .args(["-H", "Content-Type: application/json"])
    .args(["-H", &format!("Cookie: {}", session_cookie)])
    .args(["-d", &query])
    .output()?;
```

## File System Management Rules

### Directory Structure Standards

```
.leetcode/
├── config.json          # User settings and auth
├── problems/            # Cached problem data
│   ├── 1.json
│   └── 2.json
└── solutions/           # User solution files
    ├── 1-two-sum.rs
    └── 2-add-two-numbers.py
```

### File Operations Requirements

- **ALWAYS use .leetcode directory** in workspace root
- **ALWAYS cache problem data** to avoid API rate limits
- **ALWAYS use Worktree API** for file path resolution
- **NEVER create files outside workspace scope**

### Configuration Management

- **Auth data**: Store in .leetcode/config.json with basic encoding
- **File permissions**: Set 600 for sensitive files
- **Cache expiry**: Implement time-based invalidation

## Code Quality Standards

### Naming Conventions

- **Files**: snake_case (e.g., `file_manager.rs`)
- **Functions**: snake_case (e.g., `handle_login`)
- **Structs**: PascalCase (e.g., `LeetCodeApi`)
- **Constants**: SCREAMING_SNAKE_CASE

### Error Handling Patterns

- **Use anyhow::Result** for internal functions
- **Convert to String errors** for command handlers
- **Provide context** with .context() or .with_context()

### Documentation Requirements

- **ALWAYS document public functions** with /// comments
- **ALWAYS explain complex algorithms** inline
- **NEVER document obvious functionality**

## Commit Convention Standards

### Commit Message Format

```
<type>(<scope>): <description>

<body>

<footer>
```

### Commit Types

- **Feat**: New feature implementation
- **Fix**: Bug fixes
- **Test**: Test additions or modifications
- **Refactor**: Code refactoring without behavior change
- **Docs**: Documentation updates
- **Chore**: Maintenance tasks

### Scope Examples

- **Auth**: Authentication related changes
- **Api**: LeetCode API communication
- **Commands**: Slash command handlers
- **Files**: File system operations

### Example Commits

```
feat(commands): add leetcode-list command with filtering

Implement problem list command with difficulty and tag filters.
Supports pagination and caching for performance.

Closes #123
```

## Multi-File Coordination Rules

### When Adding New Modules

**MUST update these files:**

1. `src/lib.rs` - Add mod declaration
2. `src/{module}.rs` - Create module implementation
3. `Cargo.toml` - Add dependencies if needed
4. `tests/{module}_tests.rs` - Create integration tests

### When Modifying API Structures

**MUST update these files:**

1. `src/models.rs` - Update data structures
2. `src/api.rs` - Update parsing logic
3. `tests/api_tests.rs` - Update test fixtures
4. `src/commands.rs` - Update command handlers using the data

### When Adding Dependencies

**MUST update these files:**

1. `Cargo.toml` - Add dependency
2. `src/lib.rs` - Add use statements if needed
3. **Verify compatibility** with wasm32-unknown-unknown target

## AI Decision-Making Standards

### Priority Order for Feature Implementation

1. **Authentication** - Login functionality first
2. **Data Retrieval** - Problem list and details
3. **File Management** - Solution templates and caching
4. **Testing** - Code execution and validation
5. **Submission** - Final code submission

### When Facing API Limitations

- **FIRST**: Try curl-based solution
- **SECOND**: Consider caching/offline approach
- **THIRD**: Implement graceful degradation
- **NEVER**: Skip functionality entirely

### Error Recovery Strategies

- **Network errors**: Retry with exponential backoff
- **Auth errors**: Prompt for re-authentication
- **File errors**: Create directories and retry
- **Parse errors**: Log details and return user-friendly message

## Prohibited Actions

### Absolute Prohibitions

- **NEVER use main.rs** for Extension code
- **NEVER use direct HTTP client crates** (reqwest, hyper, etc.)
- **NEVER create UI components** (TreeView, Webview, etc.)
- **NEVER hardcode sensitive data** (API keys, passwords)
- **NEVER skip test implementation** when adding features
- **NEVER modify extension.toml** without updating command handlers

### Development Anti-Patterns

- **NEVER commit without running tests**
- **NEVER implement functionality without corresponding tests**
- **NEVER use unwrap() in production code**
- **NEVER ignore curl command failures**
- **NEVER create files outside .leetcode directory**

### Testing Prohibitions

- **NEVER skip integration tests** for command flows
- **NEVER use real API calls** in tests
- **NEVER rely on external network** for test success
- **NEVER commit failing tests**

## Examples

### ✅ Correct Implementation

```rust
// Command handler with proper error handling
pub fn handle_login(args: Vec<String>) -> Result<SlashCommandOutput, String> {
    let session = parse_session_cookie(args)?;
    verify_session_with_curl(&session)
        .map_err(|e| format!("Login failed: {}", e))?;

    Ok(SlashCommandOutput {
        text: "Successfully logged in to LeetCode".to_string(),
        sections: vec![],
    })
}
```

### ❌ Incorrect Implementation

```rust
// NEVER do this - direct HTTP client usage
use reqwest;  // PROHIBITED

pub fn handle_login(args: Vec<String>) -> Result<SlashCommandOutput, String> {
    let client = reqwest::Client::new();  // PROHIBITED
    // ... rest of implementation
}
```

### ✅ Correct Test Pattern

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_success() {
        // Arrange
        let args = vec!["session_cookie_123".to_string()];

        // Act
        let result = handle_login(args);

        // Assert
        assert!(result.is_ok());
    }
}
```

This document provides project-specific rules for AI Agents working on the zed-leetcode Extension. Follow these rules strictly to maintain code quality and project consistency.
