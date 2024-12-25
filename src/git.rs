use auth_git2::GitAuthenticator;
use git2::{Remote, Repository};
use std::error::Error;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Sanitize the path such that we can add it to the repo git index
// there's strict rules that requires the path to be relative to the repo
// and it cannot start with '.' or '..'
// e.g. './CHANGELOG.md' is NOT valid
fn sanitize_path(file_path: &Path) -> Result<PathBuf> {
    let path_str = file_path.to_str().ok_or("Invalid path")?;
    let clean_path = path_str.trim_start_matches("./").trim_start_matches("../");
    Ok(PathBuf::from(clean_path))
}

pub fn add_commit_and_push(
    config: &crate::config::Config,
    remote_name: &str,
    branch_ref: &str,
    branch_name: &str,
) -> Result<()> {
    let repo = Repository::open(".")?;

    // Fetch the remote branch first to ensure we have it locally
    // this is necessary in actions triggered by an opened PR because
    // they per default checkout branches detached from HEAD
    let mut remote = repo.find_remote(remote_name)?;

    let git_auth = token_git_authenticator(config.github_token());
    git_auth.fetch(
        &repo,
        &mut remote,
        &[&format!(
            "refs/heads/{branch_name}:refs/remotes/{remote_name}/{branch_name}"
        )],
        None,
    )?;

    let mut index = repo.index()?;

    // Add the file to the index
    let clean_path = sanitize_path(config.changelog_path())?;
    index.add_path(&clean_path)?;
    index.write()?;

    // Write the index to a tree
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    // Get the HEAD reference
    let head_ref = repo.head()?;
    let head_commit = head_ref.peel_to_commit()?;

    // Create the commit

    let commit_signature = config.commit_signature()?;
    repo.commit(
        Some("HEAD"),            // Update the HEAD reference
        &commit_signature,       // Author signature
        &commit_signature,       // Committer signature
        config.commit_message(), // Commit message
        &tree,                   // Tree object
        &[&head_commit],         // Parent commit(s)
    )?;

    // Push changes to the remote
    push_to_remote(config.push_token(), &repo, &mut remote, branch_ref)?;

    Ok(())
}

fn token_git_authenticator(token: &str) -> GitAuthenticator {
    GitAuthenticator::new_empty().add_plaintext_credentials("github.com", "x-access-token", token)
}

fn push_to_remote(
    push_token: &str,
    repo: &Repository,
    remote: &mut Remote,
    git_ref: &str,
) -> Result<()> {
    let git_auth = token_git_authenticator(push_token);

    if let Err(e) = git_auth.push(repo, remote, &[&format!("HEAD:{git_ref}")]) {
        log::error!("Push failed, does this job have write permissions?");
        return Err(e.into());
    }
    Ok(())
}
