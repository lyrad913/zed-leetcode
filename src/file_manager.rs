use crate::models::{Problem, ProblemDetail};
use crate::templates::TemplateGenerator;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use zed_extension_api::Worktree;

const LEETCODE_DIR: &str = ".leetcode";
const PROBLEMS_DIR: &str = "problems";
const SOLUTIONS_DIR: &str = "solutions";
const CONFIG_FILE: &str = "config.json";

/// Configuration stored in .leetcode/config.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub session_cookie: Option<String>,
    pub default_language: String,
    pub created_at: String,
    pub last_updated: String,
}

impl Default for Config {
    fn default() -> Self {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
        Self {
            session_cookie: None,
            default_language: "rust".to_string(),
            created_at: now.clone(),
            last_updated: now,
        }
    }
}

/// File system manager for LeetCode problems and solutions
pub struct FileManager {
    workspace_root: PathBuf,
    leetcode_dir: PathBuf,
}

impl FileManager {
    /// Create new FileManager with workspace root from Worktree
    pub fn new(worktree: &Worktree) -> Result<Self> {
        // For now, use a placeholder path until we can access Worktree path properly
        // This will be fixed when we have proper Worktree API documentation
        let workspace_root = PathBuf::from("/tmp/leetcode-workspace"); // Placeholder
        let leetcode_dir = workspace_root.join(LEETCODE_DIR);

        let manager = Self {
            workspace_root,
            leetcode_dir,
        };

        // Ensure directories exist
        manager.ensure_directories()?;
        Ok(manager)
    }

    /// Create FileManager for testing with custom path
    #[cfg(test)]
    pub fn new_with_path(path: PathBuf) -> Result<Self> {
        let leetcode_dir = path.join(LEETCODE_DIR);
        let manager = Self {
            workspace_root: path,
            leetcode_dir,
        };
        manager.ensure_directories()?;
        Ok(manager)
    }

    /// Get the .leetcode directory path
    pub fn get_leetcode_dir(&self) -> &Path {
        &self.leetcode_dir
    }

    /// Save problem metadata to cache
    pub fn save_problem(&self, problem: &Problem) -> Result<()> {
        let problems_dir = self.leetcode_dir.join(PROBLEMS_DIR);
        let filename = format!("{}.json", problem.frontend_id);
        let filepath = problems_dir.join(filename);

        let json = serde_json::to_string_pretty(problem)
            .context("Failed to serialize problem")?;

        fs::write(&filepath, json)
            .with_context(|| format!("Failed to write problem to {:?}", filepath))?;

        Ok(())
    }

    /// Save problem detail to cache
    pub fn save_problem_detail(&self, problem: &ProblemDetail) -> Result<()> {
        let problems_dir = self.leetcode_dir.join(PROBLEMS_DIR);
        let filename = format!("{}-detail.json", problem.frontend_id);
        let filepath = problems_dir.join(filename);

        let json = serde_json::to_string_pretty(problem)
            .context("Failed to serialize problem detail")?;

        fs::write(&filepath, json)
            .with_context(|| format!("Failed to write problem detail to {:?}", filepath))?;

        Ok(())
    }

    /// Load problem metadata from cache
    pub fn load_problem(&self, id: &str) -> Result<Option<Problem>> {
        let problems_dir = self.leetcode_dir.join(PROBLEMS_DIR);
        let filename = format!("{}.json", id);
        let filepath = problems_dir.join(filename);

        if !filepath.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&filepath)
            .with_context(|| format!("Failed to read problem from {:?}", filepath))?;

        let problem: Problem = serde_json::from_str(&content)
            .context("Failed to deserialize problem")?;

