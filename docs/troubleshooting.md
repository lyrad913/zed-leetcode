# Troubleshooting Guide - Zed LeetCode Extension

Solutions to common issues and problems you might encounter.

## Table of Contents

1. [Authentication Issues](#authentication-issues)
2. [Network and Connectivity Problems](#network-and-connectivity-problems)
3. [File System and Permissions](#file-system-and-permissions)
4. [Command Execution Issues](#command-execution-issues)
5. [Testing and Submission Problems](#testing-and-submission-problems)
6. [Performance Issues](#performance-issues)
7. [Development and Building Problems](#development-and-building-problems)
8. [Extension Installation Issues](#extension-installation-issues)

## Authentication Issues

### ❌ "Please login first to use LeetCode commands"

**Symptoms**: All commands return authentication required error

**Root Causes**:
- No session stored in config
- Session expired
- Invalid session format
- Corrupted config file

**Solutions**:

1. **Re-authenticate**:
   ```bash
   /leetcode-login YOUR_SESSION_COOKIE_HERE
   ```

2. **Verify Cookie Format**:
   - Should start with `eyJ` or similar
   - Should be a long string (500+ characters)
   - No spaces or line breaks

3. **Clear and Re-authenticate**:
   ```bash
   # Remove config file
   rm ~/.leetcode/config.json
   # Re-login
   /leetcode-login YOUR_NEW_SESSION_COOKIE
   ```

### ❌ "Failed to authenticate with LeetCode"

**Symptoms**: Login command fails during verification

**Root Causes**:
- Expired or invalid session cookie
- Network connectivity issues
- LeetCode API changes
- Incorrect cookie format

**Solutions**:

1. **Extract Fresh Cookie**:
   - Open new private browser window
   - Log into LeetCode
   - Extract `LEETCODE_SESSION` cookie value
   - Retry login

2. **Verify Cookie Source**:
   ```javascript
   // In browser console (leetcode.com)
   document.cookie.split(';')
     .find(row => row.startsWith('LEETCODE_SESSION='))
     ?.split('=')[1]
   ```

3. **Check Network Connection**:
   ```bash
   # Test connectivity
   curl -s https://leetcode.com/api/problems/all/ | head -10
   ```

### ❌ "Session cookie format is invalid"

**Symptoms**: Cookie validation fails immediately

**Root Causes**:
- Malformed cookie string
- Copied additional characters
- Missing cookie parts

**Solutions**:

1. **Clean Cookie String**:
   - Remove any quotes, spaces, or newlines
   - Ensure it's exactly the cookie value
   - Should look like: `eyJ0eXAiOiJKV1QiLCJhbGci...`

2. **Use Different Extraction Method**:
   - Try Network tab method instead of Application tab
   - Look for `Set-Cookie` headers in responses
   - Use browser extension for cookie extraction

## Network and Connectivity Problems

### ❌ "Network request failed" / "Connection timeout"

**Symptoms**: Commands fail with network errors

**Root Causes**:
- Internet connectivity issues
- Corporate firewall blocking requests
- DNS resolution problems
- LeetCode server issues

**Solutions**:

1. **Test Basic Connectivity**:
   ```bash
   ping leetcode.com
   curl -I https://leetcode.com
   ```

2. **Check Firewall/Proxy**:
   ```bash
   # Test with proxy if needed
   export https_proxy=http://your-proxy:port
   /leetcode-list
   ```

3. **DNS Issues**:
   ```bash
   # Try different DNS
   nslookup leetcode.com 8.8.8.8
   ```

4. **Wait and Retry**:
   - LeetCode may be experiencing downtime
   - Check [LeetCode status](https://status.leetcode.com/)

### ❌ "Rate limit exceeded"

**Symptoms**: Commands work initially, then start failing

**Root Causes**:
- Too many API requests in short time
- LeetCode rate limiting
- Shared IP restrictions

**Solutions**:

1. **Wait Before Retry**:
   - Wait 1-5 minutes between requests
   - Reduce command frequency

2. **Use Cached Data**:
   - Problems are cached in `.leetcode/problems/`
   - Use cached data when possible

3. **Authenticate Properly**:
   - Authenticated requests have higher limits
   - Ensure login is successful

## File System and Permissions

### ❌ "Permission denied" / "Failed to create file"

**Symptoms**: Cannot create or modify files in `.leetcode` directory

**Root Causes**:
- Insufficient file system permissions
- Read-only workspace
- Disk space issues
- Directory doesn't exist

**Solutions**:

1. **Check Permissions**:
   ```bash
   ls -la .leetcode/
   # Should show writable permissions
   ```

2. **Fix Permissions**:
   ```bash
   chmod 755 .leetcode/
   chmod 644 .leetcode/config.json
   chmod 755 .leetcode/solutions/
   ```

3. **Check Disk Space**:
   ```bash
   df -h .
   # Ensure sufficient space available
   ```

4. **Recreate Directory**:
   ```bash
   rm -rf .leetcode/
   # Re-run any leetcode command to recreate
   ```

### ❌ "Config file is corrupted"

**Symptoms**: Commands fail with JSON parsing errors

**Root Causes**:
- Malformed JSON in config.json
- File corruption
- Encoding issues

**Solutions**:

1. **Check Config File**:
   ```bash
   cat .leetcode/config.json
   # Should be valid JSON
   ```

2. **Validate JSON**:
   ```bash
   python -m json.tool .leetcode/config.json
   # Will show syntax errors if any
   ```

3. **Reset Configuration**:
   ```bash
   rm .leetcode/config.json
   /leetcode-login YOUR_SESSION_COOKIE
   ```

## Command Execution Issues

### ❌ "Command not found" / "Unknown command"

**Symptoms**: Zed doesn't recognize leetcode commands

**Root Causes**:
- Extension not installed properly
- Extension not activated
- Zed version incompatibility

**Solutions**:

1. **Verify Installation**:
   - Check if extension appears in Extensions panel
   - Look for `extension.toml` in extension directory

2. **Restart Zed**:
   - Close and reopen Zed Editor
   - Extensions may need restart to activate

3. **Check Extension Status**:
   ```bash
   # In extension directory
   ls -la
   # Should see Cargo.toml, src/, extension.toml
   ```

### ❌ "No active workspace"

**Symptoms**: Commands require file paths but none provided

**Root Causes**:
- No workspace open in Zed
- Running commands outside workspace context
- Command expects file path argument

**Solutions**:

1. **Open Workspace**:
   - File → Open Folder
   - Select your project directory

2. **Provide File Path**:
   ```bash
   # Instead of:
   /leetcode-test
   
   # Use:
   /leetcode-test .leetcode/solutions/1-two-sum.rs
   ```

3. **Navigate to Workspace**:
   ```bash
   cd /path/to/your/project
   # Then use commands
   ```

### ❌ "Failed to parse arguments"

**Symptoms**: Commands fail with argument parsing errors

**Root Causes**:
- Invalid command syntax
- Missing required arguments
- Special characters in arguments

**Solutions**:

1. **Check Command Syntax**:
   ```bash
   # Correct:
   /leetcode-list --difficulty easy --limit 10
   
   # Incorrect:
   /leetcode-list difficulty=easy limit=10
   ```

2. **Quote Complex Arguments**:
   ```bash
   # For paths with spaces:
   /leetcode-test ".leetcode/solutions/1-two sum.py"
   ```

3. **Use Full Paths**:
   ```bash
   # Absolute path:
   /leetcode-test /full/path/to/solution.rs
   ```

## Testing and Submission Problems

### ❌ "Test execution failed" / "Compilation error"

**Symptoms**: Tests fail before running logic

**Root Causes**:
- Syntax errors in code
- Missing imports/includes
- Incorrect function signatures
- Language-specific issues

**Solutions**:

1. **Check Syntax Locally**:
   ```bash
   # Python:
   python -m py_compile solution.py
   
   # Rust:
   rustc --check solution.rs
   
   # JavaScript:
   node -c solution.js
   ```

2. **Verify Function Signature**:
   - Ensure function name matches expected
   - Check parameter types and count
   - Verify return type

3. **Add Missing Imports**:
   ```python
   # Python common imports:
   from typing import List, Dict, Optional
   ```

   ```rust
   // Rust common imports:
   use std::collections::HashMap;
   ```

### ❌ "Wrong Answer" on Known Correct Solution

**Symptoms**: Local tests pass but LeetCode shows wrong answer

**Root Causes**:
- Edge case handling
- Integer overflow
- Off-by-one errors
- Input/output format differences

**Solutions**:

1. **Test Edge Cases**:
   ```python
   # Test with:
   # - Empty inputs
   # - Single element
   # - Maximum constraints
   # - Negative numbers
   ```

2. **Check Constraints**:
   - Review problem constraints carefully
   - Test with boundary values
   - Consider integer limits

3. **Debug Print Statements**:
   ```python
   # Add temporary debug prints
   print(f"Input: {input_val}")
   print(f"Output: {result}")
   # Remove before final submission
   ```

### ❌ "Time Limit Exceeded"

**Symptoms**: Solution times out on large inputs

**Root Causes**:
- Inefficient algorithm (O(n²) vs O(n))
- Infinite loops
- Excessive recursion
- Poor data structure choices

**Solutions**:

1. **Analyze Complexity**:
   ```python
   # O(n²) - might need optimization:
   for i in range(n):
       for j in range(n):
           # ...
   
   # O(n) - usually better:
   for i in range(n):
       # Single pass
   ```

2. **Use Efficient Data Structures**:
   - HashMap/Dict for O(1) lookups
   - Set for O(1) membership tests
   - Deque for O(1) front/back operations

3. **Optimize Common Patterns**:
   ```python
   # Instead of:
   if target in nums:  # O(n) lookup
   
   # Use:
   num_set = set(nums)  # O(1) lookup after O(n) preprocessing
   if target in num_set:
   ```

## Performance Issues

### ❌ Slow Extension Response

**Symptoms**: Commands take long time to execute

**Root Causes**:
- Network latency
- Large cached data
- Inefficient API calls
- System resource constraints

**Solutions**:

1. **Check Network Speed**:
   ```bash
   # Test download speed
   curl -w "@-" -o /dev/null -s https://leetcode.com/ <<< '
   time_namelookup: %{time_namelookup}
   time_connect: %{time_connect}
   time_total: %{time_total}
   '
   ```

2. **Clear Cache if Needed**:
   ```bash
   rm -rf .leetcode/problems/*
   # Cache will rebuild on next command
   ```

3. **Use Filters to Reduce Data**:
   ```bash
   # Instead of:
   /leetcode-list
   
   # Use:
   /leetcode-list --limit 20
   ```

### ❌ High Memory Usage

**Symptoms**: Zed becomes slow or unresponsive

**Root Causes**:
- Large cached problem data
- Memory leaks in extension
- Too many concurrent requests

**Solutions**:

1. **Monitor Cache Size**:
   ```bash
   du -sh .leetcode/problems/
   # If > 100MB, consider cleanup
   ```

2. **Restart Zed Periodically**:
   - Close and reopen Zed
   - Clears any memory accumulation

3. **Limit Concurrent Commands**:
   - Wait for commands to complete
   - Don't run multiple commands simultaneously

## Development and Building Problems

### ❌ "Compilation failed" during Extension Build

**Symptoms**: `cargo build` fails with errors

**Root Causes**:
- Rust version compatibility
- Missing dependencies
- Platform-specific issues
- Syntax errors in code

**Solutions**:

1. **Update Rust Toolchain**:
   ```bash
   rustup update
   rustup target add wasm32-unknown-unknown
   ```

2. **Clean and Rebuild**:
   ```bash
   cargo clean
   cargo build --release
   ```

3. **Check Dependencies**:
   ```bash
   cargo check
   cargo update
   ```

### ❌ "WebAssembly compilation failed"

**Symptoms**: Extension builds but fails to load in Zed

**Root Causes**:
- WASM target not installed
- Unsupported dependencies
- Binary size limits

**Solutions**:

1. **Install WASM Target**:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **Check Dependencies**:
   - Ensure all deps support `wasm32-unknown-unknown`
   - Remove system-specific dependencies

3. **Optimize Binary Size**:
   ```toml
   # In Cargo.toml
   [profile.release]
   opt-level = "s"  # Optimize for size
   lto = true       # Link time optimization
   ```

## Extension Installation Issues

### ❌ "Failed to install extension"

**Symptoms**: Zed cannot install the extension

**Root Causes**:
- Incorrect extension structure
- Missing required files
- Permission issues
- Zed version incompatibility

**Solutions**:

1. **Verify Extension Structure**:
   ```
   zed-leetcode/
   ├── Cargo.toml
   ├── extension.toml      # Required
   ├── src/
   │   ├── lib.rs         # Required
   │   └── ...
   └── target/
   ```

2. **Check extension.toml**:
   ```toml
   id = "leetcode"
   name = "LeetCode"
   description = "LeetCode integration for Zed"
   version = "0.1.0"
   
   [lib]
   kind = ["cdylib"]
   ```

3. **Build Extension**:
   ```bash
   cargo build --release --target wasm32-unknown-unknown
   ```

### ❌ "Extension loaded but commands not available"

**Symptoms**: Extension appears installed but commands don't work

**Root Causes**:
- Command registration issues
- Extension not activated
- Command handler errors

**Solutions**:

1. **Check Extension Status**:
   - Look in Zed Extensions panel
   - Should show as "Enabled"

2. **Restart Zed Completely**:
   - Quit Zed entirely
   - Restart application
   - Try commands again

3. **Check Extension Logs**:
   - Look for error messages in Zed console
   - Check system logs if available

## Getting Additional Help

### Log Collection

When reporting issues, include:

1. **Extension Logs**:
   - Any error messages from Zed
   - Console output if available

2. **System Information**:
   ```bash
   # Rust version
   rustc --version
   
   # System info
   uname -a
   
   # Zed version
   # From Zed → About Zed
   ```

3. **Network Test Results**:
   ```bash
   curl -v https://leetcode.com/api/problems/all/ 2>&1 | head -20
   ```

### Common Issue Patterns

| Symptom | Most Likely Cause | Quick Fix |
|---------|-------------------|-----------|
| All commands fail with auth error | Expired session | Re-run `/leetcode-login` |
| Commands not recognized | Extension not loaded | Restart Zed |
| Network timeouts | Connectivity/firewall | Check network settings |
| File creation fails | Permission issues | Fix directory permissions |
| Tests fail unexpectedly | Code logic errors | Review algorithm carefully |
| Slow performance | Network/cache issues | Clear cache and retry |

### Resources for Additional Help

- **GitHub Issues**: [Report bugs and feature requests](https://github.com/lyrad913/zed-leetcode/issues)
- **Zed Community**: [Official Zed Discord](https://discord.gg/zed-editor)
- **LeetCode Help**: [LeetCode Support](https://support.leetcode.com/)
- **Extension Documentation**: Check README.md and source code comments

### Before Reporting Issues

1. ✅ Try the solutions in this guide
2. ✅ Restart Zed Editor
3. ✅ Check your internet connection
4. ✅ Verify authentication is working
5. ✅ Include relevant error messages and logs

---

**Still need help?** [Open an issue](https://github.com/lyrad913/zed-leetcode/issues/new) with detailed information about your problem.
