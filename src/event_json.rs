use std::io::Write as _;
use std::{
    env::{self, current_dir},
    fs,
    path::{Path, PathBuf},
};

use crate::github_env::github_event_path;

// The path on the runner to the file that sets variables from workflow commands.
const ENV_VAR_GITHUB_EVENT_FILE: &str = "GITHUB_ENV";
// The variable we export the branch name to, for later pushing a signed commit
const ENV_GH_DCH_BRANCH_NAME: &str = "DCH_BRANCH_NAME";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct GithubEvent {
    branch_ref: String,
    branch_name: String,
    pr_body: Option<String>,
    pr_link: String,
    pr_number: u64,
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
            let parent = event_path.parent().unwrap_or_else(|| Path::new(""));
            let abs_path = parent.join(&event_path).display().to_string();
            if let Ok(cwd) = current_dir() {
                eprintln!("Current working directory: {}", cwd.display());
            }
            return Err(format!("No github event file at: {abs_path}").into());
        }

        let gh_event = Self::from_path(event_path)?;

        // Export BRANCH_NAME for use in later GitHub Actions steps
        if let Ok(env_path) = env::var(ENV_VAR_GITHUB_EVENT_FILE) {
            let mut env_file = fs::OpenOptions::new().append(true).open(env_path)?;
            let contents = format!("{ENV_GH_DCH_BRANCH_NAME}={}\n", gh_event.branch_name);
            log::info!("Exporting to GitHub environment: '{contents}'");
            env_file.write_all(contents.as_bytes())?;
        } else {
            log::warn!(
                "{ENV_VAR_GITHUB_EVENT_FILE} is not set, cannot locate GitHub environment file"
            );
        }

        Ok(gh_event)
    }

    pub fn from_path(event_json_path: PathBuf) -> Result<Self> {
        let event_json = fs::read_to_string(event_json_path)?;
        Self::new(event_json)
    }

    pub fn new(event_json: String) -> Result<Self> {
        log::debug!("event_json={event_json}");
        let event: serde_json::Value =
            serde_json::from_str(&event_json).map_err(|e| format!("Malformed event JSON: {e}"))?;

        let pr_number: u64 = event["number"]
            .as_u64()
            .expect("Failed to get pull request number");

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

        let pr_link = event["pull_request"]["html_url"]
            .as_str()
            .expect("Failed to retrieve pull request link")
            .to_owned();

        Ok(Self {
            branch_ref,
            branch_name,
            pr_body,
            pr_number,
            pr_link,
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

    pub fn pull_request_number(&self) -> u64 {
        self.pr_number
    }

    pub fn pull_request_link(&self) -> &str {
        &self.pr_link
    }

    pub fn markdown_pull_request_link(&self) -> String {
        format!(
            "[#{pr_number}]({pr_link})",
            pr_number = self.pull_request_number(),
            pr_link = self.pull_request_link()
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_util::*;
    use pretty_assertions::assert_str_eq;
    use tempfile::NamedTempFile;
    use testresult::TestResult;

    #[test]
    fn test_github_event_from_minimal_pr_opened_example() -> TestResult {
        let gh_event = GithubEvent::new(EXAMPLE_PR_OPENED_EVENT_JSON.to_owned())?;

        assert_eq!(gh_event.pr_body().unwrap().len(), 10814);
        assert_str_eq!(gh_event.branch_name(), "bump-changelog-writer");
        assert_eq!(gh_event.pull_request_number(), 9);
        assert_str_eq!(
            gh_event.pull_request_link(),
            "https://github.com/CramBL/dependabot-changelog-writer/pull/1"
        );
        assert_str_eq!(
            &gh_event.markdown_pull_request_link(),
            "[#9](https://github.com/CramBL/dependabot-changelog-writer/pull/1)"
        );

        Ok(())
    }

    #[test]
    fn test_branch_name_exported_to_github_env() -> TestResult {
        // Create a temporary file to simulate the GITHUB_ENV file
        let temp_env_file = NamedTempFile::new()?;
        let env_file_path = temp_env_file.path().to_path_buf();

        // Set the GITHUB_ENV environment variable to point to the temp file
        env::set_var(ENV_VAR_GITHUB_EVENT_FILE, &env_file_path);

        // Set up fake event JSON path
        let fake_event_file = NamedTempFile::new()?;
        fs::write(
            fake_event_file.path(),
            crate::test_util::EXAMPLE_PR_OPENED_EVENT_JSON,
        )?;
        env::set_var("USE_FAKE_EVENT_JSON", fake_event_file.path());

        let gh_event = GithubEvent::load_from_env()?;

        let env_contents = fs::read_to_string(env_file_path)?;
        let expected = format!("{ENV_GH_DCH_BRANCH_NAME}={}\n", gh_event.branch_name());
        assert_str_eq!(
            expected,
            env_contents,
            "Expected env file to contain '{expected}', got:\n{env_contents}"
        );

        Ok(())
    }
}
