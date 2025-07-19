use crate::models::{
    GraphQLResponse, Problem, ProblemDetail, ProblemFilters, 
    ProblemSetQuestionListData, QuestionData, Difficulty
};
use anyhow::{Context, Result};
use std::process::Command;

const LEETCODE_GRAPHQL_URL: &str = "https://leetcode.com/graphql/";

/// LeetCode API client using curl for HTTP requests
pub struct LeetCodeApi {
    session_cookie: Option<String>,
}

impl LeetCodeApi {
    /// Create a new API client
    pub fn new() -> Self {
        Self {
            session_cookie: None,
        }
    }

    /// Create API client with session cookie for authenticated requests
    pub fn with_session(session_cookie: String) -> Self {
        Self {
            session_cookie: Some(session_cookie),
        }
    }

    /// Fetch problems list with optional filters
    pub fn fetch_problems(&self, filters: &ProblemFilters) -> Result<Vec<Problem>> {
        let query = self.build_problem_list_query(filters);
        let response = self.execute_graphql_query(&query)?;
        
        let graphql_response: GraphQLResponse<ProblemSetQuestionListData> = 
            serde_json::from_str(&response)
                .context("Failed to parse GraphQL response")?;

        if let Some(errors) = graphql_response.errors {
            return Err(anyhow::anyhow!("GraphQL errors: {:?}", errors));
        }

        let data = graphql_response.data
            .context("No data in GraphQL response")?;

        Ok(data.problemset_question_list.questions)
    }

    /// Fetch problem details by title slug
    pub fn fetch_problem_detail(&self, title_slug: &str) -> Result<Option<ProblemDetail>> {
        let query = self.build_problem_detail_query(title_slug);
        let response = self.execute_graphql_query(&query)?;

        let graphql_response: GraphQLResponse<QuestionData> = 
            serde_json::from_str(&response)
                .context("Failed to parse GraphQL response")?;

        if let Some(errors) = graphql_response.errors {
            return Err(anyhow::anyhow!("GraphQL errors: {:?}", errors));
        }

        let data = graphql_response.data
            .context("No data in GraphQL response")?;

        Ok(data.question)
    }

