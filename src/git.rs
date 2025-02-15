use auth_git2::GitAuthenticator;
use git2::{Remote, Repository};
use std::error::Error;
use std::path::{Path, PathBuf};

use crate::github_env;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Sanitize the path such that we can add it to the repo git index
// there's strict rules that requires the path to be relative to the repo
// and it cannot start with '.' or '..'
// e.g. './CHANGELOG.md' is NOT valid
fn sanitize_path(file_path: &Path) -> Result<PathBuf> {
    log::debug!("Sanitizing path: {:?}", file_path);
    let path_str = file_path.to_str().ok_or("Invalid path")?;
    let clean_path = path_str.trim_start_matches("./");
    log::debug!("Sanitized path: {:?}", clean_path);
    Ok(PathBuf::from(clean_path))
}

pub fn add_commit_and_push(
    config: &crate::config::Config,
    remote_name: &str,
    branch_ref: &str,
    branch_name: &str,
) -> Result<()> {
    log::debug!("Opening repository in current directory");
    let repo = Repository::open(".")?;
    // Fetch the remote branch first to ensure we have it locally
    // this is necessary in actions triggered by an opened PR because
    // they per default checkout branches detached from HEAD

    log::debug!("Fetching remote branch: {}", branch_name);
    let mut remote = fetch_remote_branch(&repo, remote_name, branch_name)?;

    let index_tree = stage_changes(&repo, config.changelog_path())?;

    // Skip commit if no changes
    log::debug!("Checking if there are any changes to commit");
    if !has_changes(&repo, &index_tree)? {
        log::info!("No changes to commit");
        return Ok(());
    }

    create_commit(&repo, config, &index_tree)?;

    // Push changes to the remote
    log::debug!("Pushing changes to remote: {}", branch_ref);
    push_to_remote(github_env::push_token(), &repo, &mut remote, branch_ref)?;

    Ok(())
}

fn fetch_remote_branch<'r>(
    repo: &'r Repository,
    remote_name: &str,
    branch_name: &str,
) -> Result<Remote<'r>> {
    log::debug!("Finding remote: {}", remote_name);
    let mut remote = repo.find_remote(remote_name)?;
    let git_auth = token_git_authenticator(github_env::gh_token());
    log::debug!("Fetching remote branch: {}", branch_name);
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

fn stage_changes<'r>(repo: &'r Repository, changelog_path: &Path) -> Result<git2::Tree<'r>> {
    log::debug!("Staging changes for path: {:?}", changelog_path);
    let mut index = repo.index()?;
    let clean_path = sanitize_path(changelog_path)?;
    log::debug!("Adding path to index: {:?}", clean_path);
    index.add_path(&clean_path)?;
    index.write()?;

    let tree_oid = index.write_tree()?;
    log::debug!("Tree OID: {:?}", tree_oid);
    let tree = repo.find_tree(tree_oid)?;
    log::debug!("Tree found: {:?}", tree);
    Ok(tree)
}

fn has_changes(repo: &Repository, index_tree: &git2::Tree) -> Result<bool> {
    if repo.head().is_err() {
        log::debug!("Repository has no commits yet, checking if index tree has any entries");
        return Ok(index_tree.iter().count() > 0);
    }

    let head_tree = repo.head()?.peel_to_tree()?;
    let diff = repo.diff_tree_to_tree(Some(&head_tree), Some(index_tree), None)?;
    log::debug!("Number of deltas in diff: {}", diff.deltas().len());
    Ok(diff.deltas().len() > 0)
}

