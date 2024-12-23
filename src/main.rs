use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::process::ExitCode;

use git2::Signature;

mod dependabot_changes;
mod git;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Config {
    changelog_path: PathBuf,
    commit_message: String,
    committer_name: String,
    committer_email: String,
    github_output_path: String,
    github_token: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let mut args = env::args().skip(1);

        // Take ownership of each argument directly
        let changelog_path_str = args.next().ok_or("Missing changelog path")?;
        log::debug!("changelog_path_str={changelog_path_str}");
        let commit_message = args.next().ok_or("Missing commit message")?;
        log::debug!("commit_message={commit_message}");

        let committer_name = args.next().ok_or("Missing committer name")?;
        log::debug!("committer_name={committer_name}");

        let committer_email = args.next().ok_or("Missing committer email")?;
        log::debug!("committer_email={committer_email}");

        if args.next().is_some() {
            return Err("Too many arguments provided".into());
        }

        let github_output_path =
            env::var("GITHUB_OUTPUT").expect("GITHUB_OUTPUT environment variable not set");

        let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

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
            committer_name,
            committer_email,
            github_output_path,
            github_token,
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

    pub fn commit_signature(&self) -> std::result::Result<Signature, git2::Error> {
        Signature::now(&self.committer_name, &self.committer_email)
    }

    pub fn github_token(&self) -> &str {
        &self.github_token
    }

    pub fn commit_message(&self) -> &str {
        &self.commit_message
    }
}

fn run() -> Result<()> {
    let config = Config::new()?;

    // Read the event path environment variable
    let event_path = env::var("GITHUB_EVENT_PATH").expect("GITHUB_EVENT_PATH not set");
    log::debug!("event_path={event_path}");
    let event_path = PathBuf::from(event_path);

    if !event_path.is_file() {
        config.exit(&format!(
            "No github event file at: {}",
            event_path.display()
        ));
    }

    // Read and parse the event file
    let event_json = std::fs::read_to_string(event_path)?;
    log::debug!("event_json={event_json}");
    let event: serde_json::Value = serde_json::from_str(&event_json)?;

    // Extract the PR body
    if let Some(body) = event["pull_request"]["body"].as_str() {
        println!("Pull Request Body:\n{}", body);
    } else {
        println!("Pull Request has no body");
    }
    // 1. Parse body for version changes

    // 2. Locate changelog section and write the changes

    let git_ref = event["ref"]
        .as_str()
        .ok_or("Branch name not found in event JSON")?;

    let example_commit = "example contents";
    fs::write("example_file.txt", example_commit)?;

    let file_path = "example_file.txt";

    git::add_commit_and_push(
        config.github_token(),
        config.commit_signature()?,
        file_path,
        config.commit_message(),
        "origin",
        git_ref,
    )?;

    Ok(())
}

fn main() -> ExitCode {
    env_logger::init();
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