        Ok(Some(problem))
    }

    /// Load problem detail from cache
    pub fn load_problem_detail(&self, id: &str) -> Result<Option<ProblemDetail>> {
        let problems_dir = self.leetcode_dir.join(PROBLEMS_DIR);
        let filename = format!("{}-detail.json", id);
        let filepath = problems_dir.join(filename);

        if !filepath.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&filepath)
            .with_context(|| format!("Failed to read problem detail from {:?}", filepath))?;

        let problem: ProblemDetail = serde_json::from_str(&content)
            .context("Failed to deserialize problem detail")?;

        Ok(Some(problem))
    }

    /// Create solution file with template
    pub fn create_solution_file(&self, problem: &ProblemDetail, language: &str) -> Result<PathBuf> {
        let solutions_dir = self.leetcode_dir.join(SOLUTIONS_DIR);
        let filename = TemplateGenerator::generate_filename(problem, language);
        let filepath = solutions_dir.join(&filename);

        // Don't overwrite existing files
        if filepath.exists() {
            return Ok(filepath);
        }

        // Generate template
        let template = TemplateGenerator::generate_template(problem, language)
            .ok_or_else(|| anyhow::anyhow!("Unsupported language: {}", language))?;

        fs::write(&filepath, template)
            .with_context(|| format!("Failed to write solution file to {:?}", filepath))?;

        Ok(filepath)
    }

    /// Save configuration
    pub fn save_config(&self, config: &Config) -> Result<()> {
        let config_path = self.leetcode_dir.join(CONFIG_FILE);
        let json = serde_json::to_string_pretty(config)
            .context("Failed to serialize config")?;

        fs::write(&config_path, json)
            .with_context(|| format!("Failed to write config to {:?}", config_path))?;

        Ok(())
    }

    /// Load configuration
    pub fn load_config(&self) -> Result<Config> {
        let config_path = self.leetcode_dir.join(CONFIG_FILE);

        if !config_path.exists() {
            let config = Config::default();
            self.save_config(&config)?;
            return Ok(config);
        }

        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config from {:?}", config_path))?;

        let mut config: Config = serde_json::from_str(&content)
            .context("Failed to deserialize config")?;

        // Update last_updated timestamp
        config.last_updated = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
        self.save_config(&config)?;

        Ok(config)
    }

    /// Update session cookie in config
    pub fn update_session_cookie(&self, session_cookie: String) -> Result<()> {
        let mut config = self.load_config()?;
        config.session_cookie = Some(session_cookie);
        config.last_updated = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
        self.save_config(&config)
    }

    /// List all cached problems
    pub fn list_cached_problems(&self) -> Result<Vec<String>> {
        let problems_dir = self.leetcode_dir.join(PROBLEMS_DIR);
        
        if !problems_dir.exists() {
            return Ok(Vec::new());
        }

        let mut problem_ids = Vec::new();
        
        for entry in fs::read_dir(&problems_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(filename) = path.file_name() {
                if let Some(name) = filename.to_str() {
                    if name.ends_with(".json") && !name.contains("-detail") {
                        let id = name.replace(".json", "");
                        problem_ids.push(id);
                    }
                }
            }
        }
        
        problem_ids.sort_by(|a, b| {
            let a_num: i32 = a.parse().unwrap_or(0);
            let b_num: i32 = b.parse().unwrap_or(0);
            a_num.cmp(&b_num)
        });

        Ok(problem_ids)
    }

    /// Check if problem is cached
    pub fn is_problem_cached(&self, id: &str) -> bool {
        let problems_dir = self.leetcode_dir.join(PROBLEMS_DIR);
        let filename = format!("{}.json", id);
        problems_dir.join(filename).exists()
    }

    /// Check if problem detail is cached
    pub fn is_problem_detail_cached(&self, id: &str) -> bool {
        let problems_dir = self.leetcode_dir.join(PROBLEMS_DIR);
        let filename = format!("{}-detail.json", id);
        problems_dir.join(filename).exists()
    }

    /// Get solution file path if it exists
    pub fn get_solution_file(&self, problem_id: &str, language: &str) -> Option<PathBuf> {
        let solutions_dir = self.leetcode_dir.join(SOLUTIONS_DIR);
        
        // Look for files starting with the problem ID
        if let Ok(entries) = fs::read_dir(&solutions_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    let extension = TemplateGenerator::get_file_extension(language);
                    if filename.starts_with(&format!("{}-", problem_id)) && 
                       filename.ends_with(&format!(".{}", extension)) {
                        return Some(path);
                    }
                }
            }
        }
        
        None
    }

    /// Ensure all required directories exist
    fn ensure_directories(&self) -> Result<()> {
        fs::create_dir_all(&self.leetcode_dir)
            .with_context(|| format!("Failed to create .leetcode directory: {:?}", self.leetcode_dir))?;

        let problems_dir = self.leetcode_dir.join(PROBLEMS_DIR);
        fs::create_dir_all(&problems_dir)
            .with_context(|| format!("Failed to create problems directory: {:?}", problems_dir))?;

        let solutions_dir = self.leetcode_dir.join(SOLUTIONS_DIR);
        fs::create_dir_all(&solutions_dir)
            .with_context(|| format!("Failed to create solutions directory: {:?}", solutions_dir))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Difficulty, Tag, CodeSnippet};
    use tempfile::TempDir;

    fn create_test_problem() -> Problem {
        Problem {
            id: "1".to_string(),
            frontend_id: "1".to_string(),
            title: "Two Sum".to_string(),
            title_slug: "two-sum".to_string(),
            difficulty: Difficulty::Easy,
            tags: vec![
                Tag { name: "Array".to_string(), slug: "array".to_string() },
            ],
            is_paid_only: false,
            acceptance_rate: 49.5,
        }
    }

    fn create_test_problem_detail() -> ProblemDetail {
        ProblemDetail {
            id: "1".to_string(),
            frontend_id: "1".to_string(),
            title: "Two Sum".to_string(),
            title_slug: "two-sum".to_string(),
            content: "Given an array of integers nums and an integer target.".to_string(),
            difficulty: Difficulty::Easy,
            tags: vec![
                Tag { name: "Array".to_string(), slug: "array".to_string() },
            ],
            code_snippets: vec![
                CodeSnippet {
                    lang: "Rust".to_string(),
                    lang_slug: "rust".to_string(),
                    code: "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        \n    }\n}".to_string(),
                },
            ],
            sample_test_case: "nums = [2,7,11,15], target = 9".to_string(),
            example_testcases: None,
        }
    }

    #[test]
    fn test_file_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let fm = FileManager::new_with_path(temp_dir.path().to_path_buf()).unwrap();
        
        assert!(fm.leetcode_dir.exists());
        assert!(fm.leetcode_dir.join(PROBLEMS_DIR).exists());
        assert!(fm.leetcode_dir.join(SOLUTIONS_DIR).exists());
    }

    #[test]
    fn test_save_and_load_problem() {
        let temp_dir = TempDir::new().unwrap();
        let fm = FileManager::new_with_path(temp_dir.path().to_path_buf()).unwrap();
        let problem = create_test_problem();

        // Save problem
        fm.save_problem(&problem).unwrap();

        // Load problem
        let loaded = fm.load_problem("1").unwrap();
        assert!(loaded.is_some());
        let loaded_problem = loaded.unwrap();
        assert_eq!(loaded_problem.title, "Two Sum");
        assert_eq!(loaded_problem.frontend_id, "1");
    }

    #[test]
    fn test_save_and_load_problem_detail() {
        let temp_dir = TempDir::new().unwrap();
        let fm = FileManager::new_with_path(temp_dir.path().to_path_buf()).unwrap();
        let problem = create_test_problem_detail();

        // Save problem detail
        fm.save_problem_detail(&problem).unwrap();

        // Load problem detail
        let loaded = fm.load_problem_detail("1").unwrap();
        assert!(loaded.is_some());
        let loaded_problem = loaded.unwrap();
        assert_eq!(loaded_problem.title, "Two Sum");
        assert!(!loaded_problem.code_snippets.is_empty());
    }

    #[test]
    fn test_create_solution_file() {
        let temp_dir = TempDir::new().unwrap();
        let fm = FileManager::new_with_path(temp_dir.path().to_path_buf()).unwrap();
        let problem = create_test_problem_detail();

        // Create solution file
        let filepath = fm.create_solution_file(&problem, "rust").unwrap();
        assert!(filepath.exists());
        assert!(filepath.to_string_lossy().contains("1-two-sum.rs"));

        // Read file content
        let content = fs::read_to_string(&filepath).unwrap();
        assert!(content.contains("Problem: Two Sum (1)"));
        assert!(content.contains("impl Solution"));
    }

    #[test]
    fn test_config_operations() {
        let temp_dir = TempDir::new().unwrap();
        let fm = FileManager::new_with_path(temp_dir.path().to_path_buf()).unwrap();

        // Load default config
        let config = fm.load_config().unwrap();
        assert_eq!(config.default_language, "rust");
        assert!(config.session_cookie.is_none());

        // Update session cookie
        fm.update_session_cookie("test_cookie".to_string()).unwrap();

        // Load updated config
        let updated_config = fm.load_config().unwrap();
        assert_eq!(updated_config.session_cookie, Some("test_cookie".to_string()));
    }

    #[test]
    fn test_list_cached_problems() {
        let temp_dir = TempDir::new().unwrap();
        let fm = FileManager::new_with_path(temp_dir.path().to_path_buf()).unwrap();

        // Initially empty
        let problems = fm.list_cached_problems().unwrap();
        assert!(problems.is_empty());

        // Save some problems
        let problem1 = create_test_problem();
        let mut problem2 = create_test_problem();
        problem2.frontend_id = "2".to_string();
        problem2.id = "2".to_string();

        fm.save_problem(&problem1).unwrap();
        fm.save_problem(&problem2).unwrap();

        // List cached problems
        let problems = fm.list_cached_problems().unwrap();
        assert_eq!(problems.len(), 2);
        assert!(problems.contains(&"1".to_string()));
        assert!(problems.contains(&"2".to_string()));
    }

    #[test]
    fn test_is_problem_cached() {
        let temp_dir = TempDir::new().unwrap();
        let fm = FileManager::new_with_path(temp_dir.path().to_path_buf()).unwrap();
        let problem = create_test_problem();

        // Initially not cached
        assert!(!fm.is_problem_cached("1"));

        // Cache the problem
        fm.save_problem(&problem).unwrap();

        // Now it should be cached
        assert!(fm.is_problem_cached("1"));
    }

    #[test]
    fn test_get_solution_file() {
        let temp_dir = TempDir::new().unwrap();
        let fm = FileManager::new_with_path(temp_dir.path().to_path_buf()).unwrap();
        let problem = create_test_problem_detail();

        // Initially no solution file
        assert!(fm.get_solution_file("1", "rust").is_none());

        // Create solution file
        fm.create_solution_file(&problem, "rust").unwrap();

        // Now it should be found
        let solution_path = fm.get_solution_file("1", "rust");
        assert!(solution_path.is_some());
        assert!(solution_path.unwrap().to_string_lossy().contains("1-two-sum.rs"));
    }
}
