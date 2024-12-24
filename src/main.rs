use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::process::ExitCode;
use std::{env, io};

use changelog::add_changes_to_changelog_contents;
use dependabot_changes::parse_body;
use event_json::GithubEvent;
use git2::Signature;

mod changelog;
mod dependabot_changes;
mod event_json;
mod git;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Config {
    dry_run: bool,
    changelog_path: PathBuf,
    commit_message: String,
    committer_name: String,
    committer_email: String,
    version_header: String,
    section_header: String,
    github_output_path: String,
    github_token: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let mut args = env::args().skip(1);

        let first_arg = args.next().ok_or("Missing changelog path")?;
        let dry_run = first_arg == "--dry-run";
        log::debug!("dry_run={dry_run}");

        let changelog_path_str = if dry_run {
            args.next().ok_or("Missing changelog path")?
        } else {
            first_arg
        };
        log::debug!("changelog_path_str={changelog_path_str}");

        let commit_message = args.next().ok_or("Missing commit message")?;
        log::debug!("commit_message={commit_message}");

        let committer_name = args.next().ok_or("Missing committer name")?;
        log::debug!("committer_name={committer_name}");

        let committer_email = args.next().ok_or("Missing committer email")?;
        log::debug!("committer_email={committer_email}");

        let version_header = args.next().ok_or("Missing section header")?;
        log::debug!("version_header={version_header}");

        let section_header = args.next().ok_or("Missing section header")?;
        log::debug!("section_header={section_header}");

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
            dry_run,
            changelog_path,
            commit_message,
            committer_name,
            committer_email,
            version_header,
            section_header,
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

    pub fn version_header(&self) -> &str {
        &self.version_header
    }

    pub fn section_header(&self) -> &str {
        &self.section_header
    }

    pub fn changelog_path(&self) -> &Path {
        &self.changelog_path
    }

    pub fn read_changelog(&self) -> io::Result<String> {
        fs::read_to_string(&self.changelog_path)
    }

    pub fn write_changelog(&self, contents: String) -> io::Result<()> {
        fs::write(&self.changelog_path, contents)
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
    let event = GithubEvent::new(event_path)?;

    if let Some(pr_body) = event.pr_body() {
        log::debug!("Pull Request Body:\n{pr_body}");
        let changes_md = parse_body(pr_body);
        let mut changelog_contents = config.read_changelog()?;
        add_changes_to_changelog_contents(
            changes_md,
            &mut changelog_contents,
            config.version_header(),
            config.section_header(),
        );
        if config.dry_run {
            log::debug!("Dry run: Skipping commit");
            let orig_changelog = config.read_changelog()?;
            let changeset = difference::Changeset::new(&orig_changelog, &changelog_contents, "\n");
            log::info!("{changeset}");
        } else {
            config.write_changelog(changelog_contents)?;
            git::add_commit_and_push(
                config.github_token(),
                config.commit_signature()?,
                config.changelog_path(),
                config.commit_message(),
                "origin",
                event.branch_ref(),
                event.branch_name(),
            )?;
        }
    } else {
        log::warn!("Pull request body is empty");
    }

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
