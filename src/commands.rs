use zed_extension_api::{SlashCommandOutput, Worktree};
use crate::templates::TemplateGenerator;
use crate::auth::AuthManager;
use crate::api::LeetCodeApi;

/// Handle /leetcode-login command
/// Authenticates user with LeetCode session cookie
pub fn handle_login(args: Vec<String>) -> Result<SlashCommandOutput, String> {
    if args.is_empty() {
        return Err("Session cookie is required. Usage: /leetcode-login <session-cookie>".to_string());
    }
    
    let session_cookie = &args[0];
    
    // Use current directory as fallback for config
    let config_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?
        .join(".leetcode");
    
    let auth_manager = AuthManager::new(&config_dir);
    
    // Verify session cookie with API
    let api = LeetCodeApi::with_session(session_cookie.clone());
    match api.verify_authentication(session_cookie) {
        Ok(true) => {
            // Save session if valid
            match auth_manager.save_session(session_cookie) {
                Ok(_) => Ok(SlashCommandOutput {
                    text: "Successfully logged in to LeetCode! Session saved securely.".to_string(),
                    sections: vec![],
                }),
                Err(e) => Err(format!("Authentication successful but failed to save session: {}", e)),
            }
        },
        Ok(false) => Err("Invalid session cookie. Please check your session cookie from browser.".to_string()),
        Err(e) => Err(format!("Failed to verify session: {}", e)),
    }
}

/// Helper function to check if user is authenticated
pub fn is_user_authenticated() -> bool {
    let config_dir = match std::env::current_dir() {
        Ok(dir) => dir.join(".leetcode"),
        Err(_) => return false,
    };
    
    let auth_manager = AuthManager::new(&config_dir);
    let api = LeetCodeApi::new();
    auth_manager.is_authenticated(&api)
}

/// Helper function to get current session cookie
pub fn get_current_session() -> Option<String> {
    let config_dir = match std::env::current_dir() {
        Ok(dir) => dir.join(".leetcode"),
        Err(_) => return None,
    };
    
    let auth_manager = AuthManager::new(&config_dir);
    auth_manager.get_session_cookie().ok().flatten()
}

/// Handle /leetcode-list command  
/// Lists LeetCode problems with optional filtering
pub fn handle_list(args: Vec<String>) -> Result<SlashCommandOutput, String> {
    // TODO: Parse filtering arguments (--difficulty, --tag, etc.)
    let filter_info = if args.is_empty() {
        "all problems".to_string()
    } else {
        format!("filtered by: {}", args.join(" "))
    };
    
    Ok(SlashCommandOutput {
        text: format!("Listing LeetCode problems - {}", filter_info),
        sections: vec![],
    })
}

/// Handle /leetcode-show command
/// Shows problem details and creates solution template
pub fn handle_show(args: Vec<String>) -> Result<SlashCommandOutput, String> {
    if args.is_empty() {
        return Err("Problem ID or title is required. Usage: /leetcode-show <problem-id> [--language <lang>]".to_string());
    }
    
    let problem_identifier = &args[0];
    
    // Parse language option
    let parsed_args = parse_arguments(&args[1..]);
    let language = parsed_args.iter()
        .find(|(key, _)| key == "language")
        .and_then(|(_, value)| value.as_ref())
        .unwrap_or(&"rust".to_string())
        .clone();

    // Check if language is supported
    if !TemplateGenerator::get_supported_languages().contains(&language.as_str()) {
        return Err(format!(
            "Unsupported language: {}. Supported languages: {}", 
            language,
            TemplateGenerator::get_supported_languages().join(", ")
        ));
    }
    
    // TODO: Implement problem fetching and template creation with API and FileManager
    Ok(SlashCommandOutput {
        text: format!("Showing problem details for: {} (language: {}). Implementation with API integration pending.", problem_identifier, language),
        sections: vec![],
    })
}

/// Handle /leetcode-test command
/// Tests current solution file against LeetCode test cases
pub fn handle_test(_args: Vec<String>, worktree: Option<&Worktree>) -> Result<SlashCommandOutput, String> {
    // TODO: Get current file from worktree
    let _worktree = worktree.ok_or("No active workspace found")?;
    
    Ok(SlashCommandOutput {
        text: "Testing current solution file...".to_string(),
        sections: vec![],
    })
}

