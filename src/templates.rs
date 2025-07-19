use crate::models::{ProblemDetail, CodeSnippet};

/// Code template generator for different programming languages
pub struct TemplateGenerator;

impl TemplateGenerator {
    /// Generate solution template for a given problem and language
    pub fn generate_template(problem: &ProblemDetail, language: &str) -> Option<String> {
        let code_snippet = problem.code_snippets.iter()
            .find(|snippet| snippet.lang_slug == language)?;

        let template = Self::build_template_with_comments(problem, code_snippet);
        Some(template)
    }

    /// Generate solution filename
    pub fn generate_filename(problem: &ProblemDetail, language: &str) -> String {
        let extension = Self::get_file_extension(language);
        let safe_title = Self::sanitize_filename(&problem.title);
        format!("{}-{}.{}", problem.frontend_id, safe_title, extension)
    }

    /// Build complete template with problem description and code
    fn build_template_with_comments(problem: &ProblemDetail, code_snippet: &CodeSnippet) -> String {
        let comment_prefix = Self::get_comment_prefix(&code_snippet.lang_slug);
        let separator = format!("{} {}", comment_prefix, "-".repeat(60));
        
        let mut template = String::new();
        
        // Add problem header
        template.push_str(&format!("{}\n", separator));
        template.push_str(&format!("{} Problem: {} ({})\n", comment_prefix, problem.title, problem.frontend_id));
        template.push_str(&format!("{} Difficulty: {}\n", comment_prefix, problem.difficulty));
        template.push_str(&format!("{} Tags: {}\n", comment_prefix, 
            problem.tags.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join(", ")));
        template.push_str(&format!("{}\n", separator));
        template.push_str("\n");
        
        // Add problem description (truncated)
        let description = Self::extract_problem_description(&problem.content);
        for line in description.lines().take(10) { // Limit to first 10 lines
            template.push_str(&format!("{} {}\n", comment_prefix, line.trim()));
        }
        if description.lines().count() > 10 {
            template.push_str(&format!("{} ...\n", comment_prefix));
        }
        template.push_str(&format!("{}\n", separator));
        template.push_str("\n");
        
        // Add example if available
        if !problem.sample_test_case.is_empty() {
            template.push_str(&format!("{} Example:\n", comment_prefix));
            template.push_str(&format!("{} Input: {}\n", comment_prefix, problem.sample_test_case));
            template.push_str(&format!("{}\n", separator));
            template.push_str("\n");
        }
        
        // Add the code snippet
        template.push_str(&code_snippet.code);
        template.push_str("\n");
        
        template
    }

    /// Extract clean problem description from HTML content
    fn extract_problem_description(content: &str) -> String {
        // Simple HTML tag removal - in production, you'd use a proper HTML parser
        let mut description = content.to_string();
        
        // Remove common HTML tags
        description = description.replace("<p>", "");
        description = description.replace("</p>", "\n");
        description = description.replace("<strong>", "");
        description = description.replace("</strong>", "");
        description = description.replace("<em>", "");
        description = description.replace("</em>", "");
        description = description.replace("<code>", "`");
        description = description.replace("</code>", "`");
        description = description.replace("&lt;", "<");
        description = description.replace("&gt;", ">");
        description = description.replace("&amp;", "&");
        
        // Clean up extra whitespace
        description.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Get comment prefix for different languages
    fn get_comment_prefix(language: &str) -> &'static str {
        match language {
            "rust" | "cpp" | "c" | "java" | "javascript" | "typescript" => "//",
            "python" | "python3" => "#",
            "go" => "//",
            "ruby" => "#",
            "swift" => "//",
            "kotlin" => "//",
            "scala" => "//",
            _ => "//", // Default fallback
        }
    }

