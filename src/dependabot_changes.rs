use dependabot_change::DependabotChange;
use entry_pattern::EntryPattern;

pub mod dependabot_change;
pub mod entry_pattern;
pub mod old_version;

pub fn parse_body(body: &str) -> Vec<DependabotChange<'_>> {
    let changes = parse_changes(body);
    for change in &changes {
        log::debug!("{:?}", change);
    }
    changes
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

pub fn format_changes(changes: Vec<DependabotChange>, entry_pattern: &EntryPattern) -> String {
    let mut markdown = String::new();

    // Iterate over each change and format it into the markdown string
    for change in changes {
        // For each change, add a list item in markdown format
        let entry = entry_pattern.format(change.name, change.old_version(), change.new_version);
        markdown.push_str(&entry);
    }

    debug_assert!(!markdown.starts_with("\n\n"));
    debug_assert!(!markdown.ends_with("\n\n"));

    markdown
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_parse_body() {
        let changes = parse_body(DEPENDABOT_BODY_2_ACTIONS_SHA_SEMVER);
        let entry_pattern = &EntryPattern::default();
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
        let changes_md = format_changes(changes, entry_pattern);
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
        let entry_pattern = &EntryPattern::default();
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

        let changes_md = format_changes(changes, entry_pattern);
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
    fn test_parse_example_to_changes_7_crates_semver_custom_pattern() {
        let changes = EXAMPLE_CHANGES.to_vec();
        let entry_pattern = &EntryPattern::new("Bump [dep] from [old] to [new]").unwrap();

        let changes_md = format_changes(changes, entry_pattern);
        let expect_md = "\
        - Bump `serde` from 1.0.215 to 1.0.216\n\
        - Bump `chrono` from 0.4.38 to 0.4.39\n\
        - Bump `semver` from 1.0.23 to 1.0.24\n\
        - Bump `env_logger` from 0.11.5 to 0.11.6\n\
        - Bump `zip` from 2.2.1 to 2.2.2\n\
        - Bump `wasm-bindgen-futures` from 0.4.47 to 0.4.49\n\
        - Bump `web-sys` from 0.3.74 to 0.3.76\n\
        - Bump `thiserror` from 2.0.4 to 2.0.9\n";
        assert_str_eq!(changes_md, expect_md);
    }

    #[test]
    fn test_parse_example_to_changes_7_crates_semver_custom_pattern_emojies() {
        let changes = EXAMPLE_CHANGES.to_vec();
        let entry_pattern =
            &EntryPattern::new("📝 Update [dep] from 🩺[old]🩺 🚀 🍄[new]🍄").unwrap();

        let changes_md = format_changes(changes, entry_pattern);
        let expect_md = "\
        - 📝 Update `serde` from 🩺1.0.215🩺 🚀 🍄1.0.216🍄\n\
        - 📝 Update `chrono` from 🩺0.4.38🩺 🚀 🍄0.4.39🍄\n\
        - 📝 Update `semver` from 🩺1.0.23🩺 🚀 🍄1.0.24🍄\n\
        - 📝 Update `env_logger` from 🩺0.11.5🩺 🚀 🍄0.11.6🍄\n\
        - 📝 Update `zip` from 🩺2.2.1🩺 🚀 🍄2.2.2🍄\n\
        - 📝 Update `wasm-bindgen-futures` from 🩺0.4.47🩺 🚀 🍄0.4.49🍄\n\
        - 📝 Update `web-sys` from 🩺0.3.74🩺 🚀 🍄0.3.76🍄\n\
        - 📝 Update `thiserror` from 🩺2.0.4🩺 🚀 🍄2.0.9🍄\n";
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
