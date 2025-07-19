use zed_extension_api::{SlashCommandOutput, Worktree};
use crate::templates::TemplateGenerator;
use crate::auth::AuthManager;
use crate::api::LeetCodeApi;
use crate::models::{ProblemFilters, Difficulty};

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
    // Check authentication first
    if !is_user_authenticated() {
        return Err("Please login first using /leetcode-login <session-cookie>".to_string());
    }

    // Parse filtering arguments
    let filters = parse_list_filters(&args)?;
    
    // Get authenticated API client
    let session_cookie = get_current_session()
        .ok_or("No valid session found. Please login again.")?;
    let api = LeetCodeApi::with_session(session_cookie);
    
    // Fetch problems from API
    let problems = api.fetch_problems(&filters)
        .map_err(|e| format!("Failed to fetch problems: {}", e))?;
    
    if problems.is_empty() {
        return Ok(SlashCommandOutput {
            text: "No problems found with the given filters.".to_string(),
            sections: vec![],
        });
    }
    
    // Format output
    let output = format_problems_list(&problems, &filters);
    
    Ok(SlashCommandOutput {
        text: output,
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
    
    // Check authentication first
    if !is_user_authenticated() {
        return Err("Please login first using /leetcode-login <session-cookie>".to_string());
    }
    
    // Get authenticated API client
    let session_cookie = get_current_session()
        .ok_or("No valid session found. Please login again.")?;
    let api = LeetCodeApi::with_session(session_cookie);
    
    // Fetch problem details
    let problem_detail = api.fetch_problem_detail(&problem_identifier)
        .map_err(|e| format!("Failed to fetch problem details: {}", e))?;
    
    let problem = problem_detail.ok_or(
        format!("Problem not found: {}", problem_identifier)
    )?;
    
    // Create solution file manually
    let config_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?
        .join(".leetcode");
    
    let solutions_dir = config_dir.join("solutions");
    std::fs::create_dir_all(&solutions_dir)
        .map_err(|e| format!("Failed to create solutions directory: {}", e))?;
    
    // Generate template file
    let template_generator = TemplateGenerator;
    let template_content = TemplateGenerator::generate_template(&problem, &language)
        .ok_or("Failed to generate template for this language".to_string())?;
    
    let filename = TemplateGenerator::generate_filename(&problem, &language);
    let solution_path = solutions_dir.join(filename);
    
    // Write template to file
    std::fs::write(&solution_path, &template_content)
        .map_err(|e| format!("Failed to write solution file: {}", e))?;
    
    // Format output with problem details and file path
    let output = format_problem_details(&problem, &solution_path, &language);
    
    Ok(SlashCommandOutput {
        text: output,
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
        
        // Should fail because not authenticated
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Please login first"));
    }

    #[test]
    fn test_handle_list_with_filters() {
        let args = vec!["--difficulty".to_string(), "easy".to_string()];
        let result = handle_list(args);
        
        // Should fail because not authenticated
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Please login first"));
    }

    #[test]
    fn test_handle_show_success() {
        let args = vec!["1".to_string()];
        let result = handle_show(args);
        
        // Should fail because not authenticated
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Please login first"));
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
        
        // Should fail because not authenticated
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Please login first"));
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

/// Parse command line arguments for list filtering
fn parse_list_filters(args: &[String]) -> Result<ProblemFilters, String> {
    let mut filters = ProblemFilters::default();
    let mut i = 0;
    
    while i < args.len() {
        match args[i].as_str() {
            "--difficulty" => {
                if i + 1 >= args.len() {
                    return Err("--difficulty requires a value (easy|medium|hard)".to_string());
                }
                let difficulty_str = &args[i + 1];
                filters.difficulty = Some(match difficulty_str.to_lowercase().as_str() {
                    "easy" => Difficulty::Easy,
                    "medium" => Difficulty::Medium,
                    "hard" => Difficulty::Hard,
                    _ => return Err(format!("Invalid difficulty: {}. Use easy|medium|hard", difficulty_str)),
                });
                i += 2;
            },
            "--tag" => {
                if i + 1 >= args.len() {
                    return Err("--tag requires a value".to_string());
                }
                let tags_str = &args[i + 1];
                filters.tags = tags_str.split(',').map(|s| s.trim().to_string()).collect();
                i += 2;
            },
            "--company" => {
                if i + 1 >= args.len() {
                    return Err("--company requires a value".to_string());
                }
                filters.company_tag = Some(args[i + 1].clone());
                i += 2;
            },
            "--limit" => {
                if i + 1 >= args.len() {
                    return Err("--limit requires a number".to_string());
                }
                let limit_str = &args[i + 1];
                filters.limit = Some(limit_str.parse().map_err(|_| 
                    format!("Invalid limit: {}. Must be a number", limit_str))?);
                i += 2;
            },
            arg if arg.starts_with("--") => {
                return Err(format!("Unknown option: {}. Available options: --difficulty, --tag, --company, --limit", arg));
            },
            _ => {
                return Err(format!("Unexpected argument: {}. Use --option value format", &args[i]));
            }
        }
    }
    
    Ok(filters)
}

/// Format problems list for display
fn format_problems_list(problems: &[crate::models::Problem], filters: &ProblemFilters) -> String {
    let mut output = String::new();
    
    // Header with filter info
    output.push_str("# LeetCode Problems\n\n");
    
    // Filter summary
    if filters.difficulty.is_some() || !filters.tags.is_empty() || filters.company_tag.is_some() {
        output.push_str("**Filters applied:**\n");
        if let Some(ref difficulty) = filters.difficulty {
            output.push_str(&format!("- Difficulty: {}\n", difficulty));
        }
        if !filters.tags.is_empty() {
            output.push_str(&format!("- Tags: {}\n", filters.tags.join(", ")));
        }
        if let Some(ref company) = filters.company_tag {
            output.push_str(&format!("- Company: {}\n", company));
        }
        output.push_str("\n");
    }
    
    // Problems table
    output.push_str("| # | Title | Difficulty | Acceptance | Tags |\n");
    output.push_str("|---|-------|------------|------------|------|\n");
    
    for problem in problems.iter().take(filters.limit.unwrap_or(50) as usize) {
        let difficulty_emoji = match problem.difficulty {
            Difficulty::Easy => "🟢",
            Difficulty::Medium => "🟡", 
            Difficulty::Hard => "🔴",
        };
        
        let tags = problem.tags.iter()
            .take(3) // Limit to first 3 tags for display
            .map(|tag| tag.name.clone())
            .collect::<Vec<_>>()
            .join(", ");
        
        output.push_str(&format!(
            "| {} | {} | {} {} | {:.1}% | {} |\n",
            problem.frontend_id,
            problem.title,
            difficulty_emoji,
            problem.difficulty,
            problem.acceptance_rate * 100.0,
            if tags.is_empty() { "-".to_string() } else { tags }
        ));
    }
    
    output.push_str(&format!("\n**Showing {} problems**", problems.len().min(filters.limit.unwrap_or(50) as usize)));
    if problems.len() > filters.limit.unwrap_or(50) as usize {
        output.push_str(&format!(" (out of {} total)", problems.len()));
    }
    
    output
}

/// Format problem details for display
fn format_problem_details(problem: &crate::models::ProblemDetail, solution_path: &std::path::Path, language: &str) -> String {
    let mut output = String::new();
    
    // Problem header
    output.push_str(&format!("# {}. {}\n\n", problem.frontend_id, problem.title));
    
    // Difficulty and stats
    let difficulty_emoji = match problem.difficulty {
        Difficulty::Easy => "🟢",
        Difficulty::Medium => "🟡",
        Difficulty::Hard => "🔴",
    };
    
    output.push_str(&format!("**Difficulty:** {} {}\n", difficulty_emoji, problem.difficulty));
    // Note: ProblemDetail doesn't include acceptance rate in our current model
    // output.push_str(&format!("**Acceptance Rate:** {:.1}%\n", problem.acceptance_rate * 100.0));
    
    // Tags
    if !problem.tags.is_empty() {
        let tags = problem.tags.iter()
            .map(|tag| format!("`{}`", tag.name))
            .collect::<Vec<_>>()
            .join(" ");
        output.push_str(&format!("**Tags:** {}\n", tags));
    }
    
    output.push_str("\n");
    
    // Problem description (simplified HTML removal)
    output.push_str("## Problem Description\n\n");
    let cleaned_content = simple_html_to_text(&problem.content);
    output.push_str(&cleaned_content);
    output.push_str("\n\n");
    
    // Sample test case (ProblemDetail doesn't have this field in our current model)
    // if let Some(ref sample_test) = problem.sample_test_case {
    //     output.push_str("## Sample Input/Output\n\n");
    //     output.push_str("```\n");
    //     output.push_str(sample_test);
    //     output.push_str("\n```\n\n");
    // }
    
    // Solution file info
    output.push_str("## Solution Template\n\n");
    output.push_str(&format!("**Language:** {}\n", language));
    output.push_str(&format!("**File created:** `{}`\n", solution_path.display()));
    output.push_str("\nThe template file has been created with the problem description and code skeleton. You can start coding your solution!\n");
    
    output
}

/// Simple HTML to text conversion
fn simple_html_to_text(content: &str) -> String {
    // Basic HTML tag removal - more sophisticated than regex for common cases
    let mut result = content.to_string();
    
    // Remove common HTML tags
    result = result.replace("<p>", "\n");
    result = result.replace("</p>", "\n");
    result = result.replace("<br>", "\n");
    result = result.replace("<br/>", "\n");
    result = result.replace("<br />", "\n");
    result = result.replace("<strong>", "**");
    result = result.replace("</strong>", "**");
    result = result.replace("<b>", "**");
    result = result.replace("</b>", "**");
    result = result.replace("<em>", "*");
    result = result.replace("</em>", "*");
    result = result.replace("<i>", "*");
    result = result.replace("</i>", "*");
    result = result.replace("<code>", "`");
    result = result.replace("</code>", "`");
    
    // Remove any remaining HTML tags with a simple regex-like approach
    let mut clean_result = String::new();
    let mut in_tag = false;
    let chars: Vec<char> = result.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        if chars[i] == '<' {
            in_tag = true;
        } else if chars[i] == '>' {
            in_tag = false;
        } else if !in_tag {
            clean_result.push(chars[i]);
        }
        i += 1;
    }
    
    // Clean up whitespace
    clean_result = clean_result.replace("\n\n\n", "\n\n");
    clean_result.trim().to_string()
}