    /// Get file extension for different languages
    pub fn get_file_extension(language: &str) -> &'static str {
        match language {
            "rust" => "rs",
            "python" | "python3" => "py",
            "cpp" => "cpp",
            "c" => "c",
            "java" => "java",
            "javascript" => "js",
            "typescript" => "ts",
            "go" => "go",
            "ruby" => "rb",
            "swift" => "swift",
            "kotlin" => "kt",
            "scala" => "scala",
            _ => "txt", // Default fallback
        }
    }

    /// Sanitize filename by removing invalid characters
    fn sanitize_filename(title: &str) -> String {
        title.to_lowercase()
            .chars()
            .map(|c| match c {
                'a'..='z' | '0'..='9' => c,
                ' ' | '-' | '_' => '-',
                _ => '_',
            })
            .collect::<String>()
            .trim_matches('_')
            .trim_matches('-')
            .to_string()
    }

    /// Get supported languages list
    pub fn get_supported_languages() -> &'static [&'static str] {
        &[
            "rust", "python", "python3", "cpp", "c", "java", 
            "javascript", "typescript", "go", "ruby", "swift", 
            "kotlin", "scala"
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Difficulty, Tag};

    fn create_test_problem() -> ProblemDetail {
        ProblemDetail {
            id: "1".to_string(),
            frontend_id: "1".to_string(),
            title: "Two Sum".to_string(),
            title_slug: "two-sum".to_string(),
            content: "<p>Given an array of integers <code>nums</code> and an integer <code>target</code>, return <em>indices of the two numbers such that they add up to <code>target</code></em>.</p>".to_string(),
            difficulty: Difficulty::Easy,
            tags: vec![
                Tag { name: "Array".to_string(), slug: "array".to_string() },
                Tag { name: "Hash Table".to_string(), slug: "hash-table".to_string() },
            ],
            code_snippets: vec![
                CodeSnippet {
                    lang: "Rust".to_string(),
                    lang_slug: "rust".to_string(),
                    code: "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        \n    }\n}".to_string(),
                },
                CodeSnippet {
                    lang: "Python".to_string(),
                    lang_slug: "python".to_string(),
                    code: "class Solution:\n    def twoSum(self, nums: List[int], target: int) -> List[int]:\n        ".to_string(),
                },
            ],
            sample_test_case: "nums = [2,7,11,15], target = 9".to_string(),
            example_testcases: Some("Input: nums = [2,7,11,15], target = 9\nOutput: [0,1]".to_string()),
        }
    }

    #[test]
    fn test_generate_filename() {
        let problem = create_test_problem();
        let filename = TemplateGenerator::generate_filename(&problem, "rust");
        assert_eq!(filename, "1-two-sum.rs");

        let filename_py = TemplateGenerator::generate_filename(&problem, "python");
        assert_eq!(filename_py, "1-two-sum.py");
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(TemplateGenerator::sanitize_filename("Two Sum"), "two-sum");
        assert_eq!(TemplateGenerator::sanitize_filename("Valid Parentheses"), "valid-parentheses");
        assert_eq!(TemplateGenerator::sanitize_filename("3Sum"), "3sum");
    }

    #[test]
    fn test_get_file_extension() {
        assert_eq!(TemplateGenerator::get_file_extension("rust"), "rs");
        assert_eq!(TemplateGenerator::get_file_extension("python"), "py");
        assert_eq!(TemplateGenerator::get_file_extension("cpp"), "cpp");
        assert_eq!(TemplateGenerator::get_file_extension("unknown"), "txt");
    }

    #[test]
    fn test_get_comment_prefix() {
        assert_eq!(TemplateGenerator::get_comment_prefix("rust"), "//");
        assert_eq!(TemplateGenerator::get_comment_prefix("python"), "#");
        assert_eq!(TemplateGenerator::get_comment_prefix("cpp"), "//");
    }

    #[test]
    fn test_extract_problem_description() {
        let html = "<p>Given an array of integers <code>nums</code> and an integer <code>target</code>.</p>";
        let extracted = TemplateGenerator::extract_problem_description(html);
        assert!(extracted.contains("Given an array of integers `nums` and an integer `target`."));
        assert!(!extracted.contains("<p>"));
    }

    #[test]
    fn test_generate_template_rust() {
        let problem = create_test_problem();
        let template = TemplateGenerator::generate_template(&problem, "rust").unwrap();
        
        assert!(template.contains("// Problem: Two Sum (1)"));
        assert!(template.contains("// Difficulty: Easy"));
        assert!(template.contains("// Tags: Array, Hash Table"));
        assert!(template.contains("impl Solution"));
        assert!(template.contains("pub fn two_sum"));
    }

    #[test]
    fn test_generate_template_python() {
        let problem = create_test_problem();
        let template = TemplateGenerator::generate_template(&problem, "python").unwrap();
        
        assert!(template.contains("# Problem: Two Sum (1)"));
        assert!(template.contains("# Difficulty: Easy"));
        assert!(template.contains("class Solution"));
        assert!(template.contains("def twoSum"));
    }

    #[test]
    fn test_generate_template_unknown_language() {
        let problem = create_test_problem();
        let template = TemplateGenerator::generate_template(&problem, "unknown");
        assert!(template.is_none());
    }

    #[test]
    fn test_supported_languages() {
        let languages = TemplateGenerator::get_supported_languages();
        assert!(languages.contains(&"rust"));
        assert!(languages.contains(&"python"));
        assert!(languages.contains(&"cpp"));
        assert!(languages.len() > 5);
    }
}
