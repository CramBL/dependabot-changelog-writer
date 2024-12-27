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

    let h2_insert_pos = find_h2_insert_position(changelog_content, version_header)
        .expect("Could not find the specified version h2 header");

    if let Some((existing_h3_start, existing_h3_insert_pos)) =
        find_existing_h3_insert_position(&changelog_content[h2_insert_pos..], section_header)
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
        let new_h3_insert_pos = find_new_h3_insert_position(&changelog_content[h2_insert_pos..]);
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
    //
    let mut existing_deps = vec![];
    let mut current_pos = 0;
    for line in changelog.split_inclusive('\n') {
        for change in changes {
            if line.contains(change.name) {
                let existing_dep = ExistingDependency {
                    line_start: current_pos,
                    line_len: line.len(),
                    old_ver: todo!(),
                };

                // Parse old version from semver or SHA1

                existing_deps.push(existing_dep);
                break;
            }
        }

        current_pos += line.len();
    }
    existing_deps
}

fn find_h2_insert_position(changelog_content: &str, version: &str) -> Option<usize> {
    let mut content_pos = 0;
    for l in changelog_content.split_inclusive('\n') {
        if l.starts_with("##") && l[2..].contains(version) {
            return Some(content_pos + l.len());
        }

        content_pos += l.len();
    }
    None
}

// Returns the start and end position of the target H3 header.
// Returns None if the header is not found
fn find_existing_h3_insert_position(
    changelog_content: &str,
    section_header: &str,
) -> Option<(usize, usize)> {
    let mut content_pos = 0;
    for l in changelog_content.split_inclusive('\n') {
        content_pos += l.len();
        if l.starts_with("###") {
            if l[2..].contains(section_header) {
                let mut offset_within_section = 0;
                for l in changelog_content[content_pos..].split_inclusive('\n') {
                    if l.starts_with("##") {
                        // Go back one to prevent extra blank lines
                        offset_within_section -= 1;
                        break;
                    }
                    offset_within_section += l.len();
                }

                return Some((content_pos, content_pos + offset_within_section));
            }
        } else if l.starts_with("##") {
            return None;
        }
    }
    None
}

