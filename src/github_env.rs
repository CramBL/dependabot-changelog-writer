use std::env;
use std::sync::OnceLock;

const GITHUB_EVENT_PATH: &str = "GITHUB_EVENT_PATH";
const GH_TOKEN: &str = "GH_TOKEN";
const PUSH_TOKEN: &str = "PUSH_TOKEN";

static GITHUB_EVENT_PATH_VAR: OnceLock<String> = OnceLock::new();
static GH_TOKEN_VAR: OnceLock<String> = OnceLock::new();
static PUSH_TOKEN_VAR: OnceLock<String> = OnceLock::new();

pub fn github_event_path() -> &'static String {
    GITHUB_EVENT_PATH_VAR
        .get_or_init(|| env::var(GITHUB_EVENT_PATH).expect("GITHUB_EVENT_PATH not set"))
}

pub fn gh_token() -> &'static String {
    GH_TOKEN_VAR.get_or_init(|| env::var(GH_TOKEN).expect("GH_TOKEN not set"))
}

pub fn push_token() -> &'static String {
    PUSH_TOKEN_VAR.get_or_init(|| env::var(PUSH_TOKEN).expect("PUSH_TOKEN not set"))
}
