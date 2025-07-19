# Developer Guide - Zed LeetCode Extension

Contributing and development guidelines for the Zed LeetCode Extension.

## Table of Contents

1. [Development Environment Setup](#development-environment-setup)
2. [Project Architecture](#project-architecture)
3. [Code Guidelines](#code-guidelines)
4. [Testing Strategy](#testing-strategy)
5. [Adding New Features](#adding-new-features)
6. [API Integration](#api-integration)
7. [Debugging Tips](#debugging-tips)
8. [Contributing Guidelines](#contributing-guidelines)

## Development Environment Setup

### Prerequisites

- **Rust**: Version 1.70 or later
- **Zed Editor**: Latest version
- **Git**: For version control
- **curl**: For HTTP requests (system dependency)

### Initial Setup

1. **Clone Repository**:
   ```bash
   git clone https://github.com/lyrad913/zed-leetcode.git
   cd zed-leetcode
   ```

2. **Install Rust Toolchain**:
   ```bash
   rustup update
   rustup target add wasm32-unknown-unknown
   ```

3. **Install Dependencies**:
   ```bash
   cargo build
   ```

4. **Run Tests**:
   ```bash
   cargo test
   ```

### Development Build

```bash
# Debug build
cargo build

# Release build (for production)
cargo build --release --target wasm32-unknown-unknown

# Test build
cargo test --all
```

### Installing for Development

```bash
# Build and install in Zed
cargo build --release --target wasm32-unknown-unknown

# In Zed Editor:
# Command Palette → "zed: install dev extension"
# Navigate to project directory
```

## Project Architecture

### Module Structure

```
src/
├── lib.rs              # Entry point and command routing
├── commands.rs         # Slash command implementations
├── api.rs             # LeetCode API client
├── auth.rs            # Authentication management
├── file_manager.rs    # File system operations
├── models.rs          # Data structures and models
└── templates.rs       # Code template generation
```

### Data Flow

```
User Command → lib.rs → commands.rs → api.rs → LeetCode API
                ↓
         file_manager.rs → Local Storage (.leetcode/)
```

### Key Components

#### 1. Extension Entry Point (`lib.rs`)

```rust
#[export_name = "run_slash_command"]
pub extern "C" fn run_slash_command(
    command_id: *const c_char,
    args: *const c_char,
    worktree: *mut c_void,
) -> *mut c_char {
    // Command routing and execution
}
```

#### 2. Command Handlers (`commands.rs`)

```rust
pub fn handle_login(args: Vec<String>) -> Result<SlashCommandOutput, String> {
    // Authentication logic
}

pub fn handle_list(args: Vec<String>) -> Result<SlashCommandOutput, String> {
    // Problem listing logic
}
```

#### 3. API Client (`api.rs`)

```rust
pub struct LeetCodeApi {
    base_url: String,
    session_cookie: Option<String>,
}

impl LeetCodeApi {
    pub fn verify_session(&self) -> Result<bool, Box<dyn Error>> {
        // Session verification via curl
    }
}
```

### Configuration Management

**Config Structure**:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub session_cookie: Option<String>,
    pub last_updated: String,
}
```

**Storage Location**: `.leetcode/config.json`

**Security**: 600 permissions for sensitive data

## Code Guidelines

### Coding Standards

#### Error Handling

```rust
// Use Result for error propagation
pub fn fetch_problems() -> Result<Vec<Problem>, Box<dyn Error>> {
    // Implementation
}

// Convert errors for user-facing messages
pub fn handle_command(args: Vec<String>) -> Result<SlashCommandOutput, String> {
    fetch_problems()
        .map_err(|e| format!("Failed to fetch problems: {}", e))?;
    
    // Success case
    Ok(SlashCommandOutput {
        text: "Success".to_string(),
        sections: vec![],
    })
}
```

#### HTTP Requests Pattern

```rust
// Always use curl, never direct HTTP clients
fn make_request(url: &str, data: &str) -> Result<String, Box<dyn Error>> {
    let output = std::process::Command::new("curl")
        .args(["-s", "-X", "POST", url])
        .args(["-H", "Content-Type: application/json"])
        .args(["-d", data])
        .output()?;
    
    if !output.status.success() {
        return Err(format!("Request failed: {}", 
            String::from_utf8_lossy(&output.stderr)).into());
    }
    
    Ok(String::from_utf8(output.stdout)?)
}
```

#### File Operations

```rust
use crate::file_manager::FileManager;

// Always use FileManager for file operations
fn save_problem_data(problem: &Problem) -> Result<(), Box<dyn Error>> {
    let file_manager = FileManager::new()?;
    file_manager.save_problem_cache(&problem.id.to_string(), problem)?;
    Ok(())
}
```

### Naming Conventions

- **Functions**: `snake_case`
- **Structs**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`
- **Files**: `snake_case.rs`

### Documentation Standards

```rust
/// Fetches problem list from LeetCode API with optional filtering
/// 
/// # Arguments
/// 
/// * `difficulty` - Filter by difficulty level (easy/medium/hard)
/// * `tags` - Filter by problem tags
/// * `limit` - Maximum number of problems to return
/// 
/// # Returns
/// 
/// * `Result<Vec<Problem>, Box<dyn Error>>` - List of problems or error
/// 
/// # Examples
/// 
/// ```
/// let problems = api.fetch_problem_list(Some("easy"), None, Some(10))?;
/// ```
pub fn fetch_problem_list(
    &self,
    difficulty: Option<&str>,
    tags: Option<&str>,
    limit: Option<usize>,
) -> Result<Vec<Problem>, Box<dyn Error>> {
    // Implementation
}
```

## Testing Strategy

### Test Types

1. **Unit Tests**: Individual function testing
2. **Integration Tests**: Multi-module interaction
3. **Command Tests**: Full command workflows
4. **Mock Tests**: External API simulation

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_config_creation() {
        let temp_dir = TempDir::new().unwrap();
        let file_manager = FileManager::new_with_root(temp_dir.path()).unwrap();
        
        // Test implementation
        assert!(file_manager.config_path().exists());
    }
    
    #[test]
    fn test_problem_caching() {
        // Test caching logic
    }
}
```

### Mock Implementation

```rust
// Mock API responses for testing
pub struct MockApi {
    responses: HashMap<String, String>,
}

impl MockApi {
    pub fn new() -> Self {
        let mut responses = HashMap::new();
        responses.insert(
            "problems".to_string(),
            include_str!("../tests/fixtures/problems.json").to_string()
        );
        
        Self { responses }
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific module
cargo test auth

# Integration tests only
cargo test --test integration

# With output
cargo test -- --nocapture
```

## Adding New Features

### Command Implementation Pattern

1. **Add to extension.toml**:
```toml
[[slash_command]]
name = "new-command"
description = "Description of new command"
```

2. **Add route in lib.rs**:
```rust
match command_id_str {
    "new-command" => commands::handle_new_command(args),
    // existing commands...
}
```

3. **Implement handler in commands.rs**:
```rust
pub fn handle_new_command(args: Vec<String>) -> Result<SlashCommandOutput, String> {
    // Parse arguments
    let parsed_args = parse_args(&args)?;
    
    // Execute logic
    let result = execute_new_command_logic(parsed_args)?;
    
    // Format output
    Ok(SlashCommandOutput {
        text: format_result(&result),
        sections: vec![],
    })
}

fn execute_new_command_logic(args: ParsedArgs) -> Result<CommandResult, Box<dyn Error>> {
    // Implementation
}
```

4. **Add tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_command_success() {
        let args = vec!["arg1".to_string(), "arg2".to_string()];
        let result = handle_new_command(args);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_new_command_invalid_args() {
        let args = vec![];
        let result = handle_new_command(args);
        assert!(result.is_err());
    }
}
```

### API Endpoint Integration

```rust
impl LeetCodeApi {
    pub fn new_api_method(&self, params: &ApiParams) -> Result<ApiResponse, Box<dyn Error>> {
        let query = json!({
            "query": "query { ... }",
            "variables": params
        });
        
        let response = self.make_graphql_request(&query.to_string())?;
        let parsed: ApiResponse = serde_json::from_str(&response)?;
        
        Ok(parsed)
    }
}
```

### Model Definition

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct NewModel {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

impl NewModel {
    pub fn new(id: u32, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
```

## API Integration

### GraphQL Query Pattern

```rust
const PROBLEM_LIST_QUERY: &str = r#"
    query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
        problemsetQuestionList: questionList(
            categorySlug: $categorySlug
            limit: $limit
            skip: $skip
            filters: $filters
        ) {
            total: totalNum
            questions: data {
                acRate
                difficulty
                freqBar
                frontendQuestionId: questionFrontendId
                isFavor
                paidOnly: isPaidOnly
                status
                title
                titleSlug
                topicTags {
                    name
                    id
                    slug
                }
            }
        }
    }
"#;
```

### Request Implementation

```rust
fn make_graphql_request(&self, query: &str) -> Result<String, Box<dyn Error>> {
    let mut cmd = std::process::Command::new("curl");
    cmd.args(["-s", "-X", "POST", &self.base_url]);
    cmd.args(["-H", "Content-Type: application/json"]);
    
    if let Some(cookie) = &self.session_cookie {
        cmd.args(["-H", &format!("Cookie: LEETCODE_SESSION={}", cookie)]);
    }
    
    cmd.args(["-d", query]);
    
    let output = cmd.output()?;
    
    if !output.status.success() {
        return Err(format!("HTTP request failed: {}", 
            String::from_utf8_lossy(&output.stderr)).into());
    }
    
    Ok(String::from_utf8(output.stdout)?)
}
```

### Response Parsing

```rust
#[derive(Debug, Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
struct GraphQLError {
    message: String,
}

fn parse_response<T: for<'de> Deserialize<'de>>(
    response: &str
) -> Result<T, Box<dyn Error>> {
    let parsed: GraphQLResponse<T> = serde_json::from_str(response)?;
    
    if let Some(errors) = parsed.errors {
        return Err(format!("GraphQL errors: {:?}", errors).into());
    }
    
    parsed.data
        .ok_or_else(|| "No data in response".into())
}
```

## Debugging Tips

### Extension Debugging

1. **Add Debug Prints**:
```rust
eprintln!("Debug: Processing command with args: {:?}", args);
```

2. **Check Extension Logs**:
   - Look in Zed's developer console
   - System logs if available

3. **Test Isolated Components**:
```rust
#[cfg(test)]
mod debug_tests {
    use super::*;
    
    #[test]
    fn debug_api_call() {
        let api = LeetCodeApi::new();
        let result = api.verify_session();
        println!("API result: {:?}", result);
    }
}
```

### Network Debugging

```bash
# Test curl command directly
curl -v -X POST https://leetcode.com/graphql \
  -H "Content-Type: application/json" \
  -H "Cookie: LEETCODE_SESSION=your_session" \
  -d '{"query": "query { user { username } }"}'
```

### File System Debugging

```rust
fn debug_file_operations() {
    let file_manager = FileManager::new().unwrap();
    eprintln!("Config path: {:?}", file_manager.config_path());
    eprintln!("Solutions dir: {:?}", file_manager.solutions_dir());
    eprintln!("Config exists: {}", file_manager.config_path().exists());
}
```

## Contributing Guidelines

### Pull Request Process

1. **Fork and Clone**:
```bash
git clone https://github.com/YOUR_USERNAME/zed-leetcode.git
cd zed-leetcode
```

2. **Create Feature Branch**:
```bash
git checkout -b feature/your-feature-name
```

3. **Implement Changes**:
   - Write tests first (TDD)
   - Implement feature
   - Ensure all tests pass

4. **Commit Changes**:
```bash
git add .
git commit -m "feat(scope): description

Detailed description of changes.

Closes #123"
```

5. **Push and Create PR**:
```bash
git push origin feature/your-feature-name
```

### Code Review Checklist

- [ ] Tests added for new functionality
- [ ] All existing tests pass
- [ ] Documentation updated
- [ ] Error handling implemented
- [ ] Security considerations addressed
- [ ] Performance impact assessed

### Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types**: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`
**Scopes**: `auth`, `api`, `commands`, `files`, `tests`

### Examples

```bash
# Feature addition
feat(commands): add problem filtering by company tags

Implement --company flag for leetcode-list command.
Supports filtering by major tech companies.

Closes #45

# Bug fix
fix(auth): handle expired session cookies gracefully

Previously, expired cookies caused panic.
Now returns user-friendly error message.

Fixes #67

# Test addition
test(api): add integration tests for problem fetching

Cover success cases, network errors, and malformed responses.
Improves test coverage to 95%.
```

### Development Workflow

1. **Issue Discussion**: Discuss feature/bug in GitHub issues
2. **Implementation**: Follow TDD approach
3. **Code Review**: Submit PR for review
4. **Testing**: Ensure comprehensive test coverage
5. **Documentation**: Update relevant docs
6. **Release**: Maintainer handles versioning and release

---

**Ready to contribute?** Check out [open issues](https://github.com/lyrad913/zed-leetcode/issues) or propose new features!
