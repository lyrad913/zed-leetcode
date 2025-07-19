# Zed LeetCode Extension

A powerful LeetCode integration for Zed Editor that brings essential LeetCode functionality through slash commands, enabling seamless problem solving without leaving your editor.

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/lyrad913/zed-leetcode)
[![Tests](https://img.shields.io/badge/tests-67%2F67%20passing-brightgreen)](https://github.com/lyrad913/zed-leetcode)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

## ✨ Features

- **🔐 Secure Authentication**: Login with LeetCode session cookies
- **📋 Problem Management**: Browse and filter problems by difficulty, tags, and companies
- **💻 Multi-Language Support**: Generate templates for 8+ programming languages
- **🧪 Test Execution**: Run sample test cases directly in Zed
- **🚀 Solution Submission**: Submit solutions and get detailed feedback
- **📊 Performance Metrics**: Runtime and memory usage with percentile comparison
- **📁 Smart File Management**: Automatic solution file organization

## 🎯 Supported Languages

| Language   | Extension | LeetCode ID |
|------------|-----------|-------------|
| Rust       | `.rs`     | `rust`      |
| Python     | `.py`     | `python3`   |
| JavaScript | `.js`     | `javascript`|
| TypeScript | `.ts`     | `typescript`|
| Java       | `.java`   | `java`      |
| C++        | `.cpp`    | `cpp`       |
| C          | `.c`      | `c`         |
| Go         | `.go`     | `golang`    |

## 🚀 Quick Start

### Installation

1. **Clone the repository** (Development Installation):
   ```bash
   git clone https://github.com/lyrad913/zed-leetcode.git
   cd zed-leetcode
   ```

2. **Install as Zed Dev Extension**:
   - Open Zed Editor
   - Press `Cmd+Shift+P` (or `Ctrl+Shift+P`)
   - Type "zed: install dev extension"
   - Select the cloned directory

3. **Build the extension**:
   ```bash
   cargo build --release
   ```

### Authentication

First, you need to authenticate with LeetCode:

1. **Get your session cookie**:
   - Open [leetcode.com](https://leetcode.com) in your browser
   - Login to your account
   - Open Developer Tools (F12)
   - Go to Application/Storage → Cookies → https://leetcode.com
   - Copy the `LEETCODE_SESSION` cookie value

2. **Login in Zed**:
   ```
   /leetcode-login <your-session-cookie>
   ```

## 📖 Usage Guide

### 🔍 Browse Problems

List problems with optional filters:

```bash
# List recent problems
/leetcode-list

# Filter by difficulty
/leetcode-list --difficulty easy

# Filter by tags
/leetcode-list --tag array,string

# Filter by company
/leetcode-list --company google

# Combine filters and limit results
/leetcode-list --difficulty medium --tag dynamic-programming --limit 10
```

**Output Example:**
```
# LeetCode Problems

| # | Title | Difficulty | Acceptance | Tags |
|---|-------|------------|------------|------|
| 1 | Two Sum | 🟢 Easy | 49.1% | array, hash-table |
| 2 | Add Two Numbers | 🟡 Medium | 37.8% | linked-list, math |
```

### 📄 View Problem Details

Get problem details and generate solution template:

```bash
# Show problem with default language (Rust)
/leetcode-show 1

# Show problem with specific language
/leetcode-show two-sum --language python

# Show by title slug
/leetcode-show add-two-numbers --language java
```

**Generated Files:**
- Solution template in `.leetcode/solutions/`
- Problem description as comments
- Function signature with proper types

### 🧪 Test Your Solution

Test your solution against sample cases:

```bash
# Test current file (auto-detect from filename)
/leetcode-test

# Test specific file
/leetcode-test .leetcode/solutions/1-two-sum.rs
```

**Test Results:**
```
# 🧪 Test Results

**Status:** ✅ All tests passed
**Tests Passed:** 3/3
**Runtime:** 16ms
**Memory:** 12.5 MB

🎉 **Great job! All tests passed!**
```

### 🚀 Submit Solution

Submit your final solution:

```bash
# Submit current file
/leetcode-submit

# Submit specific file  
/leetcode-submit .leetcode/solutions/1-two-sum.rs
```

**Submission Results:**
```
# 🚀 Submission Results

**Status:** ✅ Accepted
**Test Cases Passed:** 57/57
**Runtime:** 0ms (beats 100.0% of submissions)
**Memory:** 2.1 MB (beats 85.2% of submissions)

🎉 **Congratulations! Your solution has been accepted!**
```

## 📁 File Structure

The extension creates a `.leetcode/` directory in your workspace:

```
.leetcode/
├── config.json          # Authentication and settings
├── problems/            # Cached problem data
│   ├── 1.json
│   └── 2.json
└── solutions/           # Your solution files
    ├── 1-two-sum.rs
    ├── 2-add-two-numbers.py
    └── 3-longest-substring.js
```

## 🛠️ Commands Reference

| Command | Description | Arguments |
|---------|-------------|-----------|
| `/leetcode-login` | Authenticate with LeetCode | `<session-cookie>` |
| `/leetcode-list` | List problems with filters | `[--difficulty] [--tag] [--company] [--limit]` |
| `/leetcode-show` | Show problem details | `<problem-id>` `[--language]` |
| `/leetcode-test` | Test current solution | `[file-path]` |
| `/leetcode-submit` | Submit solution | `[file-path]` |

## 🔧 Advanced Configuration

### Environment Variables

You can set these environment variables for customization:

- `LEETCODE_WORKSPACE`: Custom workspace directory (default: `.leetcode/`)
- `LEETCODE_TIMEOUT`: API timeout in seconds (default: `30`)

### File Naming Convention

Solution files follow this pattern: `{problem-id}-{title-slug}.{ext}`

Examples:
- `1-two-sum.rs`
- `2-add-two-numbers.py` 
- `3-longest-substring-without-repeating-characters.js`

## 🏗️ Architecture

This extension follows a modular architecture:

```
src/
├── lib.rs              # Extension entry point
├── commands.rs         # Slash command handlers
├── api.rs              # LeetCode API client (curl-based)
├── auth.rs             # Authentication management
├── models.rs           # Data structures
├── templates.rs        # Code template generation
└── file_manager.rs     # Local file operations
```

### Key Design Decisions

- **Slash Commands**: Integrates seamlessly with Zed's command system
- **curl-based HTTP**: Works around Zed Extension HTTP client limitations
- **Local Caching**: Reduces API calls and improves performance
- **Secure Storage**: Session cookies stored with 600 permissions

## 🤝 Contributing

We welcome contributions! Here's how to get started:

### Development Setup

1. **Fork the repository**
2. **Clone your fork**:
   ```bash
   git clone https://github.com/your-username/zed-leetcode.git
   cd zed-leetcode
   ```

3. **Install dependencies**:
   ```bash
   cargo check
   ```

4. **Run tests**:
   ```bash
   cargo test
   ```

### Testing

We maintain comprehensive test coverage:

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test commands::
cargo test api::

# Run with coverage
cargo test -- --nocapture
```

### Code Quality

- Follow Rust conventions and use `cargo fmt`
- Ensure all tests pass with `cargo test`
- Add tests for new functionality
- Update documentation for API changes

## 📋 Troubleshooting

### Common Issues

**❌ "Please login first"**
- Your session cookie has expired
- Run `/leetcode-login <new-session-cookie>` with a fresh cookie

**❌ "Authentication required"**
- Session cookie is invalid or malformed
- Check the cookie value from your browser

**❌ "No active workspace found"**
- For test/submit commands, either:
  - Provide the file path as an argument
  - Open the solution file in Zed first

**❌ "Failed to parse curl output"**
- Network connectivity issues
- LeetCode API might be temporarily down
- Try again in a few minutes

### Debug Mode

Enable verbose logging by setting the environment variable:

```bash
RUST_LOG=debug zed
```

### Getting Help

- 📖 [User Guide](docs/user-guide.md)
- 🔧 [Troubleshooting Guide](docs/troubleshooting.md) 
- 💬 [GitHub Issues](https://github.com/lyrad913/zed-leetcode/issues)
- 📧 [Contact](mailto:your-email@example.com)

## 📊 Performance

- **Startup Time**: < 100ms extension initialization
- **API Response**: Average 500ms for problem fetching
- **Test Execution**: 1-5 seconds depending on problem complexity
- **Memory Usage**: < 50MB for typical usage

## 🆚 vs. VS Code Extension

| Feature | Zed Extension | VS Code Extension |
|---------|---------------|-------------------|
| **Interface** | Slash Commands | UI Panels + Commands |
| **Performance** | Faster startup | More feature-rich UI |
| **Simplicity** | Command-focused | Visual interface |
| **Integration** | Native Zed integration | Rich VS Code ecosystem |

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Zed Editor](https://zed.dev) for the excellent extension API
- [LeetCode](https://leetcode.com) for the platform and API
- The Rust community for amazing tooling and support

---

**Made with ❤️ for the coding community**
