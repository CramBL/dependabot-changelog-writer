#[derive(Debug, Clone, PartialEq)]
enum OldVersion<'s> {
    FromDependabot(&'s str),
    FromChangelog(String),
}

impl<'s> OldVersion<'s> {
    pub fn len(&self) -> usize {
        match self {
            // Comes from parsing the Dependabot PR body
            OldVersion::FromDependabot(s) => s.len(),
            // Comes from the Changelog (the section already mentions upgrading this version)
            // and replaces the version that comes from the Dependabot PR body
            OldVersion::FromChangelog(s) => s.len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DependabotChange<'s> {
    pub name: &'s str,
    old_version: OldVersion<'s>,
    pub new_version: &'s str,
}

impl<'s> DependabotChange<'s> {
    const PREFIX: &'static str = "- ";
    const NAME_OLD_VER_SEPARATOR: &'static str = ": ";
    const OLD_VER_NEW_VER_SEPARATOR: &'static str = " → ";

    pub const fn new(name: &'s str, old_version: &'s str, new_version: &'s str) -> Self {
        Self {
            name,
            old_version: OldVersion::FromDependabot(old_version),
            new_version,
        }
    }

    pub fn formatted_len(&self) -> usize {
        Self::PREFIX.len()
            + self.name.len()
            + Self::NAME_OLD_VER_SEPARATOR.len()
            + self.old_version.len()
            + Self::OLD_VER_NEW_VER_SEPARATOR.len()
            + self.new_version.len()
            + "\n".len()
    }

    pub fn from_str(haystack: &'s str) -> Option<Self> {
        // Try to extract the dependency name, old version, and new version
        if let Some(from_pos) = haystack.find("from") {
            let to_pos = haystack.find("to").unwrap_or(haystack.len());

            // Extract the dependency name (before "from")
            let name = &haystack[..from_pos].trim();

            // Extract the old version (between "from" and "to")
            let old_version = &haystack[from_pos + 4..to_pos].trim();

            // Extract the new version (after "to")
            let new_version = &haystack[to_pos + 2..].trim().trim_end_matches(".");

            Some(DependabotChange::new(name, old_version, new_version))
        } else {
            None
        }
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

impl std::fmt::Display for DependabotChange<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{pre}{name}{sep1}{old_ver}{sep2}{new_ver}",
            pre = Self::PREFIX,
            sep1 = Self::NAME_OLD_VER_SEPARATOR,
            sep2 = Self::OLD_VER_NEW_VER_SEPARATOR,
            name = self.name,
            old_ver = self.old_version(),
            new_ver = self.new_version
        )
    }
}

pub fn parse_body(body: &str) -> Vec<DependabotChange<'_>> {
    let changes = parse_changes(body);
    for change in &changes {
        log::debug!("{:?}", change);
    }
    changes
}

pub fn format_changes(changes: Vec<DependabotChange>) -> String {
    let mut markdown = String::new();

    // Iterate over each change and format it into the markdown string
    for change in changes {
        // For each change, add a list item in markdown format
        markdown.push_str(&change.to_string());
    }

    debug_assert!(!markdown.starts_with("\n\n"));
    debug_assert!(!markdown.ends_with("\n\n"));

    markdown
}

const UPDATE_LINE_KEYWORD_A: &str = "Updates";
const UPDATE_LINE_KEYWORD_B: &str = "Bumps";

const UPDATE_KEYWORDS: [(&str, usize); 2] = [
    (UPDATE_LINE_KEYWORD_A, UPDATE_LINE_KEYWORD_A.len()),
    (UPDATE_LINE_KEYWORD_B, UPDATE_LINE_KEYWORD_B.len()),
];