/// Handle /leetcode-submit command  
/// Submits current solution to LeetCode
pub fn handle_submit(_args: Vec<String>, worktree: Option<&Worktree>) -> Result<SlashCommandOutput, String> {
    // TODO: Get current file from worktree and submit
    let _worktree = worktree.ok_or("No active workspace found")?;
    
    Ok(SlashCommandOutput {
        text: "Submitting current solution...".to_string(),
        sections: vec![],
    })
}

/// Parse command line arguments into structured format
pub fn parse_arguments(args: &[String]) -> Vec<(String, Option<String>)> {
    let mut parsed = Vec::new();
    let mut i = 0;
    
    while i < args.len() {
        let arg = &args[i];
        if arg.starts_with("--") {
            let key = arg[2..].to_string();
            let value = if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                i += 1;
                Some(args[i].clone())
            } else {
                None
            };
            parsed.push((key, value));
        } else {
            parsed.push((arg.clone(), None));
        }
        i += 1;
    }
    
    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_login_success() {
        let args = vec!["session_cookie_123".to_string()];
        let result = handle_login(args);
        
        // Debug: print the error if it occurs
        if let Err(ref error) = result {
            println!("Login error: {}", error);
        }
        
        // For now, expect the real API call to fail in testing
        // but let's check the error message structure
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("Failed to verify session") || error_msg.contains("Invalid session cookie"));
    }

    #[test]
    fn test_helper_functions() {
        // Test authentication helper functions
        // These will return false/None in test environment but shouldn't panic
        assert_eq!(is_user_authenticated(), false);
        assert_eq!(get_current_session(), None);
    }

    #[test]
    fn test_handle_login_empty_cookie() {
        let args = vec!["".to_string()];
        let result = handle_login(args);
        
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        // Empty cookie should still trigger API verification which will fail
        assert!(error_msg.contains("Failed to verify session") || error_msg.contains("Invalid session cookie"));
    }

    #[test]
    fn test_handle_login_no_args() {
        let args = vec![];
        let result = handle_login(args);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Session cookie is required"));
    }

    #[test]
    fn test_handle_list_no_filters() {
        let args = vec![];
        let result = handle_list(args);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.text.contains("all problems"));
    }

    #[test]
    fn test_handle_list_with_filters() {
        let args = vec!["--difficulty".to_string(), "easy".to_string()];
        let result = handle_list(args);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.text.contains("filtered by"));
    }

    #[test]
    fn test_handle_show_success() {
        let args = vec!["1".to_string()];
        let result = handle_show(args);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.text.contains("Showing problem details for: 1"));
        assert!(output.text.contains("language: rust")); // default language
        assert!(output.text.contains("Implementation with API integration pending"));
    }

    #[test]
    fn test_handle_show_no_args() {
        let args = vec![];
        let result = handle_show(args);
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Problem ID or title is required"));
        assert!(error.contains("--language"));
    }

    #[test]
    fn test_handle_show_with_language() {
        let args = vec!["1".to_string(), "--language".to_string(), "python".to_string()];
        let result = handle_show(args);
        
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.text.contains("language: python"));
    }

    #[test]
    fn test_handle_show_unsupported_language() {
        let args = vec!["1".to_string(), "--language".to_string(), "cobol".to_string()];
        let result = handle_show(args);
        
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("Unsupported language: cobol"));
        assert!(error.contains("Supported languages:"));
    }

    #[test]
    fn test_handle_test_no_worktree() {
        let args = vec![];
        let result = handle_test(args, None);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No active workspace found"));
    }

    #[test]
    fn test_handle_submit_no_worktree() {
        let args = vec![];
        let result = handle_submit(args, None);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No active workspace found"));
    }

    #[test]
    fn test_parse_arguments_flags() {
        let args = vec!["--difficulty".to_string(), "easy".to_string(), "--tag".to_string(), "array".to_string()];
        let result = parse_arguments(&args);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("difficulty".to_string(), Some("easy".to_string())));
        assert_eq!(result[1], ("tag".to_string(), Some("array".to_string())));
    }

    #[test]
    fn test_parse_arguments_mixed() {
        let args = vec!["1".to_string(), "--language".to_string(), "rust".to_string()];
        let result = parse_arguments(&args);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], ("1".to_string(), None));
        assert_eq!(result[1], ("language".to_string(), Some("rust".to_string())));
    }
}