    /// Verify authentication by checking user profile
    pub fn verify_authentication(&self, session_cookie: &str) -> Result<bool> {
        let query = r#"{
            "query": "query globalData { 
                userStatus { 
                    isSignedIn 
                    username 
                } 
            }"
        }"#;

        let mut cmd = Command::new("curl");
        cmd.args(["-s", "-X", "POST", LEETCODE_GRAPHQL_URL])
            .args(["-H", "Content-Type: application/json"])
            .args(["-H", "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36"])
            .args(["-H", &format!("Cookie: {}", session_cookie)])
            .args(["-d", query]);

        let output = cmd.output()
            .context("Failed to execute authentication curl command")?;

        if !output.status.success() {
            return Ok(false);
        }

        let response = String::from_utf8(output.stdout)
            .context("Failed to parse authentication response as UTF-8")?;

        // Parse response to check if user is signed in
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response) {
            if let Some(user_status) = json.get("data").and_then(|d| d.get("userStatus")) {
                if let Some(is_signed_in) = user_status.get("isSignedIn") {
                    return Ok(is_signed_in.as_bool().unwrap_or(false));
                }
            }
        }

        Ok(false)
    }

    /// Execute GraphQL query using curl
    fn execute_graphql_query(&self, query: &str) -> Result<String> {
        let mut cmd = Command::new("curl");
        cmd.args(["-s", "-X", "POST", LEETCODE_GRAPHQL_URL])
            .args(["-H", "Content-Type: application/json"])
            .args(["-H", "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36"]);

        // Add session cookie if available
        if let Some(ref cookie) = self.session_cookie {
            cmd.args(["-H", &format!("Cookie: {}", cookie)]);
        }

        cmd.args(["-d", query]);

        let output = cmd.output()
            .context("Failed to execute curl command")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("curl command failed: {}", stderr));
        }

        let response = String::from_utf8(output.stdout)
            .context("Failed to parse curl output as UTF-8")?;

        Ok(response)
    }

    /// Build GraphQL query for problem list
    fn build_problem_list_query(&self, filters: &ProblemFilters) -> String {
        let category_slug = "all-code-problems";
        let skip = filters.skip.unwrap_or(0);
        let limit = filters.limit.unwrap_or(50);

        let filter_obj = self.build_filter_object(filters);

        let query = format!(
            r#"{{
                "query": "query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {{
                    problemsetQuestionList(
                        categorySlug: $categorySlug
                        limit: $limit
                        skip: $skip
                        filters: $filters
                    ) {{
                        total: totalNum
                        questions: data {{
                            questionId
                            questionFrontendId
                            title
                            titleSlug
                            difficulty
                            topicTags {{
                                name
                                slug
                            }}
                            isPaidOnly
                            acRate
                        }}
                    }}
                }}",
                "variables": {{
                    "categorySlug": "{}",
                    "skip": {},
                    "limit": {},
                    "filters": {}
                }}
            }}"#,
            category_slug, skip, limit, filter_obj
        );

        query
    }

    /// Build GraphQL query for problem details
    fn build_problem_detail_query(&self, title_slug: &str) -> String {
        let query = format!(
            r#"{{
                "query": "query questionData($titleSlug: String!) {{
                    question(titleSlug: $titleSlug) {{
                        questionId
                        questionFrontendId
                        title
                        titleSlug
                        content
                        difficulty
                        topicTags {{
                            name
                            slug
                        }}
                        codeSnippets {{
                            lang
                            langSlug
                            code
                        }}
                        sampleTestCase
                        exampleTestcases
                    }}
                }}",
                "variables": {{
                    "titleSlug": "{}"
                }}
            }}"#,
            title_slug
        );

        query
    }

    /// Build filter object for GraphQL query
    fn build_filter_object(&self, filters: &ProblemFilters) -> String {
        let mut filter_parts = Vec::new();

        if let Some(ref difficulty) = filters.difficulty {
            let difficulty_str = match difficulty {
                Difficulty::Easy => "EASY",
                Difficulty::Medium => "MEDIUM", 
                Difficulty::Hard => "HARD",
            };
            filter_parts.push(format!(r#""difficulty": "{}""#, difficulty_str));
        }

        if !filters.tags.is_empty() {
            let tags_json = serde_json::to_string(&filters.tags).unwrap_or_default();
            filter_parts.push(format!(r#""tags": {}"#, tags_json));
        }

        if let Some(ref company_tag) = filters.company_tag {
            filter_parts.push(format!(r#""companyTag": "{}""#, company_tag));
        }

        if let Some(ref status) = filters.status {
            filter_parts.push(format!(r#""status": "{}""#, status));
        }

        if filter_parts.is_empty() {
            "{}".to_string()
        } else {
            format!("{{{}}}", filter_parts.join(", "))
        }
    }
}

impl Default for LeetCodeApi {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_creation() {
        let api = LeetCodeApi::new();
        assert!(api.session_cookie.is_none());

        let api_with_session = LeetCodeApi::with_session("test_cookie".to_string());
        assert!(api_with_session.session_cookie.is_some());
        assert_eq!(api_with_session.session_cookie.unwrap(), "test_cookie");
    }

    #[test]
    fn test_filter_object_empty() {
        let api = LeetCodeApi::new();
        let filters = ProblemFilters::default();
        let filter_obj = api.build_filter_object(&filters);
        assert_eq!(filter_obj, "{}");
    }

    #[test]
    fn test_filter_object_with_difficulty() {
        let api = LeetCodeApi::new();
        let mut filters = ProblemFilters::default();
        filters.difficulty = Some(Difficulty::Easy);
        
        let filter_obj = api.build_filter_object(&filters);
        assert!(filter_obj.contains(r#""difficulty": "EASY""#));
    }

    #[test]
    fn test_filter_object_with_tags() {
        let api = LeetCodeApi::new();
        let mut filters = ProblemFilters::default();
        filters.tags = vec!["array".to_string(), "string".to_string()];
        
        let filter_obj = api.build_filter_object(&filters);
        assert!(filter_obj.contains(r#""tags": ["array","string"]"#));
    }

    #[test]
    fn test_filter_object_complex() {
        let api = LeetCodeApi::new();
        let mut filters = ProblemFilters::default();
        filters.difficulty = Some(Difficulty::Medium);
        filters.tags = vec!["array".to_string()];
        filters.company_tag = Some("google".to_string());
        
        let filter_obj = api.build_filter_object(&filters);
        assert!(filter_obj.contains(r#""difficulty": "MEDIUM""#));
        assert!(filter_obj.contains(r#""tags": ["array"]"#));
        assert!(filter_obj.contains(r#""companyTag": "google""#));
    }

    #[test]
    fn test_problem_list_query_structure() {
        let api = LeetCodeApi::new();
        let filters = ProblemFilters::default();
        let query = api.build_problem_list_query(&filters);
        
        assert!(query.contains("problemsetQuestionList"));
        assert!(query.contains("questionFrontendId"));
        assert!(query.contains("title"));
        assert!(query.contains("difficulty"));
        assert!(query.contains("topicTags"));
    }

    #[test]
    fn test_problem_detail_query_structure() {
        let api = LeetCodeApi::new();
        let query = api.build_problem_detail_query("two-sum");
        
        assert!(query.contains("question"));
        assert!(query.contains("content"));
        assert!(query.contains("codeSnippets"));
        assert!(query.contains("sampleTestCase"));
        assert!(query.contains(r#""titleSlug": "two-sum""#));
    }

    // Note: Integration tests for actual API calls would be in tests/integration_tests.rs
    // to avoid real network calls in unit tests
}
