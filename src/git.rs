use auth_git2::GitAuthenticator;
use git2::{Remote, Repository};
use std::error::Error;

use crate::github_env;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub(crate) fn fetch_remote_branch<'r>(
    repo: &'r Repository,
    remote_name: &str,
    branch_name: &str,
) -> Result<Remote<'r>> {
    log::debug!("Finding remote: {remote_name}");
    let mut remote = repo.find_remote(remote_name)?;
    let git_auth = token_git_authenticator(github_env::gh_token());
    log::debug!("Fetching remote branch: {branch_name}");
    git_auth.fetch(
        repo,
        &mut remote,
        &[&format!(
            "refs/heads/{branch_name}:refs/remotes/{remote_name}/{branch_name}"
        )],
        None,
    )?;
    Ok(remote)
}

fn token_git_authenticator(token: &str) -> GitAuthenticator {
    GitAuthenticator::new_empty().add_plaintext_credentials("github.com", "x-access-token", token)
}
