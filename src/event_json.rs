use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub struct GithubEvent {
    branch_ref: String,
    pr_body: Option<String>,
}

impl GithubEvent {
    pub fn new(event_json_path: PathBuf) -> Result<Self> {
        let event_json = std::fs::read_to_string(event_json_path)?;
        log::debug!("event_json={event_json}");
        let event: serde_json::Value =
            serde_json::from_str(&event_json).map_err(|e| format!("Malformed event JSON: {e}"))?;

        let branch_ref = event["ref"]
            .as_str()
            .unwrap_or_else(|| {
                event["pull_request"]["head"]["ref"]
                    .as_str()
                    .expect("Branch name not found in event JSON")
            })
            .to_owned();

        let pr_body = if let Some(pr_body) = event["pull_request"]["body"].as_str() {
            Some(pr_body.to_owned())
        } else {
            None
        };

        Ok(Self {
            branch_ref,
            pr_body,
        })
    }

    pub fn branch_ref(&self) -> &str {
        &self.branch_ref
    }

    pub fn pr_body(&self) -> Option<&str> {
        self.pr_body.as_deref()
    }
}
