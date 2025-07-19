use anyhow::{Context, Result};
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

#[derive(Debug, Serialize, Deserialize)]
struct AuthConfig {
    session_cookie: String,
    created_at: String,
}

pub struct AuthManager {
    config_path: PathBuf,
}

impl AuthManager {
    pub fn new(config_dir: &PathBuf) -> Self {
        let config_path = config_dir.join("auth.json");
        Self { config_path }
    }

    /// Save session cookie with basic encoding
    pub fn save_session(&self, session_cookie: &str) -> Result<()> {
        // Create config directory if it doesn't exist
        if let Some(parent) = self.config_path.parent() {
            fs::create_dir_all(parent)
                .context("Failed to create config directory")?;
        }

        let auth_config = AuthConfig {
            session_cookie: BASE64.encode(session_cookie),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let config_json = serde_json::to_string_pretty(&auth_config)
            .context("Failed to serialize auth config")?;

        fs::write(&self.config_path, config_json)
            .context("Failed to write auth config")?;

        // Set restrictive permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&self.config_path)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&self.config_path, perms)?;
        }

        Ok(())
    }

    /// Load and decode session cookie
    pub fn load_session(&self) -> Result<Option<String>> {
        if !self.config_path.exists() {
            return Ok(None);
        }

        let config_content = fs::read_to_string(&self.config_path)
            .context("Failed to read auth config")?;

        let auth_config: AuthConfig = serde_json::from_str(&config_content)
            .context("Failed to parse auth config")?;

        let decoded_cookie = BASE64.decode(&auth_config.session_cookie)
            .context("Failed to decode session cookie")?;

        let cookie_str = String::from_utf8(decoded_cookie)
            .context("Invalid UTF-8 in decoded cookie")?;

        Ok(Some(cookie_str))
    }

    /// Verify session by making an authenticated API call
    pub fn verify_session(&self, api_client: &crate::api::LeetCodeApi) -> Result<bool> {
        if let Some(session_cookie) = self.load_session()? {
            // Try to fetch user profile to verify session
            match api_client.verify_authentication(&session_cookie) {
                Ok(is_valid) => Ok(is_valid),
                Err(_) => Ok(false),
            }
        } else {
            Ok(false)
        }
    }

    /// Check if user is currently authenticated
    pub fn is_authenticated(&self, api_client: &crate::api::LeetCodeApi) -> bool {
        self.verify_session(api_client).unwrap_or(false)
    }

    /// Clear stored authentication
    pub fn logout(&self) -> Result<()> {
        if self.config_path.exists() {
            fs::remove_file(&self.config_path)
                .context("Failed to remove auth config")?;
        }
        Ok(())
    }

    /// Get session cookie for API calls
    pub fn get_session_cookie(&self) -> Result<Option<String>> {
        self.load_session()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_auth_manager() -> (AuthManager, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().to_path_buf();
        let auth_manager = AuthManager::new(&config_dir);
        (auth_manager, temp_dir)
    }

    #[test]
    fn test_save_and_load_session() {
        let (auth_manager, _temp_dir) = create_test_auth_manager();
        let test_cookie = "LEETCODE_SESSION=test_session_value";

        // Save session
        assert!(auth_manager.save_session(test_cookie).is_ok());

        // Load session
        let loaded_session = auth_manager.load_session().unwrap();
        assert!(loaded_session.is_some());
        assert_eq!(loaded_session.unwrap(), test_cookie);
    }

    #[test]
    fn test_load_nonexistent_session() {
        let (auth_manager, _temp_dir) = create_test_auth_manager();

        let loaded_session = auth_manager.load_session().unwrap();
        assert!(loaded_session.is_none());
    }

    #[test]
    fn test_logout() {
        let (auth_manager, _temp_dir) = create_test_auth_manager();
        let test_cookie = "LEETCODE_SESSION=test_session_value";

        // Save and verify session exists
        auth_manager.save_session(test_cookie).unwrap();
        assert!(auth_manager.load_session().unwrap().is_some());

        // Logout and verify session is cleared
        assert!(auth_manager.logout().is_ok());
        assert!(auth_manager.load_session().unwrap().is_none());
    }

    #[test]
    fn test_get_session_cookie() {
        let (auth_manager, _temp_dir) = create_test_auth_manager();
        let test_cookie = "LEETCODE_SESSION=test_session_value";

        // Initially no session cookie
        assert!(auth_manager.get_session_cookie().unwrap().is_none());

        // Save session and get cookie
        auth_manager.save_session(test_cookie).unwrap();
        let retrieved_cookie = auth_manager.get_session_cookie().unwrap();
        assert!(retrieved_cookie.is_some());
        assert_eq!(retrieved_cookie.unwrap(), test_cookie);
    }

    #[test]
    fn test_config_file_permissions() {
        let (auth_manager, _temp_dir) = create_test_auth_manager();
        let test_cookie = "LEETCODE_SESSION=test_session_value";

        auth_manager.save_session(test_cookie).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&auth_manager.config_path).unwrap();
            let permissions = metadata.permissions();
            assert_eq!(permissions.mode() & 0o777, 0o600);
        }
    }

    #[test]
    fn test_invalid_base64_handling() {
        let (auth_manager, _temp_dir) = create_test_auth_manager();
        
        // Manually write invalid JSON to test error handling
        let invalid_config = r#"{"session_cookie": "invalid_base64!", "created_at": "2023-01-01T00:00:00Z"}"#;
        fs::write(&auth_manager.config_path, invalid_config).unwrap();

        let result = auth_manager.load_session();
        assert!(result.is_err());
    }
}
