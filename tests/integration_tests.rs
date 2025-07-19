use zed_leetcode::models::{ProblemFilters, Difficulty};

#[test]
fn test_problem_filters_creation() {
    let mut filters = ProblemFilters::default();
    assert!(filters.difficulty.is_none());
    
    filters.difficulty = Some(Difficulty::Easy);
    filters.limit = Some(5);
    
    assert_eq!(filters.difficulty, Some(Difficulty::Easy));
    assert_eq!(filters.limit, Some(5));
}

#[test] 
fn test_difficulty_variants() {
    let easy = Difficulty::Easy;
    let medium = Difficulty::Medium;
    let hard = Difficulty::Hard;
    
    assert_eq!(format!("{}", easy), "Easy");
    assert_eq!(format!("{}", medium), "Medium"); 
    assert_eq!(format!("{}", hard), "Hard");
}

// Note: Real network integration tests would be run separately
// or with feature flags to avoid network dependencies in CI
