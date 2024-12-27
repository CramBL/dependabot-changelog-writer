use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::{env, io};

use git2::Signature;

use crate::github_env;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    dry_run: bool,
    changelog_path: PathBuf,
    commit_message: String,
    committer_name: String,
    committer_email: String,
    version_header: String,
    section_header: String,
    github_output_path: String,
    github_token: String,
    push_token: String,
}

impl Config {
    pub fn new() -> Result<Self> {
        let mut args = env::args().skip(1);

        let changelog_path = args.next().ok_or("Missing changelog path")?;
        log::debug!("changelog_path={changelog_path}");

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

        let push_changes = args.next().ok_or("Missing push_changes setting")?;
        log::debug!("push_changes={push_changes}");

        let dry_run = !push_changes.eq_ignore_ascii_case("true");

        if args.next().is_some() {
            return Err("Too many arguments provided".into());
        }

        let github_output_path = github_env::github_output()?;

        let github_token = github_env::gh_token();
        let push_token = github_env::push_token();

        if changelog_path.is_empty() {
            Self::exit_with_error("No changelog path specified", &github_output_path);
        }

        let changelog_path = PathBuf::from(changelog_path);
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
            push_token,
        })
    }

    fn write_github_output(error_msg: &str, github_output_path: &str) -> Result<()> {
        fs::write(github_output_path, format!("error={error_msg}"))
            .map_err(|e| format!("Failed to write to GITHUB_OUTPUT: {e}"))?;
        Ok(())
    }

    pub fn exit_with_error(error_msg: &str, github_output_path: &str) -> ! {
        eprintln!("Error: {error_msg}");
        if let Err(e) = Self::write_github_output(error_msg, github_output_path) {
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

    pub fn push_token(&self) -> &str {
        &self.push_token
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

    pub fn dry_run(&self) -> bool {
        self.dry_run
    }
}
