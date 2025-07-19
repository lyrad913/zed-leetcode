# User Guide - Zed LeetCode Extension

Complete guide for using the Zed LeetCode Extension effectively.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Authentication Setup](#authentication-setup)
3. [Command Reference](#command-reference)
4. [Workflow Examples](#workflow-examples)
5. [File Management](#file-management)
6. [Best Practices](#best-practices)
7. [Tips and Tricks](#tips-and-tricks)

## Getting Started

### Prerequisites

- **Zed Editor**: Version 0.100.0 or later
- **Rust**: Version 1.70 or later (for building)
- **Active Internet Connection**: For LeetCode API access
- **LeetCode Account**: Free or premium account

### Installation Steps

1. **Development Installation**:
   ```bash
   git clone https://github.com/lyrad913/zed-leetcode.git
   cd zed-leetcode
   cargo build --release
   ```

2. **Install in Zed**:
   - Open Zed Editor
   - Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
   - Type "zed: install dev extension"
   - Navigate to the `zed-leetcode` directory
   - Confirm installation

3. **Verify Installation**:
   - Open any workspace in Zed
   - Type `/leetcode-` in the command palette
   - You should see all available commands

## Authentication Setup

### Method 1: Browser Cookie Extraction

1. **Open LeetCode**:
   - Navigate to [leetcode.com](https://leetcode.com)
   - Log in to your account

2. **Extract Session Cookie**:
   - Open Developer Tools (`F12` or right-click → Inspect)
   - Go to **Application** tab (Chrome) or **Storage** tab (Firefox)
   - Navigate to **Cookies** → `https://leetcode.com`
   - Find `LEETCODE_SESSION` cookie
   - Copy the **Value** (long string of characters)

3. **Login in Zed**:
   ```
   /leetcode-login eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...
   ```

### Method 2: Network Inspector

1. **Monitor Network Requests**:
   - Open Developer Tools → Network tab
   - Refresh LeetCode page
   - Look for any request to `leetcode.com`
   - Check request headers for `Cookie` header
   - Extract `LEETCODE_SESSION` value

### Session Management

- **Session Duration**: Typically lasts 30 days
- **Renewal**: Re-run `/leetcode-login` with fresh cookie when expired
- **Security**: Cookies are stored securely with 600 file permissions
- **Multiple Accounts**: Only one session active at a time

## Command Reference

### `/leetcode-login <session-cookie>`

**Purpose**: Authenticate with LeetCode using session cookie

**Parameters**:
- `session-cookie`: LeetCode session cookie value

**Examples**:
```bash
/leetcode-login eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJfYXV0aF91c2VyX2lkIjoiMTIzNDU2NyIsIl9hdXRoX3VzZXJfYmFja2VuZCI6ImFsbGF1dGguYWNjb3VudC5hdXRoX2JhY2tlbmRzLkF1dGhlbnRpY2F0aW9uQmFja2VuZCIsIl9hdXRoX3VzZXJfaGFzaCI6IjY4NGM5NmZmNDg5MWU0ZmY4MjkyYzM5OTIyNjQ5OWEwNjBhYWEzNzQiLCJpZCI6MTIzNDU2NywiZW1haWwiOiJ1c2VyQGV4YW1wbGUuY29tIiwidXNlcm5hbWUiOiJ1c2VyMTIzIiwidXNlcl9zbHVnIjoidXNlcjEyMyIsImF2YXRhciI6Imh0dHBzOi8vYXNzZXRzLmxlZXRjb2RlLmNvbS91c2Vycy9kZWZhdWx0X2F2YXRhci5qcGciLCJyZWZyZXNoZWRfYXQiOjE2MzIxNTY0ODAsImlwIjoiMTkyLjE2OC4xLjEwMCIsImlkZW50aXR5IjoiZjc4ZTkxMjM0NTY3ODkwIiwic2Vzc2lvbl9pZCI6MTA5ODc2NTQzfQ.yOXKr9Zf8QZYUmW8BjvzKFrw9VgLJZoYfBQRlm2LiJs
```

**Success Response**:
```
Successfully logged in to LeetCode! Session saved securely.
```

**Error Cases**:
- Invalid cookie format
- Expired session
- Network connectivity issues

### `/leetcode-list [options]`

**Purpose**: Browse and filter LeetCode problems

**Options**:
- `--difficulty <easy|medium|hard>`: Filter by difficulty level
- `--tag <tag1,tag2>`: Filter by problem tags (comma-separated)
- `--company <company>`: Filter by company tag
- `--limit <number>`: Limit number of results (default: 50)

**Examples**:
```bash
# Basic usage
/leetcode-list

# Filter by difficulty
/leetcode-list --difficulty easy
/leetcode-list --difficulty medium

# Filter by tags
/leetcode-list --tag array
/leetcode-list --tag array,string,hash-table

# Filter by company
/leetcode-list --company google
/leetcode-list --company amazon

# Combine filters
/leetcode-list --difficulty medium --tag dynamic-programming --limit 20
/leetcode-list --company microsoft --difficulty hard --limit 5
```

**Output Format**:
```markdown
# LeetCode Problems

**Filters applied:**
- Difficulty: Medium
- Tags: dynamic-programming

| # | Title | Difficulty | Acceptance | Tags |
|---|-------|------------|------------|------|
| 5 | Longest Palindromic Substring | 🟡 Medium | 32.8% | string, dynamic-programming |
| 62 | Unique Paths | 🟡 Medium | 62.7% | math, dynamic-programming |

**Showing 15 problems**
```

### `/leetcode-show <problem-id> [options]`

**Purpose**: Display problem details and generate solution template

**Parameters**:
- `problem-id`: Problem number (e.g., `1`) or title slug (e.g., `two-sum`)

**Options**:
- `--language <lang>`: Programming language for template (default: rust)

**Supported Languages**:
- `rust`, `python`, `javascript`, `typescript`, `java`, `cpp`, `c`, `golang`

**Examples**:
```bash
# Show with default language (Rust)
/leetcode-show 1
/leetcode-show two-sum

# Show with specific language
/leetcode-show 1 --language python
/leetcode-show longest-substring --language java
/leetcode-show 15 --language cpp
```

**Generated Template Example (Python)**:
```python
"""
1. Two Sum

Given an array of integers nums and an integer target, return indices of the 
two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may 
not use the same element twice.

Example 1:
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
"""

from typing import List

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        # Your solution here
        pass
```

**File Location**: `.leetcode/solutions/1-two-sum.py`

### `/leetcode-test [file-path]`

**Purpose**: Test solution against sample test cases

**Parameters**:
- `file-path` (optional): Path to solution file. If omitted, provide as argument.

**Examples**:
```bash
# Test by providing file path
/leetcode-test .leetcode/solutions/1-two-sum.rs
/leetcode-test /full/path/to/solution.py

# Test current file (if you have the solution file open)
/leetcode-test
```

**Success Output**:
```markdown
# 🧪 Test Results

**Status:** ✅ All tests passed
**Tests Passed:** 3/3
**Runtime:** 16ms
**Memory:** 12.5 MB

🎉 **Great job! All tests passed!**

You can now submit your solution using `/leetcode-submit`
```

**Failure Output**:
```markdown
# 🧪 Test Results

**Status:** ❌ Wrong Answer
**Tests Passed:** 2/3

## Failed Test Case

```
Input: [3,2,4]
target: 6
Expected: [1,2]
Actual: [0,2]
```
```

**Error Types**:
- ✅ Success
- ❌ Wrong Answer  
- ❌ Compile Error
- ❌ Runtime Error
- ⏰ Time Limit Exceeded
- 💾 Memory Limit Exceeded

### `/leetcode-submit [file-path]`

**Purpose**: Submit solution to LeetCode for final evaluation

**Parameters**:
- `file-path` (optional): Path to solution file

**Examples**:
```bash
# Submit specific file
/leetcode-submit .leetcode/solutions/1-two-sum.rs

# Submit current file
/leetcode-submit
```

**Accepted Solution**:
```markdown
# 🚀 Submission Results

**Status:** ✅ Accepted
**Test Cases Passed:** 57/57
**Runtime:** 0ms (beats 100.0% of submissions)
**Memory:** 2.1 MB (beats 85.2% of submissions)

🎉 **Congratulations! Your solution has been accepted!**

✨ Your code successfully passed all test cases. Well done!
```

**Failed Solution**:
```markdown
# 🚀 Submission Results

**Status:** ❌ Wrong Answer
**Test Cases Passed:** 55/57

## Failed Test Case

```
Input: [1,2,3,4,5,6,7]
target: 13
Expected: [5,6]
Actual: [4,6]
```

💡 **Keep trying! Every attempt brings you closer to the solution.**

**Hint:** Check the failed test case above and trace through your logic.
```

## Workflow Examples

### Complete Problem-Solving Workflow

1. **Browse Problems**:
   ```bash
   /leetcode-list --difficulty easy --limit 10
   ```

2. **Select and View Problem**:
   ```bash
   /leetcode-show 1 --language python
   ```

3. **Code Your Solution**:
   - Open generated file: `.leetcode/solutions/1-two-sum.py`
   - Implement your algorithm

4. **Test Solution**:
   ```bash
   /leetcode-test .leetcode/solutions/1-two-sum.py
   ```

5. **Debug if Needed**:
   - Review failed test cases
   - Modify your code
   - Re-test

6. **Submit Final Solution**:
   ```bash
   /leetcode-submit .leetcode/solutions/1-two-sum.py
   ```

### Daily Practice Routine

1. **Morning Warm-up**:
   ```bash
   /leetcode-list --difficulty easy --limit 5
   ```

2. **Skill Building**:
   ```bash
   /leetcode-list --tag array --difficulty medium --limit 3
   ```

3. **Interview Prep**:
   ```bash
   /leetcode-list --company faang --difficulty hard --limit 2
   ```

### Contest Preparation

1. **Algorithm Focus**:
   ```bash
   /leetcode-list --tag dynamic-programming --limit 20
   /leetcode-list --tag graph --limit 15  
   ```

2. **Time-based Practice**:
   - Use `/leetcode-test` for quick validation
   - Focus on `/leetcode-submit` for final timing

## File Management

### Directory Structure

```
workspace/
├── .leetcode/
│   ├── config.json          # Authentication config
│   ├── problems/            # Cached problem data
│   │   ├── 1.json          # Problem 1 metadata
│   │   ├── 2.json          # Problem 2 metadata
│   │   └── ...
│   └── solutions/          # Your solution files
│       ├── 1-two-sum.rs
│       ├── 2-add-two-numbers.py
│       ├── 3-longest-substring.js
│       └── ...
└── your-other-files...
```

### File Naming Conventions

**Pattern**: `{problem-id}-{title-slug}.{extension}`

**Examples**:
- `1-two-sum.rs` → Problem #1, Rust
- `2-add-two-numbers.py` → Problem #2, Python  
- `146-lru-cache.java` → Problem #146, Java

### Automatic File Generation

When you run `/leetcode-show`, the extension:

1. Creates `.leetcode/solutions/` directory if needed
2. Generates appropriately named file
3. Includes problem description as comments
4. Provides language-specific template
5. Includes necessary imports/includes

### Manual File Organization

You can organize files manually:

```bash
# Create topic-based directories
mkdir .leetcode/solutions/arrays
mkdir .leetcode/solutions/dynamic-programming

# Move files by topic
mv .leetcode/solutions/1-two-sum.rs .leetcode/solutions/arrays/
mv .leetcode/solutions/5-longest-palindrome.py .leetcode/solutions/dynamic-programming/
```

Just ensure you use full paths when testing/submitting:
```bash
/leetcode-test .leetcode/solutions/arrays/1-two-sum.rs
```

## Best Practices

### Authentication Management

✅ **Do**:
- Store session cookies securely
- Refresh cookies before expiration  
- Use private browsing for cookie extraction
- Log out from shared computers

❌ **Don't**:
- Share session cookies with others
- Store cookies in plain text files
- Use expired or invalid cookies
- Leave sessions open on public computers

### Code Organization

✅ **Do**:
- Use consistent naming conventions
- Add comments explaining your approach
- Keep template structure intact
- Test before submitting

❌ **Don't**:
- Modify generated file structure drastically
- Remove problem description comments
- Skip testing phase
- Submit without understanding the solution

### Efficient Problem Solving

✅ **Do**:
- Start with easier problems
- Focus on one topic at a time
- Use filters to find relevant problems
- Review failed test cases carefully
- Learn from submission feedback

❌ **Don't**:
- Jump to hard problems immediately
- Ignore test failures
- Submit without testing
- Skip understanding problem constraints

### Performance Optimization

✅ **Do**:
- Cache problems locally (automatic)
- Use appropriate data structures
- Consider time/space complexity
- Benchmark critical sections

❌ **Don't**:
- Fetch same problem repeatedly
- Ignore performance metrics
- Use brute force for large inputs
- Neglect memory usage

## Tips and Tricks

### Keyboard Shortcuts

- `Cmd+Shift+P` → Command palette (all commands)
- Type `/leetcode-` → Quick access to all commands
- Use tab completion for file paths

### Problem Discovery

```bash
# Find problems you haven't solved
/leetcode-list --status TODO

# Explore trending problems
/leetcode-list --limit 20

# Company-specific preparation
/leetcode-list --company google --difficulty medium
```

### Code Templates

The extension provides optimized templates:

**Python**:
- Includes proper type hints
- Pre-imports common modules (`List`, `Dict`, etc.)
- Follows PEP 8 style guidelines

**Rust**:
- Uses standard library efficiently
- Includes necessary `use` statements
- Follows Rust naming conventions

**Java**:
- Proper class structure
- Import statements for collections
- Follows Java conventions

### Testing Strategy

1. **Test Early**: Run tests after implementing basic logic
2. **Edge Cases**: Consider empty inputs, single elements, etc.
3. **Performance**: Watch for timeout on large inputs
4. **Debug Output**: Add print statements for debugging

### Submission Strategy

1. **Final Test**: Always test before submitting
2. **Clean Code**: Remove debug print statements
3. **Comment Clarity**: Ensure code is readable
4. **Understand Results**: Review performance metrics

### Troubleshooting Quick Reference

| Issue | Solution |
|-------|----------|
| "Please login first" | Run `/leetcode-login` with fresh cookie |
| "Authentication required" | Check cookie validity and format |
| "No active workspace" | Provide file path as argument |
| "Failed to parse output" | Check network connection |
| Test failures | Review failed test case details |
| Slow performance | Check algorithm complexity |

---

**Need more help?** Check the [Troubleshooting Guide](troubleshooting.md) or [open an issue](https://github.com/lyrad913/zed-leetcode/issues).