fn create_commit(
    repo: &Repository,
    config: &crate::config::Config,
    tree: &git2::Tree,
) -> Result<()> {
    log::debug!("Creating commit with message: {}", config.commit_message());
    let head_ref = repo.head()?;
    let head_commit = head_ref.peel_to_commit()?;
    log::debug!("current head_commit: {head_commit:?}");
    let commit_signature = config.commit_signature()?;
    repo.commit(
        Some("HEAD"),
        &commit_signature,
        &commit_signature,
        config.commit_message(),
        tree,
        &[&head_commit],
    )?;

    log::debug!("new head_commit: {:?}", repo.head()?.peel_to_commit()?);
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
    log::debug!("Pushing to remote: {}", git_ref);
    let git_auth = token_git_authenticator(push_token);

    if let Err(e) = git_auth.push(repo, remote, &[&format!("HEAD:{git_ref}")]) {
        log::warn!("Push failed, does this job have write permissions? - Retrying with force push");
        if let Err(e) = git_auth.push(repo, remote, &[&format!("+HEAD:{git_ref}")]) {
            return Err(e.into());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::{self, CommitSettings, VersionHeader};
    use crate::dependabot_changes::entry_pattern::EntryPattern;

    use super::*;
    use git2::Repository;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;
    use testresult::TestResult;

    // Helper function to create a test config
    fn create_test_config() -> config::Config {
        config::Config::new(
            false,
            PathBuf::from("CHANGELOG.md"),
            EntryPattern::default(),
            CommitSettings {
                message: "Test commit message".into(),
                author: "Test User".into(),
                author_email: "test@example.com".into(),
            },
            VersionHeader::Unreleased,
            "Dependencies".into(),
        )
    }

    struct TestRepo {
        temp_dir: TempDir, // Keep in struct to maintain directory lifetime
        repo: Repository,
    }

    impl TestRepo {
        fn new() -> Result<Self> {
            let temp_dir = TempDir::new()?;
            let repo = Repository::init(temp_dir.path())?;

            // Set up basic git config for commits
            let mut config = repo.config()?;
            config.set_str("user.name", "Test User")?;
            config.set_str("user.email", "test@example.com")?;

            Ok(TestRepo { temp_dir, repo })
        }

        fn create_file(&self, name: &str, content: &str) -> Result<()> {
            let path = self.temp_dir.path().join(name);
            fs::write(&path, content)?;
            Ok(())
        }

        fn create_initial_commit(&self) -> Result<()> {
            let signature = git2::Signature::now("Test User", "test@example.com")?;
            let tree_id = {
                let mut index = self.repo.index()?;
                index.write_tree()?
            };
            let tree = self.repo.find_tree(tree_id)?;
            self.repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                "Initial commit",
                &tree,
                &[],
            )?;
            Ok(())
        }
    }

    #[test]
    fn test_has_changes_initial_repo_no_changes() -> TestResult {
        let test_repo = TestRepo::new()?;

        // Create an empty tree from the empty index
        let mut index = test_repo.repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = test_repo.repo.find_tree(tree_id)?;

        // In a fresh repo with no commits, comparing against an empty tree
        // should show no changes
        assert!(!has_changes(&test_repo.repo, &tree)?);

        Ok(())
    }

    #[test]
    fn test_has_changes_create_file_has_changes() -> TestResult {
        let test_repo = TestRepo::new()?;

        // Create an empty tree from the empty index
        let mut index = test_repo.repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = test_repo.repo.find_tree(tree_id)?;

        // In a fresh repo with no commits and no changes,
        // an empty tree should show no changes
        assert!(!has_changes(&test_repo.repo, &tree)?);

        // Add a test file to verify we detect changes
        test_repo.create_file("test.txt", "content")?;
        index.add_path(Path::new("test.txt"))?;
        let tree_id = index.write_tree()?;
        let tree_with_changes = test_repo.repo.find_tree(tree_id)?;

        // Now we should detect changes
        assert!(has_changes(&test_repo.repo, &tree_with_changes)?);

        Ok(())
    }

    #[test]
    fn test_stage_changes_with_new_file() -> TestResult {
        let test_repo = TestRepo::new()?;
        test_repo.create_initial_commit()?;

        // Create and stage a changelog file
        test_repo.create_file("CHANGELOG.md", "# Changelog\n")?;
        let tree = stage_changes(&test_repo.repo, Path::new("CHANGELOG.md"))?;

        assert!(has_changes(&test_repo.repo, &tree)?);
        Ok(())
    }

    #[test]
    fn test_stage_changes_with_no_changes() -> TestResult {
        let test_repo = TestRepo::new()?;
        test_repo.create_initial_commit()?;

        // Create and commit a changelog file
        test_repo.create_file("CHANGELOG.md", "# Changelog\n")?;
        let tree = stage_changes(&test_repo.repo, Path::new("CHANGELOG.md"))?;
        create_commit(&test_repo.repo, &create_test_config(), &tree)?;

        // Try to stage the same content again
        let tree = stage_changes(&test_repo.repo, Path::new("CHANGELOG.md"))?;
        assert!(!has_changes(&test_repo.repo, &tree)?);
        Ok(())
    }

    #[test]
    fn test_stage_changes_with_modifications() -> TestResult {
        let test_repo = TestRepo::new()?;
        test_repo.create_initial_commit()?;

        // Create and commit initial changelog
        test_repo.create_file("CHANGELOG.md", "# Changelog\n")?;
        let tree = stage_changes(&test_repo.repo, Path::new("CHANGELOG.md"))?;
        create_commit(&test_repo.repo, &create_test_config(), &tree)?;

        // Modify the changelog
        test_repo.create_file("CHANGELOG.md", "# Changelog\n## New Version\n")?;
        let tree = stage_changes(&test_repo.repo, Path::new("CHANGELOG.md"))?;
        assert!(has_changes(&test_repo.repo, &tree)?);
        Ok(())
    }

    #[test]
    fn test_sanitize_path() -> TestResult {
        assert_eq!(
            sanitize_path(Path::new("./CHANGELOG.md"))?,
            PathBuf::from("CHANGELOG.md")
        );
        assert_eq!(
            sanitize_path(Path::new("CHANGELOG.md"))?,
            PathBuf::from("CHANGELOG.md")
        );
        Ok(())
    }
}
