use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::{env, io};

use git2::Signature;

use crate::dependabot_changes::entry_pattern::EntryPattern;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn next_arg_trimmed(args: &mut impl Iterator<Item = String>) -> Option<String> {
    Some(args.next()?.trim().to_owned())
}

#[derive(Debug)]
pub enum VersionHeader {
    Unreleased,
    Custom(String),
}

impl VersionHeader {
    pub fn new(version_header: String) -> Self {
        if version_header.eq_ignore_ascii_case("unreleased") {
            Self::Unreleased
        } else {
            Self::Custom(version_header)
        }
    }
}

#[derive(Debug)]
pub struct CommitSettings {
    pub message: String,
    pub author: String,
    pub author_email: String,
}

#[derive(Debug)]
pub struct Config {
    dry_run: bool,
    changelog_path: PathBuf,
    entry_pattern: EntryPattern,
    commit_settings: CommitSettings,
    version_header: VersionHeader,
    section_header: String,
}

impl Config {
    pub fn from_env_args() -> Result<Self> {
        let mut args = env::args().skip(1);

        let changelog_path = next_arg_trimmed(&mut args).ok_or("Missing changelog path")?;
        log::debug!("changelog_path={changelog_path}");

        let changelog_entry_pattern =
            next_arg_trimmed(&mut args).ok_or("Missing changelog path")?;
        log::debug!("changelog_entry_pattern={changelog_entry_pattern}");

        let commit_message = next_arg_trimmed(&mut args).ok_or("Missing commit message")?;
        log::debug!("commit_message={commit_message}");

        let committer_name = next_arg_trimmed(&mut args).ok_or("Missing committer name")?;
        log::debug!("committer_name={committer_name}");

        let committer_email = next_arg_trimmed(&mut args).ok_or("Missing committer email")?;
        log::debug!("committer_email={committer_email}");

        let version_header = next_arg_trimmed(&mut args).ok_or("Missing section header")?;
        log::debug!("version_header={version_header}");

        let section_header = next_arg_trimmed(&mut args).ok_or("Missing section header")?;
        log::debug!("section_header={section_header}");

        let push_changes = next_arg_trimmed(&mut args).ok_or("Missing push_changes setting")?;
        log::debug!("push_changes={push_changes}");

        let dry_run = !push_changes.eq_ignore_ascii_case("true");

        if args.next().is_some() {
            return Err("Too many arguments provided".into());
        }

        if changelog_path.is_empty() {
            return Err("No changelog path specified".into());
        }

        let changelog_path = PathBuf::from(changelog_path);
        if !changelog_path.is_file() {
            return Err("The specified changelog could not be found".into());
        }

        let entry_pattern = EntryPattern::new(&changelog_entry_pattern)?;

        Ok(Self::new(
            dry_run,
            changelog_path,
            entry_pattern,
            CommitSettings {
                message: commit_message,
                author: committer_name,
                author_email: committer_email,
            },
            VersionHeader::new(version_header),
            section_header,
        ))
    }

    pub const fn new(
        dry_run: bool,
        changelog_path: PathBuf,
        entry_pattern: EntryPattern,
        commit_settings: CommitSettings,
        version_header: VersionHeader,
        section_header: String,
    ) -> Self {
        Self {
            dry_run,
            changelog_path,
            entry_pattern,
            commit_settings,
            version_header,
            section_header,
        }
    }

    pub fn commit_signature(&self) -> std::result::Result<Signature, git2::Error> {
        Signature::now(
            &self.commit_settings.author,
            &self.commit_settings.author_email,
        )
    }

    pub fn commit_message(&self) -> &str {
        &self.commit_settings.message
    }

    pub fn version_header(&self) -> &VersionHeader {
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

    pub fn entry_pattern(&self) -> &EntryPattern {
        &self.entry_pattern
    }
}
