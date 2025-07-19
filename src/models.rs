use serde::{Deserialize, Serialize};

/// Difficulty levels for LeetCode problems
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
        }
    }
}

/// Problem tag information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub slug: String,
}

/// LeetCode problem information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Problem {
    #[serde(rename = "questionId")]
    pub id: String,
    #[serde(rename = "questionFrontendId")]
    pub frontend_id: String,
    pub title: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    pub difficulty: Difficulty,
    #[serde(rename = "topicTags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "isPaidOnly")]
    pub is_paid_only: bool,
    #[serde(rename = "acRate")]
    pub acceptance_rate: f64,
}

/// Problem details with content
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProblemDetail {
    #[serde(rename = "questionId")]
    pub id: String,
    #[serde(rename = "questionFrontendId")]
    pub frontend_id: String,
    pub title: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    pub content: String,
    pub difficulty: Difficulty,
    #[serde(rename = "topicTags")]
    pub tags: Vec<Tag>,
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Vec<CodeSnippet>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    #[serde(rename = "exampleTestcases")]
    pub example_testcases: Option<String>,
}

/// Code snippet for different languages
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeSnippet {
    pub lang: String,
    #[serde(rename = "langSlug")]
    pub lang_slug: String,
    pub code: String,
}

/// Filters for problem list queries
#[derive(Debug, Clone, Default)]
pub struct ProblemFilters {
    pub difficulty: Option<Difficulty>,
    pub tags: Vec<String>,
    pub company_tag: Option<String>,
    pub status: Option<String>, // "TODO", "SOLVED", etc.
    pub list_id: Option<String>,
    pub skip: Option<i32>,
    pub limit: Option<i32>,
}

/// GraphQL response wrapper
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

/// Test execution status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TestStatus {
    Success,
    CompileError,
    RuntimeError,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    WrongAnswer,
    UnknownError,
}

impl std::fmt::Display for TestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestStatus::Success => write!(f, "✅ All tests passed"),
            TestStatus::CompileError => write!(f, "❌ Compile Error"),
            TestStatus::RuntimeError => write!(f, "❌ Runtime Error"),
            TestStatus::TimeLimitExceeded => write!(f, "⏰ Time Limit Exceeded"),
            TestStatus::MemoryLimitExceeded => write!(f, "💾 Memory Limit Exceeded"),
            TestStatus::WrongAnswer => write!(f, "❌ Wrong Answer"),
            TestStatus::UnknownError => write!(f, "❓ Unknown Error"),
        }
    }
}

/// Test execution result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestResult {
    pub status: TestStatus,
    pub runtime: Option<u32>,  // milliseconds
    pub memory: Option<f64>,   // MB
    pub passed_tests: u32,
    pub total_tests: u32,
    pub failed_test_case: Option<String>,
    pub compile_error: Option<String>,
}

/// Test execution response data structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestExecutionData {
    #[serde(rename = "interpretSolution")]
    pub interpret_solution: Option<TestSubmissionResult>,
}

/// Test submission result from API
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestSubmissionResult {
    #[serde(rename = "submissionId")]
    pub submission_id: String,
    #[serde(rename = "statusCode")]
    pub status_code: u32,
    pub status: String,
    pub runtime: Option<String>,
    pub memory: Option<String>,
    #[serde(rename = "correctAnswer")]
    pub correct_answer: Option<bool>,
    #[serde(rename = "totalTestcases")]
    pub total_testcases: Option<u32>,
    #[serde(rename = "correctTestcases")]
    pub correct_testcases: Option<u32>,
    #[serde(rename = "compileError")]
    pub compile_error: Option<String>,
    #[serde(rename = "runtimeError")]
    pub runtime_error: Option<String>,
    #[serde(rename = "lastTestcase")]
    pub last_testcase: Option<String>,
    #[serde(rename = "expectedOutput")]
    pub expected_output: Option<String>,
    #[serde(rename = "codeOutput")]
    pub code_output: Option<String>,
}

/// Generic GraphQL error structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
}

/// Response structure for problem list query
#[derive(Debug, Deserialize)]
pub struct ProblemSetQuestionListData {
    #[serde(rename = "problemsetQuestionList")]
    pub problemset_question_list: ProblemSetQuestionList,
}

/// Problem list with metadata
#[derive(Debug, Deserialize)]
pub struct ProblemSetQuestionList {
    pub total: i32,
    pub questions: Vec<Problem>,
}

/// Response structure for problem detail query  
#[derive(Debug, Deserialize)]
pub struct QuestionData {
    pub question: Option<ProblemDetail>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_display() {
        assert_eq!(format!("{}", Difficulty::Easy), "Easy");
        assert_eq!(format!("{}", Difficulty::Medium), "Medium");
        assert_eq!(format!("{}", Difficulty::Hard), "Hard");
    }

    #[test]
    fn test_difficulty_serde() {
        let easy = Difficulty::Easy;
        let json = serde_json::to_string(&easy).unwrap();
        assert_eq!(json, "\"EASY\"");
        
        let parsed: Difficulty = serde_json::from_str("\"MEDIUM\"").unwrap();
        assert_eq!(parsed, Difficulty::Medium);
    }

    #[test]
    fn test_problem_filters_default() {
        let filters = ProblemFilters::default();
        assert!(filters.difficulty.is_none());
        assert!(filters.tags.is_empty());
        assert!(filters.company_tag.is_none());
        assert!(filters.status.is_none());
    }

    #[test]
    fn test_graphql_response_success() {
        let json = r#"{"data": {"test": "value"}}"#;
        let response: GraphQLResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(response.data.is_some());
        assert!(response.errors.is_none());
    }

    #[test]
    fn test_graphql_response_error() {
        let json = r#"{"errors": [{"message": "Test error"}]}"#;
        let response: GraphQLResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(response.data.is_none());
        assert!(response.errors.is_some());
        
        let errors = response.errors.unwrap();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "Test error");
    }

    #[test]
    fn test_test_result_display() {
        let result = TestResult {
            status: TestStatus::Success,
            runtime: Some(16),
            memory: Some(12.5),
            passed_tests: 5,
            total_tests: 5,
            failed_test_case: None,
            compile_error: None,
        };

        assert_eq!(result.status, TestStatus::Success);
        assert_eq!(result.runtime, Some(16));
        assert_eq!(result.passed_tests, 5);
    }

    #[test]
    fn test_test_result_failure() {
        let result = TestResult {
            status: TestStatus::RuntimeError,
            runtime: None,
            memory: None,
            passed_tests: 2,
            total_tests: 5,
            failed_test_case: Some("Input: [1,2,3]\nExpected: [1,3]\nActual: [1,2]".to_string()),
            compile_error: None,
        };

        assert_eq!(result.status, TestStatus::RuntimeError);
        assert!(result.failed_test_case.is_some());
    }
}