fn parse_changes(body: &str) -> Vec<DependabotChange> {
    let mut changes = Vec::new();

    let mut skip_parsing = false;
    for line in body.lines() {
        // Skip content in a <details> section
        if skip_parsing {
            if line.starts_with("</details") {
                skip_parsing = false;
            }
        } else if line.starts_with("<details") {
            skip_parsing = true;
        } else {
            for &(keyword, keyword_len) in &UPDATE_KEYWORDS {
                if let Some(start) = line.find(keyword) {
                    let remaining = &line[start + keyword_len + 1..]; // +1 for whitespace
                    if let Some(dependabot_change) = DependabotChange::from_str(remaining) {
                        changes.push(dependabot_change);
                        break;
                    }
                }
            }
        }
    }
    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::example_dependabot_pr_bodies::*;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_parse_body() {
        let changes = parse_body(DEPENDABOT_BODY_2_ACTIONS_SHA_SEMVER);
        assert_eq!(changes.len(), 2);
        assert_eq!(
            changes[0],
            DependabotChange::new("`crate-ci/typos`", "1.27.0", "1.28.4")
        );
        assert_eq!(
            changes[1],
            DependabotChange::new(
                "`docker/login-action`",
                "3d58c274f17dffee475a5520cbe67f0a882c4dbb",
                "7ca345011ac4304463197fac0e56eab1bc7e6af0"
            )
        );
        let changes_md = format_changes(changes);
        let expect_md = "\
        - `crate-ci/typos`: 1.27.0 → 1.28.4\n\
        - `docker/login-action`: 3d58c274f17dffee475a5520cbe67f0a882c4dbb → 7ca345011ac4304463197fac0e56eab1bc7e6af0\n";
        assert_str_eq!(changes_md, expect_md);
    }

    #[test]
    fn test_parse_example_to_changes() {
        let changes = parse_changes(DEPENDABOT_BODY_2_ACTIONS_SHA_SEMVER);
        assert_eq!(changes.len(), 2);
    }

    #[test]
    fn test_parse_example_to_changes_7_crates_semver() {
        let changes = parse_changes(DEPENDABOT_BODY_7_CRATES_SEMVER);
        assert_eq!(changes.len(), 8);
        assert_eq!(
            changes[0],
            DependabotChange::new("`serde`", "1.0.215", "1.0.216")
        );
        assert_eq!(
            changes[1],
            DependabotChange::new("`chrono`", "0.4.38", "0.4.39")
        );
        assert_eq!(
            changes[2],
            DependabotChange::new("`semver`", "1.0.23", "1.0.24")
        );
        assert_eq!(
            changes[3],
            DependabotChange::new("`env_logger`", "0.11.5", "0.11.6")
        );
        assert_eq!(changes[4], DependabotChange::new("`zip`", "2.2.1", "2.2.2"));
        assert_eq!(
            changes[5],
            DependabotChange::new("`wasm-bindgen-futures`", "0.4.47", "0.4.49")
        );
        assert_eq!(
            changes[6],
            DependabotChange::new("`web-sys`", "0.3.74", "0.3.76")
        );
        assert_eq!(
            changes[7],
            DependabotChange::new("`thiserror`", "2.0.4", "2.0.9")
        );

        let changes_md = format_changes(changes);
        let expect_md = "\
        - `serde`: 1.0.215 → 1.0.216\n\
        - `chrono`: 0.4.38 → 0.4.39\n\
        - `semver`: 1.0.23 → 1.0.24\n\
        - `env_logger`: 0.11.5 → 0.11.6\n\
        - `zip`: 2.2.1 → 2.2.2\n\
        - `wasm-bindgen-futures`: 0.4.47 → 0.4.49\n\
        - `web-sys`: 0.3.74 → 0.3.76\n\
        - `thiserror`: 2.0.4 → 2.0.9\n";
        assert_str_eq!(changes_md, expect_md);
    }

    #[test]
    fn test_parse_body_1_submodule_short_sha() {
        let changes = parse_changes(DEPENDABOT_BODY_1_SUBMODULE_SHORT_SHA);
        assert_eq!(changes.len(), 1);
        assert_eq!(
            changes[0],
            DependabotChange::new(
                "[some-submodule](https://github.com/updates-org/some-submodule)",
                "`b0c35f6`",
                "`c8bd600`"
            )
        );
    }

    #[test]
    fn test_parse_body_1_docker_novel_version() {
        let changes = parse_changes(DEPENDABOT_BODY_1_DOCKER_NOVEL_VERSION);
        assert_eq!(changes.len(), 1);
        assert_eq!(
            changes[0],
            DependabotChange::new("ubi9/ubi", "9.4-1214.1726694543", "9.4-1214.1729773476")
        );
    }

    #[test]
    fn test_parse_body_skips_details_section() {
        let pr_body = r"Bumps foo from 0.1.0a to 0.1.1b
<details>
Bumps bar from 0.1.0 to 0.2.0
</details>
Updates baz from 2024.1.2 to 2025.1.2-rc1
";
        let changes = parse_body(pr_body);
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0], DependabotChange::new("foo", "0.1.0a", "0.1.1b"));
        assert_eq!(
            changes[1],
            DependabotChange::new("baz", "2024.1.2", "2025.1.2-rc1")
        );
    }
}
