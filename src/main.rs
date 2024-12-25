use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;

use changelog::add_changes_to_changelog_contents;
use dependabot_changes::parse_body;
use event_json::GithubEvent;

mod changelog;
mod config;
mod dependabot_changes;
mod event_json;
mod git;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn run() -> Result<()> {
    let config = config::Config::new()?;

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
        let changes = parse_body(pr_body);
        let mut changelog_contents = config.read_changelog()?;
        add_changes_to_changelog_contents(
            changes,
            &mut changelog_contents,
            config.version_header(),
            config.section_header(),
        );
        if config.dry_run() {
            log::debug!("Dry run: Skipping commit");
            let orig_changelog = config.read_changelog()?;
            let changeset = difference::Changeset::new(&orig_changelog, &changelog_contents, "\n");
            log::info!("{changeset}");
        } else {
            config.write_changelog(changelog_contents)?;
            git::add_commit_and_push(&config, "origin", event.branch_ref(), event.branch_name())?;
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
            config::Config::exit_with_error(&err.to_string(), &github_output_path);
        } else {
            eprintln!("Error: {err} (Failed to access GITHUB_OUTPUT)");
        }
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
