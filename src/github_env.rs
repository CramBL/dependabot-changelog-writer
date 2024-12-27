use std::env;

const GITHUB_EVENT_PATH: &str = "GITHUB_EVENT_PATH";
const GITHUB_OUTPUT: &str = "GITHUB_OUTPUT";
const GH_TOKEN: &str = "GH_TOKEN";
const PUSH_TOKEN: &str = "PUSH_TOKEN";

pub fn github_event_path() -> String {
    env::var(GITHUB_EVENT_PATH).expect("GITHUB_EVENT_PATH not set")
}

pub fn github_output() -> Result<String, env::VarError> {
    env::var(GITHUB_OUTPUT)
}

pub fn gh_token() -> String {
    env::var(GH_TOKEN).expect("GH_TOKEN not set")
}

pub fn push_token() -> String {
    env::var(PUSH_TOKEN).expect("PUSH_TOKEN not set")
}
