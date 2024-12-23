use auth_git2::GitAuthenticator;
use git2::{Credentials, Direction, RemoteCallbacks, Repository, Signature};
use std::error::Error;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn add_commit_and_push(
    repo_path: &str,
    file_path: &str,
    commit_message: &str,
    remote_name: &str,
    git_ref: &str,
) -> Result<()> {
    // Open the repository
    let repo = Repository::open(repo_path)?;

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

    // Retrieve the author and committer signatures
    let signature = Signature::now("Your Name", "your.email@example.com")?;

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
    push_to_remote(&repo, remote_name, git_ref)?;

    Ok(())
}

fn push_to_remote(repo: &Repository, remote_name: &str, git_ref: &str) -> Result<()> {
    // Find the remote
    let mut remote = repo.find_remote(remote_name)?;
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let git_auth = GitAuthenticator::default().add_plaintext_credentials("github.com", token, "");

    if let Err(e) = git_auth.push(repo, &mut remote, &[&format!("{git_ref}:{git_ref}")]) {
        eprintln!("Error: Push failed, does this job have write permissions?");
        return Err(e.into());
    }

    println!("Successfully pushed to remote '{remote_name}'");

    Ok(())
}
