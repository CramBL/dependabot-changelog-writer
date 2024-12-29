use std::error::Error;
use std::process::ExitCode;

use changelog::add_changes_to_changelog_contents;
use dependabot_changes::parse_body;
use event_json::GithubEvent;

mod changelog;
mod config;
mod dependabot_changes;
mod event_json;
mod git;

#[cfg(test)]
mod test_util;
mod util;

mod github_env;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn run() -> Result<()> {
    let config = config::Config::from_env_args()?;
    let event = GithubEvent::load_from_env()?;

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
            util::print_diff(&orig_changelog, &changelog_contents);
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
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {e}");
            ExitCode::FAILURE
        }
    }
}
