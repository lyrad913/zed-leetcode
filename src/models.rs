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

/// GraphQL error structure
#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    pub locations: Option<Vec<ErrorLocation>>,
    pub path: Option<Vec<serde_json::Value>>,
}

/// Error location in GraphQL query
#[derive(Debug, Deserialize)]
pub struct ErrorLocation {
    pub line: i32,
    pub column: i32,
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
}
