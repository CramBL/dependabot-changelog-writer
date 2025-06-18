use std::io::Write as _;
use std::sync::OnceLock;
use std::{env, fs};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const GITHUB_EVENT_PATH: &str = "GITHUB_EVENT_PATH";
const ENV_VAR_GH_TOKEN: &str = "GH_TOKEN";
// The path on the runner to the file that sets variables from workflow commands.
pub const ENV_VAR_GITHUB_EVENT_FILE: &str = "GITHUB_ENV";
// The variable we export the branch name to, for later pushing a signed commit
pub const ENV_GH_DCH_BRANCH_NAME: &str = "DCH_BRANCH_NAME";
// Store '1' if any changes to the changelog has been made
pub const ENV_GH_DCH_CHANGES_MADE: &str = "DCH_CHANGES_MADE";

static GH_TOKEN_VAR: OnceLock<String> = OnceLock::new();
pub fn gh_token() -> &'static String {
    GH_TOKEN_VAR.get_or_init(|| env::var(ENV_VAR_GH_TOKEN).expect("GH_TOKEN not set"))
}

static GITHUB_EVENT_PATH_VAR: OnceLock<String> = OnceLock::new();

pub fn github_event_path() -> &'static String {
    GITHUB_EVENT_PATH_VAR
        .get_or_init(|| env::var(GITHUB_EVENT_PATH).expect("GITHUB_EVENT_PATH not set"))
}

pub fn write_to_github_env(env_var: &str, contents: &str) -> Result<()> {
    if let Ok(env_path) = env::var(ENV_VAR_GITHUB_EVENT_FILE) {
        let mut env_file = fs::OpenOptions::new().append(true).open(env_path)?;
        let contents = format!("{env_var}={contents}\n");
        log::info!("Exporting to GitHub environment: '{contents}'");
        env_file.write_all(contents.as_bytes())?;
    } else {
        log::warn!("{ENV_VAR_GITHUB_EVENT_FILE} is not set, cannot locate GitHub environment file");
    }
    Ok(())
}
