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

#[derive(Debug, Clone, Copy)]
pub enum DuplicateEntryStrategy {
    Append,
    Overwrite,
}

impl TryFrom<&str> for DuplicateEntryStrategy {
    type Error = Box<dyn Error>;

    fn try_from(s: &str) -> Result<Self> {
        match s {
            "overwrite" => Ok(Self::Overwrite),
            "append" => Ok(Self::Append),
            _ => Err(format!("Invalid value for 'duplicate-entry-pattern': {s}\nHINT: Valid values are: 'append' or 'overwrite'").into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SeparatorKind {
    Lf,
    DoubleLf,
    Crlf,
    DoubleCrlf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntrySeparator(SeparatorKind);

impl EntrySeparator {
    pub const LF: Self = Self(SeparatorKind::Lf);
    pub const DOUBLE_LF: Self = Self(SeparatorKind::DoubleLf);
    pub const CRLF: Self = Self(SeparatorKind::Crlf);
    pub const DOUBLE_CRLF: Self = Self(SeparatorKind::DoubleCrlf);

    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self.0 {
            SeparatorKind::Lf => "\n",
            SeparatorKind::DoubleLf => "\n\n",
            SeparatorKind::Crlf => "\r\n",
            SeparatorKind::DoubleCrlf => "\r\n\r\n",
        }
    }

    #[must_use]
    pub const fn is_blank_line(&self) -> bool {
        matches!(self.0, SeparatorKind::DoubleLf | SeparatorKind::DoubleCrlf)
    }

    #[must_use]
    pub const fn is_crlf(&self) -> bool {
        matches!(self.0, SeparatorKind::Crlf | SeparatorKind::DoubleCrlf)
    }

    #[must_use]
    pub const fn newline(&self) -> &'static str {
        if self.is_crlf() { "\r\n" } else { "\n" }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.as_str().len()
    }

    #[must_use]
    #[allow(dead_code)]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

impl Default for EntrySeparator {
    fn default() -> Self {
        Self::LF
    }
}

impl std::ops::Deref for EntrySeparator {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for EntrySeparator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for EntrySeparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&str> for EntrySeparator {
    type Error = Box<dyn Error>;

    fn try_from(s: &str) -> Result<Self> {
        let unescaped = s
            .replace("\\r\\n", "\r\n")
            .replace("\\n", "\n")
            .replace("\\r", "\r");
        match unescaped.as_str() {
            "\n" => Ok(Self::LF),
            "\n\n" => Ok(Self::DOUBLE_LF),
            "\r\n" => Ok(Self::CRLF),
            "\r\n\r\n" => Ok(Self::DOUBLE_CRLF),
            _ => Err(format!(
                "Invalid entry separator: {s:?}. Supported values are '\\n', '\\n\\n', '\\r\\n', or '\\r\\n\\r\\n'"
            )
            .into()),
        }
    }
}

impl std::str::FromStr for EntrySeparator {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self> {
        Self::try_from(s)
    }
}

#[expect(dead_code)]
pub const DEFAULT_ENTRY_SEPARATOR: &str = "\n";

#[expect(dead_code)]
pub fn parse_entry_separator(s: &str) -> Result<EntrySeparator> {
    EntrySeparator::try_from(s)
}

#[derive(Debug)]
pub struct Config {
    dry_run: bool,
    changelog_path: PathBuf,
    entry_pattern: EntryPattern,
    entry_separator: EntrySeparator,
    duplicate_entry_strategy: DuplicateEntryStrategy,
    version_header: VersionHeader,
    section_header: String,
}

impl Config {
    pub fn from_env_args() -> Result<Self> {
        Self::from_args(env::args().skip(1))
    }

    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Self> {
        let changelog_path = next_arg_trimmed(&mut args).ok_or("Missing changelog path")?;
        log::debug!("changelog-path={changelog_path}");

        let changelog_entry_pattern =
            next_arg_trimmed(&mut args).ok_or("Missing changelog-entry-pattern")?;
        log::debug!("changelog-entry-pattern={changelog_entry_pattern}");

        let duplicate_entry_strategy =
            next_arg_trimmed(&mut args).ok_or("Missing duplicate-entry-strategy")?;
        log::debug!("duplicate-entry-strategy={duplicate_entry_strategy}");
        let duplicate_entry_strategy =
            DuplicateEntryStrategy::try_from(duplicate_entry_strategy.as_str())?;

        let commit_message = next_arg_trimmed(&mut args).ok_or("Missing commit-message")?;
        log::debug!("commit-message={commit_message}");

        let version_header = next_arg_trimmed(&mut args).ok_or("Missing version-header")?;
        log::debug!("version-header={version_header}");

        let section_header = next_arg_trimmed(&mut args).ok_or("Missing section-header")?;
        log::debug!("section-header={section_header}");

        if section_header.starts_with("###") {
            log::error!(
                "Invalid section header: Starts with '###' when an h3 header already implies a prefix of '###'"
            );
            return Err(format!("Invalid section header '{section_header}', expected a section header such as 'Changes'.\n\
            NOTE: section header is assumed to be an h3 header, meaning it implies a prefix of '###' such as '### Changes'\n\
            HINT: Try removing the '###' prefix").into());
        }

        let next = args.next().map(|s| s.trim_matches([' ', '\t']).to_owned());
        let (entry_separator, dry_run) = match next {
            Some(arg) if arg == "dry-run" => (EntrySeparator::default(), true),
            Some(arg) => {
                let sep = EntrySeparator::try_from(arg.as_str())?;
                let dry_run = next_arg_trimmed(&mut args).is_some_and(|s| s == "dry-run");
                (sep, dry_run)
            }
            None => (EntrySeparator::default(), false),
        };

        log::debug!("entry-separator={entry_separator:?}");
        log::info!("dry-run={dry_run}");

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
            entry_separator,
            duplicate_entry_strategy,
            VersionHeader::new(version_header),
            section_header,
        ))
    }

    pub const fn new(
        dry_run: bool,
        changelog_path: PathBuf,
        entry_pattern: EntryPattern,
        entry_separator: EntrySeparator,
        duplicate_entry_strategy: DuplicateEntryStrategy,
        version_header: VersionHeader,
        section_header: String,
    ) -> Self {
        Self {
            dry_run,
            changelog_path,
            entry_pattern,
            entry_separator,
            duplicate_entry_strategy,
            version_header,
            section_header,
        }
    }

    pub const fn entry_separator(&self) -> EntrySeparator {
        self.entry_separator
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

    pub fn duplicate_entry_strategy(&self) -> DuplicateEntryStrategy {
        self.duplicate_entry_strategy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_separator_values() {
        assert_eq!(EntrySeparator::try_from("\n").unwrap(), EntrySeparator::LF);
        assert_eq!(EntrySeparator::try_from("\\n").unwrap(), EntrySeparator::LF);
        assert_eq!(
            EntrySeparator::try_from("\n\n").unwrap(),
            EntrySeparator::DOUBLE_LF
        );
        assert_eq!(
            EntrySeparator::try_from("\\n\\n").unwrap(),
            EntrySeparator::DOUBLE_LF
        );
        assert_eq!(
            EntrySeparator::try_from("\r\n").unwrap(),
            EntrySeparator::CRLF
        );
        assert_eq!(
            EntrySeparator::try_from("\\r\\n").unwrap(),
            EntrySeparator::CRLF
        );
        assert_eq!(
            EntrySeparator::try_from("\r\n\r\n").unwrap(),
            EntrySeparator::DOUBLE_CRLF
        );
        assert_eq!(
            EntrySeparator::try_from("\\r\\n\\r\\n").unwrap(),
            EntrySeparator::DOUBLE_CRLF
        );
        assert!(EntrySeparator::try_from("").is_err());
        assert!(EntrySeparator::try_from("foo").is_err());
        assert!(EntrySeparator::try_from("\n\n\n").is_err());
    }

    #[test]
    fn test_entry_separator_methods() {
        let lf = EntrySeparator::LF;
        assert_eq!(lf.as_str(), "\n");
        assert_eq!(lf.newline(), "\n");
        assert_eq!(lf.len(), 1);
        assert!(!lf.is_empty());
        assert!(!lf.is_blank_line());
        assert!(!lf.is_crlf());
        assert_eq!(lf.to_string(), "\n");
        assert_eq!(&*lf, "\n");
        assert_eq!(lf.as_ref(), "\n");
        assert_eq!(EntrySeparator::default(), EntrySeparator::LF);

        let double_lf = EntrySeparator::DOUBLE_LF;
        assert_eq!(double_lf.as_str(), "\n\n");
        assert_eq!(double_lf.newline(), "\n");
        assert!(double_lf.is_blank_line());
        assert!(!double_lf.is_crlf());

        let crlf = EntrySeparator::CRLF;
        assert_eq!(crlf.as_str(), "\r\n");
        assert_eq!(crlf.newline(), "\r\n");
        assert!(!crlf.is_blank_line());
        assert!(crlf.is_crlf());

        let double_crlf = EntrySeparator::DOUBLE_CRLF;
        assert_eq!(double_crlf.as_str(), "\r\n\r\n");
        assert_eq!(double_crlf.newline(), "\r\n");
        assert!(double_crlf.is_blank_line());
        assert!(double_crlf.is_crlf());
    }

    #[test]
    fn test_config_from_args_branches() {
        let base_args = vec![
            "CHANGELOG.md".to_owned(),
            "`[dep]`: [old] → [new]".to_owned(),
            "overwrite".to_owned(),
            "commit msg".to_owned(),
            "Unreleased".to_owned(),
            "Dependencies".to_owned(),
        ];

        // Default with no extra args
        let config = Config::from_args(base_args.clone().into_iter()).unwrap();
        assert_eq!(config.entry_separator(), EntrySeparator::default());
        assert!(!config.dry_run());

        // Dry-run alone (7 args)
        let mut args = base_args.clone();
        args.push("dry-run".to_owned());
        let config = Config::from_args(args.into_iter()).unwrap();
        assert_eq!(config.entry_separator(), EntrySeparator::default());
        assert!(config.dry_run());

        // Separator specified, no dry-run (7 args)
        let mut args = base_args.clone();
        args.push("\\n\\n".to_owned());
        let config = Config::from_args(args.into_iter()).unwrap();
        assert_eq!(config.entry_separator(), EntrySeparator::DOUBLE_LF);
        assert!(!config.dry_run());

        // Separator specified with dry-run (8 args)
        let mut args = base_args.clone();
        args.push("\\n".to_owned());
        args.push("dry-run".to_owned());
        let config = Config::from_args(args.into_iter()).unwrap();
        assert_eq!(config.entry_separator(), EntrySeparator::LF);
        assert!(config.dry_run());

        // Separator specified with 'true' (CI case, 8 args)
        let mut args = base_args.clone();
        args.push("\\n".to_owned());
        args.push("true".to_owned());
        let config = Config::from_args(args.into_iter()).unwrap();
        assert_eq!(config.entry_separator(), EntrySeparator::LF);
        assert!(!config.dry_run());

        // Invalid separator produces error
        let mut args = base_args.clone();
        args.push("invalid".to_owned());
        assert!(Config::from_args(args.into_iter()).is_err());

        // Too many arguments
        let mut args = base_args;
        args.push("\\n".to_owned());
        args.push("dry-run".to_owned());
        args.push("unexpected".to_owned());
        assert!(Config::from_args(args.into_iter()).is_err());
    }
}
