mod parse;

use crate::dependabot_changes::{format_changes, DependabotChange};

pub fn add_changes_to_changelog_contents(
    mut changes: Vec<DependabotChange>,
    changelog_content: &mut String,
    version_header: &str,
    section_header: &str,
) {
    let changes_formatted_len = changes.iter().fold(0, |sum, c| c.formatted_len() + sum);

    let mut h3_header = format!("### {section_header}\n");
    // Reserve for the new changelog entry to avoid the worst case of allocating
    // the size of the existing content
    changelog_content.reserve(changes_formatted_len + h3_header.len() + 3); // +3 for the worst case of adding 3 newlines

    let change_log_str_capacity = changelog_content.capacity();

    let h2_insert_pos = parse::find_h2_insert_position(changelog_content, version_header)
        .expect("Could not find the specified version h2 header");

    if let Some((existing_h3_start, existing_h3_insert_pos)) =
        parse::find_existing_h3_insert_position(&changelog_content[h2_insert_pos..], section_header)
    {
        let existing_deps = parse::find_existing_dependency_lines_to_replace(
            &changelog_content[existing_h3_start..],
            &mut changes,
        );

        let mut string_offset = 0;
        for line in existing_deps.iter().rev() {
            eprintln!(
                "Removing: '{}'",
                &changelog_content[line.range_offset(existing_h3_start)]
            );
            changelog_content.replace_range(line.range_offset(existing_h3_start), "");
            string_offset += line.range().len();
        }

        let changes_md = format_changes(changes);
        debug_assert!(!changes_md.starts_with("\n\n"));
        debug_assert!(!changes_md.ends_with("\n\n"));
        changelog_content.insert_str(
            h2_insert_pos + existing_h3_insert_pos - string_offset,
            &changes_md,
        );
    } else {
        let changes_md = format_changes(changes);
        debug_assert!(!changes_md.starts_with("\n\n"));
        debug_assert!(!changes_md.ends_with("\n\n"));
        let new_h3_insert_pos =
            parse::find_new_h3_insert_position(&changelog_content[h2_insert_pos..]);
        let insert_pos = h2_insert_pos + new_h3_insert_pos;

        // Insert a leading newline if we are not inserting the header just after two newlines
        let prev_two_chars = &changelog_content[insert_pos - 2..insert_pos];
        if prev_two_chars != "\n\n" {
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
    use pretty_assertions::assert_str_eq;

    use crate::test_util::*;

    #[test]
    fn test_add_changes_to_changelog_content_small_changelog() {
        let mut changelog_content = EXAMPLE_SMALL_CHANGELOG_CONTENTS.to_owned();

        let expect_final_changelog_contents = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Some behaviour

### Dependencies

- `serde`: 1.0.215 → 1.0.216
- `chrono`: 0.4.38 → 0.4.39
- `semver`: 1.0.23 → 1.0.24
- `env_logger`: 0.11.5 → 0.11.6
- `zip`: 2.2.1 → 2.2.2
- `wasm-bindgen-futures`: 0.4.47 → 0.4.49
- `web-sys`: 0.3.74 → 0.3.76
- `thiserror`: 2.0.4 → 2.0.9

## [0.1.0]

### Added

- Some features
"#;

        add_changes_to_changelog_contents(
            EXAMPLE_CHANGES.to_vec(),
            &mut changelog_content,
            "Unreleased",
            "Dependencies",
        );

        assert_str_eq!(&changelog_content, expect_final_changelog_contents);
    }

    #[test]
    fn test_add_changes_to_changelog_content_small_changelog_with_dependencies_section() {
        let mut changelog_content = EXAMPLE_SMALL_CHANGELOG_WITH_DEPENDENCIES_CONTENTS.to_owned();

        let expect_final_changelog_contents = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dependencies

- Bump egui from 0.29.0 to 0.30.0
- `serde`: 1.0.215 → 1.0.216
- `chrono`: 0.4.38 → 0.4.39
- `semver`: 1.0.23 → 1.0.24
- `env_logger`: 0.11.5 → 0.11.6
- `zip`: 2.2.1 → 2.2.2
- `wasm-bindgen-futures`: 0.4.47 → 0.4.49
- `web-sys`: 0.3.74 → 0.3.76
- `thiserror`: 2.0.4 → 2.0.9

### Changed

- Some behaviour

## [0.1.0]

### Added

- Some features
"#;

        add_changes_to_changelog_contents(
            EXAMPLE_CHANGES.to_vec(),
            &mut changelog_content,
            "Unreleased",
            "Dependencies",
        );

        assert_str_eq!(&changelog_content, expect_final_changelog_contents);
    }

    #[test]
    fn test_add_changes_to_changelog_contents_empty_changelog() {
        let mut changelog_content = EXAMPLE_EMPTY_CHANGELOG_CONTENTS.to_owned();

        let expect_final_changelog_contents = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dependencies

- `serde`: 1.0.215 → 1.0.216
- `chrono`: 0.4.38 → 0.4.39
- `semver`: 1.0.23 → 1.0.24
- `env_logger`: 0.11.5 → 0.11.6
- `zip`: 2.2.1 → 2.2.2
- `wasm-bindgen-futures`: 0.4.47 → 0.4.49
- `web-sys`: 0.3.74 → 0.3.76
- `thiserror`: 2.0.4 → 2.0.9

"#;

        add_changes_to_changelog_contents(
            EXAMPLE_CHANGES.to_vec(),
            &mut changelog_content,
            "Unreleased",
            "Dependencies",
        );

        assert_str_eq!(&changelog_content, expect_final_changelog_contents);
    }

    /// The section already contains the env_logger dependency so we expect that line to be
    /// removed and replaced by a new entry showing the highest/newest version it was bumped to
    /// but not replacing the old/former version from the previous entry as that would be misleading
    #[test]
    fn test_insert_changes_when_changes_section_exists() {
        let mut changelog_content = EXAMPLE_CHANGELOG_CONTENTS_CONTAINS_DEPENDENCIES.to_owned();

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
- `serde`: 1.0.215 → 1.0.216
- `env_logger`: 0.11.5 → 0.12.0

### Fix

- Some issue
"##;

        add_changes_to_changelog_contents(
            EXAMPLE_CHANGES_SMALL.to_vec(),
            &mut changelog_content,
            "Unreleased",
            "Dependencies",
        );

        assert_str_eq!(&changelog_content, expect_final_changelog_contents);
    }
}
