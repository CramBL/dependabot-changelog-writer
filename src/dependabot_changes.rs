#[cfg(test)]
mod dependabot_example_bodies;

#[derive(Debug, Clone)]
pub struct DependabotChange<'s> {
    pub name: &'s str,
    pub old_version: &'s str,
    pub new_version: &'s str,
}

impl<'s> DependabotChange<'s> {
    const PREFIX: &'static str = "- ";
    const NAME_OLD_VER_SEPARATOR: &'static str = ": ";
    const OLD_VER_NEW_VER_SEPARATOR: &'static str = " → ";

    pub const fn new(name: &'s str, old_version: &'s str, new_version: &'s str) -> Self {
        Self {
            name,
            old_version,
            new_version,
        }
    }

    pub const fn formatted_len(&self) -> usize {
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
            let name = &haystack[from_version_character_pos()..from_pos].trim();

            // Extract the old version (between "from" and "to")
            let old_version = &haystack[from_pos + 4..to_pos].trim();

            // Extract the new version (after "to")
            let new_version = &haystack[to_pos + 2..].trim();

            Some(DependabotChange::new(name, old_version, new_version))
        } else {
            None
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
            old_ver = self.old_version,
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
    if changes.is_empty() {
        return String::new();
    }

    // Start with an empty string to accumulate the formatted result
    let mut markdown = String::new();

    // Iterate over each change and format it into the markdown string
    for change in changes {
        // For each change, add a list item in markdown format
        markdown.push_str(&change.to_string());
    }

    // Return the final markdown string
    markdown
}

const UPDATE_LINE_KEYWORD: &str = "Updates";

/// Update keyword + 1 for the whitespace separator
const fn from_version_character_pos() -> usize {
    UPDATE_LINE_KEYWORD.len() + 1
}

fn parse_changes(body: &str) -> Vec<DependabotChange> {
    let mut changes = Vec::new();

    // Split the body into lines
    let lines = body.lines();

    // Loop through the lines
    for line in lines {
        // Look for lines that match the pattern of a version change
        if let Some(start) = line.find(UPDATE_LINE_KEYWORD) {
            let remaining = &line[start..];

            if let Some(dependabot_change) = DependabotChange::from_str(remaining) {
                changes.push(dependabot_change);
            }
        }
    }

    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use dependabot_example_bodies::*;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_parse_body() {
        let changes_md = format_changes(parse_body(EXAMPLE_DEPENDABOT_BODY_SETTINGS_MANAGER));
        let expect_md = "\
        - `crate-ci/typos`: 1.27.0 → 1.28.4\n\
        - `docker/login-action`: 3d58c274f17dffee475a5520cbe67f0a882c4dbb → 7ca345011ac4304463197fac0e56eab1bc7e6af0\n";
        assert_str_eq!(changes_md, expect_md);
    }

    #[test]
    fn test_parse_example_to_changes() {
        let changes = parse_changes(EXAMPLE_DEPENDABOT_BODY_SETTINGS_MANAGER);
        assert_eq!(changes.len(), 2);
    }

    #[test]
    fn test_parse_example_to_changes_plotinator() {
        let changes = parse_changes(EXAMPLE_DEPENDABOT_BODY_PLOTINATOR);
        assert_eq!(changes.len(), 8);

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
}
