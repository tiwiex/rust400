use std::env;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SessionContext {
    current_user: String,
    job_name: String,
    started_at_epoch_seconds: u64,
    status: &'static str,
}

impl SessionContext {
    pub fn new() -> Self {
        let started_at_epoch_seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_secs();
        let current_user = env::var("USER")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| "MW".to_string())
            .to_ascii_uppercase();
        let job_name = format!("QPADEV{:04}", process::id() % 10_000);

        Self {
            current_user,
            job_name,
            started_at_epoch_seconds,
            status: "ACTIVE",
        }
    }

    pub fn current_user(&self) -> &str {
        &self.current_user
    }

    pub fn job_name(&self) -> &str {
        &self.job_name
    }

    pub fn started_at_epoch_seconds(&self) -> u64 {
        self.started_at_epoch_seconds
    }

    pub fn status(&self) -> &str {
        self.status
    }
}

impl Default for SessionContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::SessionContext;

    #[test]
    fn new_session_has_identity_and_status() {
        let session = SessionContext::new();

        assert!(!session.current_user().is_empty());
        assert!(session.job_name().starts_with("QPADEV"));
        assert!(session.started_at_epoch_seconds() > 0);
        assert_eq!(session.status(), "ACTIVE");
    }
}
