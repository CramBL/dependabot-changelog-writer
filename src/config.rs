use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::{env, io};

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
pub struct Config {
    dry_run: bool,
    changelog_path: PathBuf,
    entry_pattern: EntryPattern,
    version_header: VersionHeader,
    section_header: String,
}

impl Config {
    pub fn from_env_args() -> Result<Self> {
        let mut args = env::args().skip(1);

        let changelog_path = next_arg_trimmed(&mut args).ok_or("Missing changelog path")?;
        log::debug!("changelog-path={changelog_path}");

        let changelog_entry_pattern =
            next_arg_trimmed(&mut args).ok_or("Missing changelog-entry-pattern")?;
        log::debug!("changelog-entry-pattern={changelog_entry_pattern}");

        let commit_message = next_arg_trimmed(&mut args).ok_or("Missing commit-message")?;
        log::debug!("commit-message={commit_message}");

        let version_header = next_arg_trimmed(&mut args).ok_or("Missing version-header")?;
        log::debug!("version-header={version_header}");

        let section_header = next_arg_trimmed(&mut args).ok_or("Missing section-header")?;
        log::debug!("section-header={section_header}");

        let dry_run = next_arg_trimmed(&mut args).is_some_and(|s| s == "dry-run");
        log::info!("dry-run={dry_run}");

        if section_header.starts_with("###") {
            log::error!("Invalid section header: Starts with '###' when an h3 header already implies a prefix of '###'");
            return Err(format!("Invalid section header '{section_header}', expected a section header such as 'Changes'.\n\
            NOTE: section header is assumed to be an h3 header, meaning it implies a prefix of '###' such as '### Changes'\n\
            HINT: Try removing the '###' prefix").into());
        }

        if args.next().is_some() {
            return Err("Too many arguments provided".into());
        }

        if changelog_path.is_empty() {
            return Err("No changelog path specified".into());
        }

        let changelog_path = PathBuf::from(changelog_path);
        if !changelog_path.is_file() {
            return Err(format!(
                "The specified changelog '{}' could not be found",
                changelog_path.display()
            )
            .into());
        }

        let entry_pattern = EntryPattern::new(&changelog_entry_pattern)?;

        Ok(Self::new(
            dry_run,
            changelog_path,
            entry_pattern,
            VersionHeader::new(version_header),
            section_header,
        ))
    }

    pub const fn new(
        dry_run: bool,
        changelog_path: PathBuf,
        entry_pattern: EntryPattern,
        version_header: VersionHeader,
        section_header: String,
    ) -> Self {
        Self {
            dry_run,
            changelog_path,
            entry_pattern,
            version_header,
            section_header,
        }
    }

    pub fn version_header(&self) -> &VersionHeader {
        &self.version_header
    }

    pub fn section_header(&self) -> &str {
        &self.section_header
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
