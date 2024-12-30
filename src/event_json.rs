use std::{env, path::PathBuf};

use crate::github_env::github_event_path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub struct GithubEvent {
    branch_ref: String,
    branch_name: String,
    pr_body: Option<String>,
}

impl GithubEvent {
    /// Read and parse the event file
    pub fn load_from_env() -> Result<Self> {
        // Read the event path environment variable, use the fake one event JSON if it is set
        let event_path = match env::var("USE_FAKE_EVENT_JSON") {
            Ok(fake_event_json) => PathBuf::from(fake_event_json),
            Err(_) => PathBuf::from(github_event_path()),
        };
        log::debug!("event_path={}", event_path.display());

        if !event_path.is_file() {
            return Err(format!("No github event file at: {}", event_path.display()).into());
        }

        Self::from_path(event_path)
    }

    pub fn from_path(event_json_path: PathBuf) -> Result<Self> {
        let event_json = std::fs::read_to_string(event_json_path)?;
        Self::new(event_json)
    }

    pub fn new(event_json: String) -> Result<Self> {
        log::debug!("event_json={event_json}");
        let event: serde_json::Value =
            serde_json::from_str(&event_json).map_err(|e| format!("Malformed event JSON: {e}"))?;

        let branch_ref = if let Some(branch_ref) = event["ref"].as_str() {
            branch_ref.to_owned()
        } else {
            let branch_name = event["pull_request"]["head"]["ref"]
                .as_str()
                .expect("Branch name not found in event JSON");
            format!("refs/heads/{branch_name}")
        };

        let branch_name = branch_ref
            .strip_prefix("refs/heads/")
            .expect("Unexpected branch ref prefix")
            .to_owned();

        let pr_body = event["pull_request"]["body"]
            .as_str()
            .map(|pr_body| pr_body.to_owned());

        Ok(Self {
            branch_ref,
            branch_name,
            pr_body,
        })
    }

    pub fn branch_ref(&self) -> &str {
        &self.branch_ref
    }

    pub fn branch_name(&self) -> &str {
        &self.branch_name
    }

    pub fn pr_body(&self) -> Option<&str> {
        self.pr_body.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;
    use testresult::TestResult;

    #[test]
    fn test_github_event_from_minimal_pr_opened_example() -> TestResult {
        let gh_event = GithubEvent::new(EXAMPLE_PR_OPENED_EVENT_JSON.to_owned())?;

        debug_assert_eq!(gh_event.pr_body().unwrap().len(), 10814);
        debug_assert_eq!(gh_event.branch_name(), "bump-changelog-writer");

        Ok(())
    }
}
