mod parse;

use crate::dependabot_changes::{format_changes, DependabotChange};

pub fn add_changes_to_changelog_contents(
    changes: Vec<DependabotChange>,
    changelog_content: &mut String,
    version_header: &str,
    section_header: &str,
) {
    let changes_formatted_len = changes.iter().fold(0, |sum, c| c.formatted_len() + sum);

    let mut h3_header = format!("### {section_header}\n");
    // Reserve for the new changelog entry to avoid the worst case of allocating
    // the size of the existing content
    changelog_content.reserve(changes_formatted_len + h3_header.len() + 3); // +3 for the worst case of adding 3 newlines

    let h2_insert_pos = parse::find_h2_insert_position(changelog_content, version_header)
        .expect("Could not find the specified version h2 header");

    if let Some((existing_h3_start, existing_h3_insert_pos)) =
        parse::find_existing_h3_insert_position(&changelog_content[h2_insert_pos..], section_header)
    {
        let abs_h3_start = h2_insert_pos + existing_h3_start;
        let abs_h3_insert_pos = h2_insert_pos + existing_h3_insert_pos;
        // Parse the existing section content to see if it already contains mentions of bumping the same dependencies we are
        // about to add.
        let mut replace_line_at: Vec<(usize, usize)> = vec![];
        let h3_section = &changelog_content[abs_h3_start..abs_h3_insert_pos];
        let mut h3_section_pos = 0;
        for line in h3_section.split_inclusive('\n') {
            for change in &changes {
                if line.contains(change.name) {
                    replace_line_at.push((h3_section_pos, line.len()));
                    break;
                }
            }

            h3_section_pos += line.len();
        }
        let mut string_offset = 0;
        for (line_start, line_len) in replace_line_at.into_iter().rev() {
            changelog_content.replace_range(line_start..line_len, "");
            string_offset += line_len;
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
}

#[derive(Debug)]
struct ExistingDependency {
    line_start: usize,
    line_len: usize,
    old_ver: String,
}

fn find_existing_dependency_lines_to_replace(
    changelog: &str,
    changes: &[DependabotChange],
) -> Vec<ExistingDependency> {
    let mut existing_deps = vec![];
    let mut current_pos = 0;
    for line in changelog.split_inclusive('\n') {
        for change in changes {
            if let Some(name_pos) = line.find(change.name) {
                let end_of_name_pos = name_pos + change.name.len();
                // Parse old version from semver or SHA1
                if let Some(old_ver) = parse::find_old_ver_from_line(&line[end_of_name_pos..]) {
                    let existing_dep = ExistingDependency {
                        line_start: current_pos,
                        line_len: line.len(),
                        old_ver,
                    };
                    existing_deps.push(existing_dep);
                    break;
                }
            }
        }

        current_pos += line.len();
    }
    existing_deps
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_str_eq;

    const EXAMPLE_CHANGES: &[DependabotChange<'_>] = &[
        DependabotChange::new("`serde`", "1.0.215", "1.0.216"),
        DependabotChange::new("`chrono`", "0.4.38", "0.4.39"),
        DependabotChange::new("`semver`", "1.0.23", "1.0.24"),
        DependabotChange::new("`env_logger`", "0.11.5", "0.11.6"),
        DependabotChange::new("`zip`", "2.2.1", "2.2.2"),
        DependabotChange::new("`wasm-bindgen-futures`", "0.4.47", "0.4.49"),
        DependabotChange::new("`web-sys`", "0.3.74", "0.3.76"),
        DependabotChange::new("`thiserror`", "2.0.4", "2.0.9"),
    ];

    const EXAMPLE_CHANGES_SMALL: &[DependabotChange<'_>] = &[
        DependabotChange::new("`serde`", "1.0.215", "1.0.216"),
        DependabotChange::new("`env_logger`", "0.11.8", "0.12.0"),
    ];

    use crate::test_util::example_changelogs::*;

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
- `env_logger`: 0.11.5 → 0.12.0
- `serde`: 1.0.215 → 1.0.216

### Fix

- Some issue
"##;

        add_changes_to_changelog_contents(
            EXAMPLE_CHANGES_SMALL.to_vec(),
            &mut changelog_content,
            "Unreleased",
            "Dependencies",
        );

        eprintln!("{changelog_content}");
        assert_str_eq!(&changelog_content, expect_final_changelog_contents);
    }
}
