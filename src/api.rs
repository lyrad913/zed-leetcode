use crate::models::{
    GraphQLResponse, Problem, ProblemDetail, ProblemFilters, 
    ProblemSetQuestionListData, QuestionData, Difficulty,
    TestExecutionData, TestSubmissionResult, TestResult, TestStatus,
    SubmissionData, SubmissionCheckData, SubmissionDetails,
    SubmissionResult, SubmissionStatus
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

    /// Execute test for a problem solution
    pub fn run_test(&self, title_slug: &str, code: &str, lang: &str) -> Result<TestResult> {
        if self.session_cookie.is_none() {
            return Err(anyhow::anyhow!("Authentication required for testing"));
        }

        // Step 1: Submit code for testing
        let submission_id = self.submit_for_test(title_slug, code, lang)?;

        // Step 2: Poll for results
        let result = self.poll_test_result(&submission_id)?;

        Ok(result)
    }

    /// Submit code for testing and return submission ID
    fn submit_for_test(&self, title_slug: &str, code: &str, lang: &str) -> Result<String> {
        let query = format!(
            r#"{{
                "query": "mutation interpretSolution($titleSlug: String!, $code: String!, $lang: String!) {{
                    interpretSolution(titleSlug: $titleSlug, code: $code, lang: $lang) {{
                        submissionId
                    }}
                }}",
                "variables": {{
                    "titleSlug": "{}",
                    "code": "{}",
                    "lang": "{}"
                }}
            }}"#,
            title_slug,
            code.replace('"', "\\\"").replace('\n', "\\n"),
            lang
        );

        let response = self.execute_graphql_query(&query)?;
        let graphql_response: GraphQLResponse<TestExecutionData> = 
            serde_json::from_str(&response)
                .context("Failed to parse test submission response")?;

        if let Some(errors) = graphql_response.errors {
            return Err(anyhow::anyhow!("GraphQL errors: {:?}", errors));
        }

        let data = graphql_response.data
            .context("No data in test submission response")?;

        let interpret_solution = data.interpret_solution
            .context("No interpretation result in response")?;

        Ok(interpret_solution.submission_id)
    }

    /// Poll for test result by submission ID
    fn poll_test_result(&self, submission_id: &str) -> Result<TestResult> {
        let max_polls = 30; // Poll for up to 30 seconds
        let poll_interval = std::time::Duration::from_secs(1);

        for _ in 0..max_polls {
            let query = format!(
                r#"{{
                    "query": "query checkInterpret($submissionId: String!) {{
                        interpretSolution(submissionId: $submissionId) {{
                            submissionId
                            statusCode
                            status
                            runtime
                            memory
                            correctAnswer
                            totalTestcases
                            correctTestcases
                            compileError
                            runtimeError
                            lastTestcase
                            expectedOutput
                            codeOutput
                        }}
                    }}",
                    "variables": {{
                        "submissionId": "{}"
                    }}
                }}"#,
                submission_id
            );

            let response = self.execute_graphql_query(&query)?;
            let graphql_response: GraphQLResponse<TestExecutionData> = 
                serde_json::from_str(&response)
                    .context("Failed to parse test result response")?;

            if let Some(errors) = graphql_response.errors {
                return Err(anyhow::anyhow!("GraphQL errors: {:?}", errors));
            }

            if let Some(data) = graphql_response.data {
                if let Some(interpret_result) = data.interpret_solution {
                    // Check if execution is complete
                    if interpret_result.status_code == 10 { // SUCCESS
                        return Ok(self.parse_test_result(interpret_result));
                    } else if interpret_result.status_code > 10 { // ERROR or COMPLETE
                        return Ok(self.parse_test_result(interpret_result));
                    }
                    // If status_code is < 10, continue polling (still running)
                }
            }

            std::thread::sleep(poll_interval);
        }

        Err(anyhow::anyhow!("Test execution timed out"))
    }

    /// Parse API result into TestResult structure
    fn parse_test_result(&self, result: TestSubmissionResult) -> TestResult {
        let status = match result.status_code {
            10 => TestStatus::Success,
            11 => TestStatus::WrongAnswer,
            12 => TestStatus::MemoryLimitExceeded,
            13 => TestStatus::TimeLimitExceeded,
            14 => TestStatus::RuntimeError,
            15 => TestStatus::CompileError,
            _ => TestStatus::UnknownError,
        };

        let runtime = result.runtime
            .and_then(|r| r.parse::<u32>().ok());

        let memory = result.memory
            .and_then(|m| m.parse::<f64>().ok());

        let failed_test_case = if status != TestStatus::Success {
            let mut failure_info = Vec::new();
            
            if let Some(input) = result.last_testcase {
                failure_info.push(format!("Input: {}", input));
            }
            if let Some(expected) = result.expected_output {
                failure_info.push(format!("Expected: {}", expected));
            }
            if let Some(actual) = result.code_output {
                failure_info.push(format!("Actual: {}", actual));
            }
            
            if !failure_info.is_empty() {
                Some(failure_info.join("\n"))
            } else {
                None
            }
        } else {
            None
        };

        TestResult {
            status,
            runtime,
            memory,
            passed_tests: result.correct_testcases.unwrap_or(0),
            total_tests: result.total_testcases.unwrap_or(0),
            failed_test_case,
            compile_error: result.compile_error.or(result.runtime_error),
        }
    }

    /// Submit solution to LeetCode
    pub fn submit_solution(&self, title_slug: &str, code: &str, lang: &str) -> Result<SubmissionResult> {
        if self.session_cookie.is_none() {
            return Err(anyhow::anyhow!("Authentication required for solution submission"));
        }

        // Step 1: Submit solution
        let submission_id = self.submit_code(title_slug, code, lang)?;

        // Step 2: Poll for results
        let result = self.poll_submission_result(&submission_id)?;

        Ok(result)
    }

    /// Submit code and return submission ID
    fn submit_code(&self, title_slug: &str, code: &str, lang: &str) -> Result<String> {
        let query = format!(
            r#"{{
                "query": "mutation submitSolution($titleSlug: String!, $code: String!, $lang: String!) {{
                    submitSolution(titleSlug: $titleSlug, code: $code, lang: $lang) {{
                        submissionId
                    }}
                }}",
                "variables": {{
                    "titleSlug": "{}",
                    "code": "{}",
                    "lang": "{}"
                }}
            }}"#,
            title_slug,
            code.replace('"', "\\\"").replace('\n', "\\n"),
            lang
        );

        let response = self.execute_graphql_query(&query)?;
        let graphql_response: GraphQLResponse<SubmissionData> = 
            serde_json::from_str(&response)
                .context("Failed to parse submission response")?;

        if let Some(errors) = graphql_response.errors {
            return Err(anyhow::anyhow!("GraphQL errors: {:?}", errors));
        }

        let data = graphql_response.data
            .context("No data in submission response")?;

        let submit_info = data.submit_solution
            .context("No submission result in response")?;

        Ok(submit_info.submission_id)
    }

    /// Poll for submission result by submission ID
    fn poll_submission_result(&self, submission_id: &str) -> Result<SubmissionResult> {
        let max_polls = 30; // Poll for up to 30 seconds
        let poll_interval = std::time::Duration::from_secs(1);

        for _ in 0..max_polls {
            let query = format!(
                r#"{{
                    "query": "query checkSubmission($submissionId: String!) {{
                        submissionDetails(submissionId: $submissionId) {{
                            submissionId
                            statusCode
                            status
                            runtime
                            memory
                            runtimePercentile
                            memoryPercentile
                            totalCorrect
                            totalTestcases
                            compileError
                            runtimeError
                            lastTestcase
                            expectedOutput
                            codeOutput
                        }}
                    }}",
                    "variables": {{
                        "submissionId": "{}"
                    }}
                }}"#,
                submission_id
            );

            let response = self.execute_graphql_query(&query)?;
            let graphql_response: GraphQLResponse<SubmissionCheckData> = 
                serde_json::from_str(&response)
                    .context("Failed to parse submission result response")?;

            if let Some(errors) = graphql_response.errors {
                return Err(anyhow::anyhow!("GraphQL errors: {:?}", errors));
            }

            if let Some(data) = graphql_response.data {
                if let Some(details) = data.submission_details {
                    // Check if submission is complete
                    if details.status_code >= 10 { // Complete (success or error)
                        return Ok(self.parse_submission_result(details));
                    }
                    // If status_code < 10, continue polling (still running)
                }
            }

            std::thread::sleep(poll_interval);
        }

        Err(anyhow::anyhow!("Submission polling timed out"))
    }

    /// Parse API result into SubmissionResult structure
    fn parse_submission_result(&self, details: SubmissionDetails) -> SubmissionResult {
        let status = match details.status_code {
            10 => SubmissionStatus::Accepted,
            11 => SubmissionStatus::WrongAnswer,
            12 => SubmissionStatus::MemoryLimitExceeded,
            13 => SubmissionStatus::TimeLimitExceeded,
            14 => SubmissionStatus::RuntimeError,
            15 => SubmissionStatus::CompileError,
            16 => SubmissionStatus::OutputLimitExceeded,
            20 => SubmissionStatus::InternalError,
            _ => SubmissionStatus::Unknown,
        };

        let runtime = details.runtime
            .and_then(|r| r.parse::<u32>().ok());

        let memory = details.memory
            .and_then(|m| m.parse::<f64>().ok());

        let failed_test_case = if status != SubmissionStatus::Accepted {
            let mut failure_info = Vec::new();
            
            if let Some(input) = details.last_testcase {
                failure_info.push(format!("Input: {}", input));
            }
            if let Some(expected) = details.expected_output {
                failure_info.push(format!("Expected: {}", expected));
            }
            if let Some(actual) = details.code_output {
                failure_info.push(format!("Actual: {}", actual));
            }
            
            if !failure_info.is_empty() {
                Some(failure_info.join("\n"))
            } else {
                None
            }
        } else {
            None
        };

        SubmissionResult {
            status,
            runtime,
            memory,
            runtime_percentile: details.runtime_percentile,
            memory_percentile: details.memory_percentile,
            total_correct: details.total_correct,
            total_testcases: details.total_testcases,
            failed_test_case,
            compile_error: details.compile_error,
            runtime_error: details.runtime_error,
        }
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

    #[test]
    fn test_parse_test_result_success() {
        let api = LeetCodeApi::new();
        let mock_result = TestSubmissionResult {
            submission_id: "123".to_string(),
            status_code: 10,
            status: "Success".to_string(),
            runtime: Some("16".to_string()),
            memory: Some("12.5".to_string()),
            correct_answer: Some(true),
            total_testcases: Some(5),
            correct_testcases: Some(5),
            compile_error: None,
            runtime_error: None,
            last_testcase: None,
            expected_output: None,
            code_output: None,
        };

        let result = api.parse_test_result(mock_result);
        assert_eq!(result.status, TestStatus::Success);
        assert_eq!(result.runtime, Some(16));
        assert_eq!(result.memory, Some(12.5));
        assert_eq!(result.passed_tests, 5);
        assert_eq!(result.total_tests, 5);
    }

    #[test]
    fn test_parse_test_result_wrong_answer() {
        let api = LeetCodeApi::new();
        let mock_result = TestSubmissionResult {
            submission_id: "123".to_string(),
            status_code: 11,
            status: "Wrong Answer".to_string(),
            runtime: None,
            memory: None,
            correct_answer: Some(false),
            total_testcases: Some(5),
            correct_testcases: Some(2),
            compile_error: None,
            runtime_error: None,
            last_testcase: Some("[1,2,3]".to_string()),
            expected_output: Some("[1,3]".to_string()),
            code_output: Some("[1,2]".to_string()),
        };

        let result = api.parse_test_result(mock_result);
        assert_eq!(result.status, TestStatus::WrongAnswer);
        assert_eq!(result.passed_tests, 2);
        assert_eq!(result.total_tests, 5);
        assert!(result.failed_test_case.is_some());
        
        let failure_info = result.failed_test_case.unwrap();
        assert!(failure_info.contains("Input: [1,2,3]"));
        assert!(failure_info.contains("Expected: [1,3]"));
        assert!(failure_info.contains("Actual: [1,2]"));
    }

    #[test]
    fn test_run_test_requires_auth() {
        let api = LeetCodeApi::new(); // No session cookie
        let result = api.run_test("two-sum", "test code", "python");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Authentication required"));
    }

    #[test]
    fn test_parse_submission_result_accepted() {
        let api = LeetCodeApi::new();
        let mock_details = SubmissionDetails {
            submission_id: "123".to_string(),
            status_code: 10,
            status: "Accepted".to_string(),
            runtime: Some("16".to_string()),
            memory: Some("12.5".to_string()),
            runtime_percentile: Some(85.2),
            memory_percentile: Some(91.3),
            total_correct: Some(100),
            total_testcases: Some(100),
            compile_error: None,
            runtime_error: None,
            last_testcase: None,
            expected_output: None,
            code_output: None,
        };

        let result = api.parse_submission_result(mock_details);
        assert_eq!(result.status, SubmissionStatus::Accepted);
        assert_eq!(result.runtime, Some(16));
        assert_eq!(result.memory, Some(12.5));
        assert_eq!(result.runtime_percentile, Some(85.2));
        assert_eq!(result.memory_percentile, Some(91.3));
        assert_eq!(result.total_correct, Some(100));
        assert_eq!(result.total_testcases, Some(100));
    }

    #[test]
    fn test_parse_submission_result_wrong_answer() {
        let api = LeetCodeApi::new();
        let mock_details = SubmissionDetails {
            submission_id: "123".to_string(),
            status_code: 11,
            status: "Wrong Answer".to_string(),
            runtime: None,
            memory: None,
            runtime_percentile: None,
            memory_percentile: None,
            total_correct: Some(95),
            total_testcases: Some(100),
            compile_error: None,
            runtime_error: None,
            last_testcase: Some("[1,2,3,4,5]".to_string()),
            expected_output: Some("[1,3,5]".to_string()),
            code_output: Some("[1,2,3]".to_string()),
        };

        let result = api.parse_submission_result(mock_details);
        assert_eq!(result.status, SubmissionStatus::WrongAnswer);
        assert_eq!(result.total_correct, Some(95));
        assert_eq!(result.total_testcases, Some(100));
        assert!(result.failed_test_case.is_some());
        
        let failure_info = result.failed_test_case.unwrap();
        assert!(failure_info.contains("Input: [1,2,3,4,5]"));
        assert!(failure_info.contains("Expected: [1,3,5]"));
        assert!(failure_info.contains("Actual: [1,2,3]"));
    }

    #[test]
    fn test_submit_solution_requires_auth() {
        let api = LeetCodeApi::new(); // No session cookie
        let result = api.submit_solution("two-sum", "test code", "python");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Authentication required"));
    }

    // Note: Integration tests for actual API calls would be in tests/integration_tests.rs
    // to avoid real network calls in unit tests
}
