mod parse;

use crate::{
    config::VersionHeader,
    dependabot_changes::{
        dependabot_change::DependabotChange, entry_pattern::EntryPattern, format_changes,
    },
};

pub fn add_changes_to_changelog_contents(
    mut changes: Vec<DependabotChange>,
    changelog_content: &mut String,
    markdown_pull_request_link: &str,
    entry_pattern: &EntryPattern,
    version_header: &VersionHeader,
    section_header: &str,
) {
    let pr_link_len =
        markdown_pull_request_link.len() * entry_pattern.pull_request_link_token_occurrences();
    let changes_formatted_len = changes.iter().fold(0, |sum, c| {
        c.total_len() + entry_pattern.min_len() + pr_link_len + sum
    });

    let mut h3_header = format!("### {section_header}\n");
    // Reserve for the new changelog entry to avoid the worst case of allocating
    // the size of the existing content
    changelog_content.reserve(changes_formatted_len + h3_header.len() + 4); // +4 for the worst case of adding 4 newlines

    let change_log_str_capacity = changelog_content.capacity();

    let h2_insert_pos = parse::find_h2_insert_position(changelog_content, version_header)
        .expect("Could not find the specified version h2 header");

    log::debug!("h2_insert_pos={h2_insert_pos}");
    log::debug!("Remaining string: {}", &changelog_content[h2_insert_pos..]);

    if let Some((existing_h3_rel_start, existing_h3_rel_insert_pos)) =
        parse::find_existing_h3_insert_position(&changelog_content[h2_insert_pos..], section_header)
    {
        let abs_existing_h3_start = h2_insert_pos + existing_h3_rel_start;
        let abs_existing_h3_insert_pos = h2_insert_pos + existing_h3_rel_insert_pos;
        log::debug!("abs_existing_h3_start={abs_existing_h3_start}, abs_existing_h3_insert_pos={abs_existing_h3_insert_pos}");
        let existing_deps = parse::find_existing_dependency_lines_to_replace(
            &changelog_content[abs_existing_h3_start..abs_existing_h3_insert_pos],
            &mut changes,
        );

        let mut string_offset = 0;
        // Iterate in reverse to go towards to the start of the changelog string
        // that way the next content we might have to update doesn't change position
        // and we don't have to keep track of an intermediate offset
        for line in existing_deps.iter().rev() {
            let range_to_remove = line.range_offset(abs_existing_h3_start);
            log::debug!(
                "Removing previous dependency entry: {}",
                &changelog_content[range_to_remove.clone()]
            );
            debug_assert_eq!(
                changelog_content[range_to_remove.clone()]
                    .matches('\n')
                    .count(),
                1,
                "Removed previous dependency entry should contain exactly one newline"
            );
            changelog_content.replace_range(range_to_remove.clone(), "");
            string_offset += range_to_remove.len();
        }
        let changes_md = format_changes(changes, entry_pattern, markdown_pull_request_link);
        let mut changes_insert_pos = abs_existing_h3_insert_pos - string_offset;
        log::debug!("changes_insert_pos={changes_insert_pos}");
        let three_prev_chars = &changelog_content[changes_insert_pos - 3..changes_insert_pos];
        if three_prev_chars == "\n\n\n" {
            // Special handling for the case where the h3 section header already contains
            // dependency update entries and all those dependencies are also being bumped
            // by the newly created dependabot PR. This would result in an extra newline and the
            // start of the section, and one missing at the end. We fix it by shifting the insertion
            // one character back.
            changes_insert_pos -= 1;
        }
        log::info!(
            "Inserting changes immediately after the following content: {}",
            &changelog_content[..=changes_insert_pos]
        );
        changelog_content.insert_str(changes_insert_pos, &changes_md);
    } else {
        let changes_md = format_changes(changes, entry_pattern, markdown_pull_request_link);
        let new_h3_insert_pos =
            parse::find_new_h3_insert_position(&changelog_content[h2_insert_pos..]);
        let insert_pos = h2_insert_pos + new_h3_insert_pos;

        // Ensure that we are inserting after 2 newlines
        // by counting the the newlines in the previous two characters
        // and inserting newlines if needed.
        let prev_two_chars = &changelog_content[insert_pos - 2..insert_pos];
        let prev_two_newline_count = prev_two_chars
            .chars()
            .fold(0, |acc, c| acc + (c == '\n') as u8);
        for _ in 0..(2 - prev_two_newline_count) {
            h3_header.insert(0, '\n');
        }
        h3_header.push('\n');
        h3_header.push_str(&changes_md);
        h3_header.push('\n');
        changelog_content.insert_str(h2_insert_pos + new_h3_insert_pos, &h3_header);
    }

    debug_assert_eq!(
        change_log_str_capacity,
        changelog_content.capacity(),
        "Changelog capacity changed after reserving extra memory"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_add_changes_to_changelog_content_small_changelog() {
        let mut changelog_content = EXAMPLE_SMALL_CHANGELOG_CONTENTS.to_owned();
        let changes = EXAMPLE_CHANGES.to_vec();
        let entry_pattern = EntryPattern::default();
        let version_header = VersionHeader::new("Unreleased".into());
        let section_header = "Dependencies";
        let expect_final_changelog_contents = r#"# Changelog

## [Unreleased]

### Changed

- Some behaviour

### Dependencies

- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))
- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))
- `semver`: 1.0.23 → 1.0.24 ([#1](https://github.com/user/repo/pull/1))
- `env_logger`: 0.11.5 → 0.11.6 ([#1](https://github.com/user/repo/pull/1))
- `zip`: 2.2.1 → 2.2.2 ([#1](https://github.com/user/repo/pull/1))
- `wasm-bindgen-futures`: 0.4.47 → 0.4.49 ([#1](https://github.com/user/repo/pull/1))
- `web-sys`: 0.3.74 → 0.3.76 ([#1](https://github.com/user/repo/pull/1))
- `thiserror`: 2.0.4 → 2.0.9 ([#1](https://github.com/user/repo/pull/1))

## [0.1.0]

### Added

- Some features
"#;

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(&changelog_content, expect_final_changelog_contents);

        // Run again to ensure idempotency
        add_changes_to_changelog_contents(
            changes,
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(
            &changelog_content,
            expect_final_changelog_contents,
            "Not idempotent!"
        );
    }

    #[test]
    fn test_add_changes_to_changelog_content_small_changelog_with_dependencies_section() {
        let mut changelog_content = EXAMPLE_SMALL_CHANGELOG_WITH_DEPENDENCIES_CONTENTS.to_owned();
        let entry_pattern = EntryPattern::default();
        let changes = EXAMPLE_CHANGES.to_vec();
        let version_header = VersionHeader::new("Unreleased".into());
        let section_header = "Dependencies";
        let expect_final_changelog_contents = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dependencies

- Bump egui from 0.29.0 to 0.30.0
- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))
- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))
- `semver`: 1.0.23 → 1.0.24 ([#1](https://github.com/user/repo/pull/1))
- `env_logger`: 0.11.5 → 0.11.6 ([#1](https://github.com/user/repo/pull/1))
- `zip`: 2.2.1 → 2.2.2 ([#1](https://github.com/user/repo/pull/1))
- `wasm-bindgen-futures`: 0.4.47 → 0.4.49 ([#1](https://github.com/user/repo/pull/1))
- `web-sys`: 0.3.74 → 0.3.76 ([#1](https://github.com/user/repo/pull/1))
- `thiserror`: 2.0.4 → 2.0.9 ([#1](https://github.com/user/repo/pull/1))

### Changed

- Some behaviour

## [0.1.0]

### Added

- Some features
"#;

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );

        assert_str_eq!(&changelog_content, expect_final_changelog_contents);

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(
            &changelog_content,
            expect_final_changelog_contents,
            "Not idempotent!"
        );
    }

    #[test]
    fn test_add_changes_to_changelog_contents_empty_changelog() {
        let mut changelog_content = EXAMPLE_EMPTY_CHANGELOG_CONTENTS.to_owned();
        let entry_pattern = EntryPattern::default();
        let changes = EXAMPLE_CHANGES.to_vec();
        let version_header = VersionHeader::new("Unreleased".into());
        let section_header = "Dependencies";
        let expect_final_changelog_contents = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dependencies

- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))
- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))
- `semver`: 1.0.23 → 1.0.24 ([#1](https://github.com/user/repo/pull/1))
- `env_logger`: 0.11.5 → 0.11.6 ([#1](https://github.com/user/repo/pull/1))
- `zip`: 2.2.1 → 2.2.2 ([#1](https://github.com/user/repo/pull/1))
- `wasm-bindgen-futures`: 0.4.47 → 0.4.49 ([#1](https://github.com/user/repo/pull/1))
- `web-sys`: 0.3.74 → 0.3.76 ([#1](https://github.com/user/repo/pull/1))
- `thiserror`: 2.0.4 → 2.0.9 ([#1](https://github.com/user/repo/pull/1))

"#;

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(&changelog_content, expect_final_changelog_contents);

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(
            &changelog_content,
            expect_final_changelog_contents,
            "Not idempotent!"
        );
    }

    /// The section already contains the env_logger dependency so we expect that line to be
    /// removed and replaced by a new entry showing the highest/newest version it was bumped to
    /// but not replacing the old/former version from the previous entry as that would be misleading
    #[test]
    fn test_insert_changes_when_changes_section_exists() {
        let mut changelog_content = EXAMPLE_CHANGELOG_CONTENTS_CONTAINS_DEPENDENCIES.to_owned();
        let changes = EXAMPLE_CHANGES_SMALL.to_vec();
        let entry_pattern = EntryPattern::default();
        let version_header = VersionHeader::new("Unreleased".into());
        let section_header = "Dependencies";
        let expect_final_changelog_contents = r##"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Some feature

### Dependencies

- `chrono`: 0.4.38 → 0.4.39
- `semver`: 1.0.23 → 1.0.24
- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))
- `env_logger`: 0.11.5 → 0.12.0 ([#1](https://github.com/user/repo/pull/1))

### Fix

- Some issue
"##;

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(&changelog_content, expect_final_changelog_contents);

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(
            &changelog_content,
            expect_final_changelog_contents,
            "Not idempotent!"
        );
    }

    /// Check that we correctly update all entries when all the new version updates match earlier entries in the same section header
    #[test]
    fn test_insert_changes_when_changes_section_exists_replaces_all_entries() {
        let mut changelog_content = EXAMPLE_CHANGELOG_CONTENTS_CONTAINS_DEPENDENCIES.to_owned();
        let changes = vec![
            DependabotChange::new("`chrono`", "0.4.39", "0.4.41"),
            DependabotChange::new("`env_logger`", "0.11.5", "0.12.1"),
            DependabotChange::new("`semver`", "1.0.24", "1.0.25"),
        ];
        let entry_pattern = EntryPattern::default();
        let version_header = VersionHeader::new("Unreleased".into());
        let section_header = "Dependencies";
        let expect_final_changelog_contents = r##"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Some feature

### Dependencies

- `chrono`: 0.4.38 → 0.4.41 ([#1](https://github.com/user/repo/pull/1))
- `env_logger`: 0.11.5 → 0.12.1 ([#1](https://github.com/user/repo/pull/1))
- `semver`: 1.0.23 → 1.0.25 ([#1](https://github.com/user/repo/pull/1))

### Fix

- Some issue
"##;

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(&changelog_content, expect_final_changelog_contents);

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(
            &changelog_content,
            expect_final_changelog_contents,
            "Not idempotent!"
        );
    }

    #[test]
    fn test_insert_changes_in_previous_version_no_trailing_newline() {
        let mut changelog_content = EXAMPLE_SMALL_CHANGELOG_CONTENTS_NO_NEWLINE.to_owned();
        let entry_pattern = EntryPattern::default();
        let changes = EXAMPLE_CHANGES_SMALL_WITH_SHA1.to_vec();
        let version_header = VersionHeader::new("0.1.0".into());
        let section_header = "Dependencies";
        let expect_final_changelog_contents = r##"# Changelog

## [Unreleased]

## [0.1.0] - 2024-12-25

### Added

- Another changelog for testing alternate dependabot-changelog-writer scenarios

### Dependencies

- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))
- `docker/login-action`: 3d58c274f17dffee475a5520cbe67f0a882c4dbb → 7ca345011ac4304463197fac0e56eab1bc7e6af0 ([#1](https://github.com/user/repo/pull/1))

"##;

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(&changelog_content, expect_final_changelog_contents);

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(
            &changelog_content,
            expect_final_changelog_contents,
            "Not idempotent!"
        );
    }

    #[test]
    fn test_add_changes_to_changelog_contents_issues51() {
        let mut changelog_content = ISSUE_51_CHANGELOG.to_owned();
        let entry_pattern = EntryPattern::default();
        let changes = CHANGES_ISSUE_51.to_vec();
        let version_header = VersionHeader::Unreleased;
        let section_header = "Dependencies";
        let expect_final_changelog_contents = r##"# Changelog

## [unreleased]

### Dependencies

- `clap`: 4.5.26 → 4.5.28 ([#1](https://github.com/user/repo/pull/1))
- `clap_complete`: 4.5.42 → 4.5.44 ([#1](https://github.com/user/repo/pull/1))
- `mdns-sd`: 0.13.1 → 0.13.2 ([#1](https://github.com/user/repo/pull/1))
- `strum`: 0.26.3 → 0.27.0 ([#1](https://github.com/user/repo/pull/1))

## [0.4.2]

### Dependencies

- `mdns-sd`: 0.13.0 → 0.13.1
- `crambl/dependabot-changelog-writer`: 0.4.0 → 0.5.0
- `crate-ci/typos`: 1.28.3 → 1.29.4 ([#47](https://github.com/CramBL/fidelityfetch/pull/47))
- `axum`: 0.7.9 → 0.8.1 ([#48](https://github.com/CramBL/fidelityfetch/pull/48))
- `clap`: 4.5.23 → 4.5.26 ([#49](https://github.com/CramBL/fidelityfetch/pull/49))
- `clap_complete`: 4.5.38 → 4.5.42 ([#49](https://github.com/CramBL/fidelityfetch/pull/49))
- `tokio`: 1.42.0 → 1.43.0 ([#49](https://github.com/CramBL/fidelityfetch/pull/49))
- `thiserror`: 2.0.7 → 2.0.11 ([#49](https://github.com/CramBL/fidelityfetch/pull/49))

## [0.4.1]

### Changed

- Reduce response size by trimming white space
- Make the file list more compact

## [0.4.0]

### Added

- Allow logging to journald explicitly
- Allow setting logging destination to either `journald` (unix only), `stderr`, or `stdout`.

### Fix

- Main process now exits with a non-zero exit when failing to bind to a port.

### Misc.

- Update dependencies

## [0.3.3]

### Changed

- Update dependencies

### Internals

- Refactors
- More tests

## [0.3.2]

### Added

- npm publish job

## [0.3.1]

### Changed

- Update dependencies to get some critical fixes to some async dependencies.

## [0.3.0]

### Added

- Recognize and add icons to many more filetypes
- Better error messages

## [0.2.0]

### Added

- Completions for bash, zsh, fish, powershell, elvish.
- Favicon.ico
- Various tweaks

## [0.1.1]

### Fix

- Paths with spaces and non-English letters not being recognized
"##;

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(&changelog_content, expect_final_changelog_contents);

        add_changes_to_changelog_contents(
            changes.clone(),
            &mut changelog_content,
            EXAMPLE_MARKDOWN_PR_LINK,
            &entry_pattern,
            &version_header,
            section_header,
        );
        assert_str_eq!(&changelog_content, expect_final_changelog_contents);
    }
}