fn find_new_h3_insert_position(changelog_content: &str) -> usize {
    let mut content_pos = 0;
    for l in changelog_content.split_inclusive('\n') {
        // First check for h3 header then h2 header
        if l.starts_with("###") {
            content_pos += l.len();
        } else if l.starts_with("##") {
            return content_pos;
        } else {
            content_pos += l.len();
        }
    }
    content_pos
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

    const EXAMPLE_EMPTY_CHANGELOG_CONTENTS: &str = r##"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
"##;

    const EXAMPLE_CHANGELOG_CONTENTS_CONTAINS_DEPENDENCIES: &str = r##"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Some feature

### Dependencies

- `chrono`: 0.4.38 → 0.4.39
- `env_logger`: 0.11.5 → 0.11.6
- `semver`: 1.0.23 → 1.0.24

### Fix

- Some issue
"##;

    const EXAMPLE_SMALL_CHANGELOG_CONTENTS: &str = r##"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Some behaviour

## [0.1.0]

### Added

- Some features
"##;

    const EXAMPLE_SMALL_CHANGELOG_WITH_DEPENDENCIES_CONTENTS: &str = r##"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dependencies

- Bump egui from 0.29.0 to 0.30.0

### Changed

- Some behaviour

## [0.1.0]

### Added

- Some features
"##;

    const EXAMPLE_USED_CHANGELOG_CONTENTS: &str = r##"# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [unreleased]

## [1.3.5]

### Changed

- Update egui from `0.29.0` to `0.30.0`
- Port fixes from `eframe_template`

## [1.3.4]

### Changed

- Only set font styles when it changes instead of at every frame (leftover tech-debt from starting to learn egui)
- Make the loaded files window scrollable - resolves https://github.com/luftkode/plotinator3000/issues/118

## [1.3.3]

### Changed

- Update dependencies

## [1.3.2]

### Changed

- Bump Rust compiler from `1.81.0` to `1.82.0`
- Bump dependencies

## [1.3.1]

### Fix

- Selfupdater failing to determine install receipt prevents it from doing an upgrade

## [1.3.0]

### Added

- Show/hide all button for loaded files window

### Changed

- Bump `thiserror` to 2.0
- Bump dependencies

## [1.2.2]

### Fixed

- Self-updater has been fixed and re-enabled

## [1.2.1]

### Changed

- Update Mbed log v4 with the new config changes.

## [1.2.0]

### Changed

- Re-enable installation of an updater executable
- MBED logs now normalize `servo duty cycle` to 0-100%. The full range of the servo duty cycle is [0; 0.1].

### Internal

- Bump cargo-dist from 0.23.0 -> 0.25.1
- Bump axoupdater from 0.7.3 -> 0.8.1

## [1.1.2]

### Changed

- Guard self-updater behind feature flag as it is currently broken.

## [1.1.1]

### Changed

- When auto downsampling is enabled and the mipmap level for a given plot is determined to be 1, we use all data points instead of downsampling to level 1. When we are plotting a downsampled min and max, level 1 is just as many data points as the original non-downsampled plot, so this is a strictly better solution.

## [1.1.0]

### Added

- Support for Mbed log v3
- Preliminary support for Mbed log v3

## [1.0.2]

### Fix

- Updater was looking for an install receipt which plotinator3000 no longer uses. The updater now proceeds without needing an install receipt.

## [1.0.1]

### Changed

- Added a notification whenever a log is loaded, showing the total data points of loaded files.

### Internal

- Cleanup unused library code
- Prepare support for a new version of the mbed config present in mbed log headers.

## [1.0.0]

### Changed

- `logviewer-rs` is now renamed to `Plotinator3000`, signifying that it is not really a logviewer and more of a plotting app that will plot any supported format, and do it very fast.

## [0.28.0]

### Added

- Auto updater that queries for newer versions and opens an installer window if a new update is available

## [0.27.0]

### Added

- Support for the `NavSys.sps` format.

## [0.26.0]

### Fix

- Plot alignment

### Changed

- Make some UI elements smaller
- Allow main window (viewport) to be shrink much more than before
- Plot setting UI elements wrap instead of stay fixed when window shrinks

## [0.25.0]

### Added

- File dialog for native and web, which also allows mobile users to load logs.

### Changed

- Various UI tweaks
- Clean up some outdated error messages.

### Internals

- Decouple file parsing from file loading method.

## [0.24.1]

### Fix

- Web version of `plotinator3000` was broken due an integer overflow. When determining down sample level, a cast from 64-bit float to pointer size caused integer overflow on wasm due to wasm having a 32-bit pointer size.

## [0.24.0]

### Added

- Initial support for `HDF` files, starting with bifrost (TX) loop current. The feature is currently guarded behind a feature flag, enabling it is tracked at: https://github.com/luftkode/plotinator3000/issues/84.

### Changed

- Various UI tweaks

### Internal

- Upgraded `cargo-dist` `0.22.1` -> `0.23.0`

## [0.23.0]

### Added

- A warning notification is now shown if a log was parsed from contents where more than 128 bytes of the content was not recognized as log content (and therefor skipped)
- When viewing log info, the first line shows parse info, how many bytes were parsed out of the total length of the file.

### Changed

- Plot labels are now sorted alphabetically
- Remove unused `T_SHUTDOWN` config value that was not supposed to be in mbed log v2.
- Avoid downsampling all the way to 2 samples by setting a minimum downsample threshold (set to 512 samples)
- Avoid storing redundant copies of source plot data when creating multiple mipmaps from the same source.

### Internals

- Refactor to simplify mipmap configuration

## [0.22.0]

### Changed

- Plots retain the color they originally were assigned, no matter if logs are toggled to be invisible or a plot filter is hiding some plots.
- Min/max downsampled plots now have the same color

### Internals

- Refactor to reduce a bunch of duplication and tech debt

## [0.21.0]

### Changed

- Much faster way of determining which plot points that fit within plot boundings.
- Avoid double work when auto downsampling is enabled, previously the fitting downsampling level was first found before handing off that level to a filtering function, which would find partition bounds that were already known from finding the fitting downsampling level.

## [0.20.0]

### Added

- Display log metadata when clicking on a loaded log.

### Fix

- `Mbed status Log v2` mistakenly interpreted as `Mbed status Log v1` when loaded via a path (native `plotinator3000`)

## [0.19.0]

### Added

- Support for Mbed Log version 2

## [0.18.4]

### Changed

- Remove playback feature

## [0.18.3]

### Fix

- Integer overflow when searching for the appropriate downsampled MipMap level.

## [0.18.2]

### Changed

- Much faster implementation for finding the appropriate MipMap level for the current zoom level.

### Fix

- Min/Max MipMap algorithm flipped min and max.

## [0.18.1]

### Fix

- Fix accidentally using plot **name** instead of plot **label**, causing name conflicts when plotting multiple plots with the same name
- Bad naming in a PID controller implementation caused misunderstanding, what we thought was the PID error was actually the PID output.

## [0.18.0]

### Added

- Min/Max MipMap downsampling which makes plotting much larger datasets feasible, and facilitates outlier detection.

### Internals

- Major clearing of tech debts related to plot settings and the ui for plot settings.

## [0.17.0]

### Added

- Allow toggling whether or not plots from each loaded log are shown.

### Internals

- Update dependencies

## [0.16.0]

### Added

- Hovering on a plot now also shows the plot name.
- Allow Filtering the shown plots by legend/label name.

### Changed

- (Native only) Recursively parsing drag-n-dropped directories also parses zip archives
- Reduce UI by allowing toggling the list of logs.
- Reduce UI cluttering by removing the "Time" label on the X-axis.
- Reduce verbosity of the name of logs
- Better visuals for viewing and changing settings of loaded logs

### Internals

- Refactors

## [0.15.0]

### Added

- (Not available on web version) Recursively parse drag-n-dropped directories for supported logs
- (Not available on web version) Recursively parse drag-n-dropped zip archives for supported logs
- Show tooltip when hovering above a clickable log name

### Internals

- Refactors

## [0.14.0]

### Add

- Ability for logs to add labels (rough initial mvp, needs more work)
- `show grid` button for showing/hiding grid lines on plots

## [0.13.0]

### Internals

- Better divide interfaces in the `log_if` crate and use a prelude to make it easy to import all relevant traits via glob imports.
- Change `GitMetadata` trait to return `Option<String>` to accommodate logs that don't contain git metadata.

### Changes

- Mbed motor control PID log's `RPM Error Count` and `First Valid RPM Count` are moved to the plot ranging from 1-100.

## [0.12.0]

### Changes

- Allow Setting the date/offset of plots with the `Enter`-key and closing the settings window by pressing `Escape`
- Upgrade `egui` to v0.29
- Y-axis lock is now compatible with all zoom and scroll actions

### Internals

- Migrate to workspace project structure
- Decouple Log implementation from the plotting interface"##;

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

    #[test]
    fn test_find_insert_position_empty_changelog() {
        let changelog_content = EXAMPLE_EMPTY_CHANGELOG_CONTENTS;
        let insert_pos = find_h2_insert_position(changelog_content, "Unreleased").unwrap();
        assert_eq!(insert_pos, 269);

        let insert_h3_pos =
            find_existing_h3_insert_position(&changelog_content[insert_pos..], "Dependencies");
        assert_eq!(insert_h3_pos, None);
    }

    #[test]
    fn test_find_insert_position_version_empty_changelog() {
        let insert_pos = find_h2_insert_position(EXAMPLE_EMPTY_CHANGELOG_CONTENTS, "0.1.0");
        assert_eq!(insert_pos, None);
    }

    #[test]
    fn test_find_insert_position_used_changelog() {
        let changelog_content = EXAMPLE_USED_CHANGELOG_CONTENTS;
        let insert_pos = find_h2_insert_position(changelog_content, "unreleased").unwrap();
        assert_eq!(insert_pos, 269);

        let insert_h3_pos =
            find_existing_h3_insert_position(&changelog_content[insert_pos..], "Dependencies");
        assert_eq!(insert_h3_pos, None);
    }

    #[test]
    fn test_find_insert_position_version_used_changelog() {
        let insert_pos = find_h2_insert_position(EXAMPLE_USED_CHANGELOG_CONTENTS, "1.3.5").unwrap();
        assert_eq!(insert_pos, 281);

        let remainder = EXAMPLE_USED_CHANGELOG_CONTENTS[insert_pos..].to_owned();
        eprintln!("{}", remainder);
    }

    #[test]
    fn test_find_insert_position_small_changelog() {
        let changelog_content = EXAMPLE_SMALL_CHANGELOG_CONTENTS;
        let insert_pos = find_h2_insert_position(changelog_content, "Unreleased").unwrap();
        assert_eq!(insert_pos, 269);

        let insert_h3_pos =
            find_existing_h3_insert_position(&changelog_content[insert_pos..], "Dependencies");
        assert_eq!(insert_h3_pos, None);
    }
}
