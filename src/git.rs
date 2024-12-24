use auth_git2::GitAuthenticator;
use git2::{Repository, Signature};
use std::error::Error;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn add_commit_and_push(
    github_token: &str,
    signature: Signature,
    file_path: &Path,
    commit_message: &str,
    remote_name: &str,
    git_ref: &str,
) -> Result<()> {
    // Open the repository
    let repo = Repository::open(".")?;

    // Get the index
    let mut index = repo.index()?;

    // Add the file to the index
    index.add_path(Path::new(file_path))?;
    index.write()?;

    // Write the index to a tree
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;

    // Get the HEAD reference
    let head_ref = repo.head()?;
    let head_commit = head_ref.peel_to_commit()?;

    // Create the commit
    repo.commit(
        Some("HEAD"),    // Update the HEAD reference
        &signature,      // Author signature
        &signature,      // Committer signature
        commit_message,  // Commit message
        &tree,           // Tree object
        &[&head_commit], // Parent commit(s)
    )?;

    println!("Successfully committed: {commit_message}");

    // Push changes to the remote
    push_to_remote(github_token, &repo, remote_name, git_ref)?;

    Ok(())
}

fn push_to_remote(
    github_token: &str,
    repo: &Repository,
    remote_name: &str,
    git_ref: &str,
) -> Result<()> {
    let git_auth = GitAuthenticator::new_empty().add_plaintext_credentials(
        "github.com",
        "x-access-token",
        github_token,
    );

    let mut remote = repo.find_remote(remote_name)?;

    if let Err(e) = git_auth.push(repo, &mut remote, &[&format!("{git_ref}:{git_ref}")]) {
        eprintln!("Error: Push failed, does this job have write permissions?");
        return Err(e.into());
    }

    println!("Successfully pushed to remote '{remote_name}'");

    Ok(())
}
