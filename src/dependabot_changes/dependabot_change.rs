use super::old_version::OldVersion;

#[derive(Debug, Clone)]
pub struct DependabotChange<'s> {
    pub name: &'s str,
    old_version: OldVersion<'s>,
    pub new_version: &'s str,
    pub url: Option<&'s str>,
}

impl<'s> PartialEq for DependabotChange<'s> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.old_version == other.old_version
            && self.new_version == other.new_version
    }
}

impl<'s> DependabotChange<'s> {
    pub const fn new(name: &'s str, old_version: &'s str, new_version: &'s str) -> Self {
        Self {
            name,
            old_version: OldVersion::FromDependabot(old_version),
            new_version,
            url: None,
        }
    }

    #[allow(dead_code)]
    pub const fn with_url(mut self, url: &'s str) -> Self {
        self.url = Some(url);
        self
    }

    pub fn set_url(&mut self, url: &'s str) {
        self.url = Some(url);
    }

    pub fn dep_name(&self) -> &'s str {
        if self.name.starts_with('[') {
            if let Some(end) = self.name.find(']') {
                return &self.name[1..end];
            }
        }
        self.name
    }

    pub fn dep_url(&self) -> Option<&'s str> {
        if let Some(url) = self.url {
            return Some(url);
        }
        if self.name.starts_with('[') {
            if let Some(open_paren) = self.name.find("](") {
                if let Some(end_paren) = self.name[open_paren + 2..].find(')') {
                    return Some(&self.name[open_paren + 2..open_paren + 2 + end_paren]);
                }
            }
        }
        None
    }

    /// The combined length of the name + old_version + new_version strings
    ///
    /// Add the length of the template string without tokens to get the
    /// total formatted length.
    pub fn total_len(&self) -> usize {
        self.name.len()
            + self.old_version.len()
            + self.new_version.len()
            + self.dep_url().map_or(0, |u| u.len())
    }

    /// Attempts to parse a DependabotChange from a string.
    ///
    /// # Assumptions
    ///
    /// ## #1 The input string does not include the 'update' keyword
    ///
    /// e.g. a line such as:
    ///
    /// `Updates foo from 1.0 to 2.0`
    ///
    /// should be passed as a slice containing
    ///
    /// `foo from 1.0 to 2.0`
    ///
    /// ## #2 The input string follows the form `[dep] from [ver] to [ver]`
    ///
    /// Valid:
    ///
    /// ```md
    /// [project](link) from `b0c35f6` to `c8bd600`
    /// ```
    ///
    /// NOT valid:
    ///
    /// ```md
    /// bar is updated to `c8bd600` from `b0c35f6`
    /// ```
    pub fn from_str(haystack: &'s str) -> Option<Self> {
        let (name, rest) = haystack.split_once("from")?;
        let (old_version, new_version) = rest.split_once("to")?;

        let trimmed_name = name.trim().trim_start_matches('`').trim_end_matches('`');
        let mut change = DependabotChange::new(
            trimmed_name,
            old_version.trim(),
            new_version.trim().trim_end_matches('.'),
        );
        if trimmed_name.starts_with('[') {
            if let Some(open_paren) = trimmed_name.find("](") {
                if let Some(end_paren) = trimmed_name[open_paren + 2..].find(')') {
                    change.url = Some(&trimmed_name[open_paren + 2..open_paren + 2 + end_paren]);
                }
            }
        }
        Some(change)
    }

    pub fn replace_old_version(&mut self, old_version: String) {
        self.old_version = OldVersion::FromChangelog(old_version)
    }

    pub fn old_version(&self) -> &str {
        match self.old_version {
            OldVersion::FromDependabot(s) => s,
            OldVersion::FromChangelog(ref s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependabot_change_from_str_basic() {
        let teststr = "Bumps ubi9/ubi from 9.4-1214.1726694543 to 9.4-1214.1729773476.";
        let dep_change = DependabotChange::from_str(&teststr[5..]);

        assert!(dep_change.is_some());
        assert_eq!(
            dep_change.unwrap(),
            DependabotChange::new("ubi9/ubi", "9.4-1214.1726694543", "9.4-1214.1729773476")
        );
    }

    #[test]
    fn test_from_str_basic_version_change() {
        let input = "foo-package from 1.0.0 to 2.0.0";
        let change = DependabotChange::from_str(input).unwrap();
        assert_eq!(change.name, "foo-package");
        assert_eq!(change.old_version(), "1.0.0");
        assert_eq!(change.new_version, "2.0.0");
    }

    #[test]
    fn test_from_str_git_commit_hashes() {
        let input = "my-project from b0c35f6 to c8bd600";
        let change = DependabotChange::from_str(input).unwrap();
        assert_eq!(change.old_version(), "b0c35f6");
        assert_eq!(change.new_version, "c8bd600");
    }

    #[test]
    fn test_from_str_markdown_links() {
        let input = "[my-project](https://github.com/user/repo) from `1.0.0` to `2.0.0`";
        let change = DependabotChange::from_str(input).unwrap();
        assert_eq!(change.name, "[my-project](https://github.com/user/repo)");
        assert_eq!(change.old_version(), "`1.0.0`");
        assert_eq!(change.new_version, "`2.0.0`");
    }

    #[test]
    fn test_from_str_trailing_period() {
        let input = "package from 1.0 to 2.0.";
        let change = DependabotChange::from_str(input).unwrap();
        assert_eq!(change.new_version, "2.0");
    }

    #[test]
    fn test_from_str_invalid_formats() {
        assert!(DependabotChange::from_str("").is_none());
        assert!(DependabotChange::from_str("package to 2.0").is_none());
        assert!(DependabotChange::from_str("package from 1.0").is_none());
    }

    #[test]
    fn test_from_str_extra_whitespace() {
        let input = "  package   from   1.0.0   to   2.0.0  ";
        let change = DependabotChange::from_str(input).unwrap();
        assert_eq!(change.name, "package");
        assert_eq!(change.old_version(), "1.0.0");
        assert_eq!(change.new_version, "2.0.0");
    }

    #[test]
    fn test_dependabot_change_from_str_sanitize_backticks() {
        let teststr = "Bumps `ubi9/ubi` from 9.4-1214.1726694543 to 9.4-1214.1729773476.";
        let dep_change = DependabotChange::from_str(&teststr[5..]);

        assert!(dep_change.is_some());
        assert_eq!(
            dep_change.unwrap(),
            DependabotChange::new("ubi9/ubi", "9.4-1214.1726694543", "9.4-1214.1729773476")
        );
    }
}
