use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::process::ExitCode;

use octocrab::Octocrab;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Config {
    changelog_path: PathBuf,
    commit_message: String,
    github_output_path: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let mut args = env::args().skip(1);

        // Take ownership of each argument directly
        let changelog_path_str = args.next().ok_or("Missing changelog path")?;
        let commit_message = args.next().ok_or("Missing commit message")?;

        if args.next().is_some() {
            return Err("Too many arguments provided".into());
        }

        let github_output_path =
            env::var("GITHUB_OUTPUT").map_err(|_| "GITHUB_OUTPUT environment variable not set")?;

        if changelog_path_str.is_empty() {
            Self::exit_with_error("No changelog path specified", &github_output_path);
        }

        let changelog_path = PathBuf::from(changelog_path_str);
        if !changelog_path.is_file() {
            Self::exit_with_error(
                "The specified changelog could not be found",
                &github_output_path,
            );
        }

        Ok(Self {
            changelog_path,
            commit_message,
            github_output_path,
        })
    }

    fn write_github_output(error_msg: &str, github_output_path: &str) -> Result<()> {
        fs::write(github_output_path, format!("error={error_msg}"))
            .map_err(|e| format!("Failed to write to GITHUB_OUTPUT: {e}"))?;
        Ok(())
    }

    fn exit_with_error(error_msg: &str, github_output_path: &str) -> ! {
        eprintln!("Error: {error_msg}");
        if let Err(e) = Self::write_github_output(error_msg, github_output_path) {
            eprintln!("Additional error when writing output: {e}");
        }
        process::exit(1);
    }

    pub fn exit(&self, error_msg: &str) -> ! {
        eprintln!("Error: {error_msg}");
        if let Err(e) = Self::write_github_output(error_msg, &self.github_output_path) {
            eprintln!("Additional error when writing output: {e}");
        }
        process::exit(1);
    }
}

struct GitHubApi {
    octocrab: Octocrab,
}

impl GitHubApi {
    pub fn new(github_token: String) -> Result<Self> {
        let octocrab = Octocrab::builder().personal_token(github_token).build()?;

        Ok(Self { octocrab })
    }
}

fn run() -> Result<()> {
    let config = Config::new()?;

    // Get the GitHub token from environment
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let github_api = GitHubApi::new(token)?;

    // Read the event path environment variable
    let event_path = env::var("GITHUB_EVENT_PATH").expect("GITHUB_EVENT_PATH not set");
    let event_path = PathBuf::from(event_path);
    if !event_path.is_file() {
        config.exit(&format!(
            "No github event file at: {}",
            event_path.display()
        ));
    }

    // Read and parse the event file
    let event_json = std::fs::read_to_string(event_path)?;
    let event: serde_json::Value = serde_json::from_str(&event_json)?;

    // Extract the PR body
    if let Some(body) = event["pull_request"]["body"].as_str() {
        println!("Pull Request Body:\n{}", body);
    } else {
        println!("Pull Request has no body");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(err) = run() {
        if let Ok(github_output_path) = env::var("GITHUB_OUTPUT") {
            Config::exit_with_error(&err.to_string(), &github_output_path);
        } else {
            eprintln!("Error: {err} (Failed to access GITHUB_OUTPUT)");
        }
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
