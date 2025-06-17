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
            &event.markdown_pull_request_link(),
            config.entry_pattern(),
            config.version_header(),
            config.section_header(),
        );

        let orig_changelog = config.read_changelog()?;
        util::print_diff(&orig_changelog, &changelog_contents);
        if config.dry_run() {
            log::debug!("Dry run: Skipping writing to changelog");
            log::debug!("Dry run: Skipping commit & push");
        } else {
            config.write_changelog(changelog_contents)?;
            log::debug!("Opening repository in current directory");
            let repo = git2::Repository::open(".")?;
            // Fetch the remote branch first to ensure we have it locally
            // this is necessary in actions triggered by an opened PR because
            // they per default checkout branches detached from HEAD
            log::debug!("Fetching remote branch: {}", event.branch_name());
            let _remote = git::fetch_remote_branch(&repo, "origin", event.branch_name())?;

            // if config.push_changes() {
            //     git::add_commit_and_push(&config, remote, event.branch_ref())?;
            // } else {
            //     log::debug!("push-changes disabled: Skipping commit & push");
            // }
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
