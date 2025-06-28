use std::ops;

use crate::{config::VersionHeader, dependabot_changes::dependabot_change::DependabotChange};

/// Attempts to find the "old version" in a line describing a dependency update
/// will attempt to find semver or SHA1
/// The results will be most reliable if the line to start from after the name of the dependency
/// e.g. a line such as 'update `docker` from 1.0.2 to 1.0.5' should passed as 'from 1.0.2 to 1.0.5'
///
/// # Examples
///
/// ```
/// let line = "from 1.0.2 to 1.0.5"
/// let old_ver = find_old_ver_from_line(line).unwrap();
/// assert_eq(&old_ver, "1.0.2");
///
/// let line = "from 3d58c274f17dffee475a5520cbe67f0a882c4dbb to 7ca345011ac4304463197fac0e56eab1bc7e6af0"
/// let old_ver = find_old_ver_from_line(line).unwrap();
/// assert_eq(&old_ver, "3d58c274f17dffee475a5520cbe67f0a882c4dbb");
///
/// let line = " 0.11.5 → 0.11.6"
/// let old_ver = find_old_ver_from_line(line).unwrap();
/// assert_eq(&old_ver, "0.11.5");
///
/// let line = "from `b0c35f6` to `c8bd600`"
/// let old_ver = find_old_ver_from_line(line).unwrap();
/// assert_eq(&old_ver, "`b0c35f6`");
/// ```
pub(crate) fn find_old_ver_from_line(line: &str) -> Option<String> {
    enum ParseSt {
        BeforeOld,
        SHA1,
        Minor,
        Patch,
        Extra,
    }
    let mut old_ver_start_pos = 0;
    let mut progress = 0;
    let mut st = ParseSt::BeforeOld;
    let mut maybe_semver = false;
    let mut code_block = false;
    let mut extra_offset = 0;
    for (i, ch) in line.chars().enumerate() {
        progress += 1;
        extra_offset += ch.len_utf8() - 1;

        match st {
            ParseSt::BeforeOld => {
                if ch == '`' && !code_block {
                    progress = 0;
                    old_ver_start_pos = i;
                    code_block = true;
                } else if ch == '`' && code_block {
                    if progress > 6 {
                        let line_offset = old_ver_start_pos + extra_offset;
                        let old_ver = &line[line_offset..line_offset + progress - 1];
                        return Some(old_ver.to_owned());
                    }
                } else if ch.is_ascii_digit() {
                    if progress == 1 {
                        old_ver_start_pos = i;
                        maybe_semver = true;
                    }
                    if progress > 30 {
                        // At this point it must be a Git SHA1
                        st = ParseSt::SHA1;
                    }
                } else if ch.is_ascii_hexdigit() {
                    maybe_semver = false;
                    if progress == 1 {
                        old_ver_start_pos = i;
                    }
                    if progress > 30 {
                        // At this point it must be a Git SHA1
                        st = ParseSt::SHA1;
                    }
                } else if maybe_semver && ch.eq(&'.') {
                    // We parsed digits and then a '.' so it must be semver
                    // so we start parsing minor
                    st = ParseSt::Minor;
                } else {
                    maybe_semver = false;
                    old_ver_start_pos = i;
                    progress = 0;
                }
            }
            ParseSt::SHA1 => {
                let line_offset = old_ver_start_pos + extra_offset;
                if progress == 40 {
                    return Some(line[line_offset..line_offset + 40].to_owned());
                }
            }
            ParseSt::Minor => {
                if ch == '.' {
                    st = ParseSt::Patch;
                }
            }
            ParseSt::Patch => {
                if ch == '-' || ch == '.' || ch == '+' {
                    // Prerelease or build metadata so we just accept input until newline or whitespace character
                    st = ParseSt::Extra;
                } else if !ch.is_ascii_digit() {
                    let line_offset = old_ver_start_pos + extra_offset;
                    return Some(line[line_offset..line_offset + progress - 1].to_owned());
                }
            }
            ParseSt::Extra => {
                if ch == ' ' || ch == '\n' {
                    let line_offset = old_ver_start_pos + extra_offset;
                    return Some(line[line_offset..line_offset + progress - 1].to_owned());
                }
            }
        }
    }

    None
}

pub fn find_h2_insert_position(changelog_content: &str, version: &VersionHeader) -> Option<usize> {
    let mut content_pos = 0;

    // NOTE: We're matching in every iteration but the performance impact is negligible
    // and the alternative is to duplicate the surrounding loop. We do this for simplicity.
    for l in changelog_content.split_inclusive('\n') {
        if let Some(stripped) = l.strip_prefix("##") {
            match version {
                // Match any spelling of 'unreleased' e.g. 'UNRELEASED' | 'Unreleased' | '[unreleased]' | etc..
                VersionHeader::Unreleased => {
                    if stripped.to_lowercase().contains("unreleased") {
                        return Some(content_pos + l.len());
                    }
                }

                // Match exactly the specified string
                VersionHeader::Custom(v) => {
                    if stripped.contains(v) {
                        return Some(content_pos + l.len());
                    }
                }
            }
        }
        content_pos += l.len();
    }

    None
}

// Returns the start and end position of the target H3 header.
// Returns None if the header is not found
pub fn find_existing_h3_insert_position(
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

pub fn find_new_h3_insert_position(changelog_content: &str) -> usize {
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

#[derive(Debug, PartialEq)]
pub struct DependencyEntryLine {
    line_start: usize,
    line_len: usize,
}

impl DependencyEntryLine {
    #[expect(dead_code)]
    pub fn range(&self) -> ops::Range<usize> {
        self.line_start..self.line_start + self.line_len
    }

    pub fn range_offset(&self, offset: usize) -> ops::Range<usize> {
        let start = self.line_start + offset;
        start..start + self.line_len
    }
}

pub fn find_existing_dependency_lines_to_replace(
    changelog: &str,
    changes: &mut [DependabotChange],
) -> Vec<DependencyEntryLine> {
    let mut existing_deps = vec![];
    let mut current_pos = 0;
    let mut seen_changes = vec![]; // ensure we only replace the first instance

    for line in changelog.split_inclusive('\n') {
        for change in &mut *changes {
            if seen_changes.contains(&change.name) {
                continue; // Skip if this change has already been matched
            }
            if let Some(name_pos) = line.find(change.name) {
                // We might have a partial match so we need to ensure it's not a match
                // on e.g. 'clap' in 'clap_complete'
                let end_of_name_pos = name_pos + change.name.len();
                const VALID_SURROUNDING_CHARS_IF_EQ: [char; 3] = ['`', '_', '*'];
                const EXTRA_VALID_POST_CHARS: [char; 2] = [':', '→'];

                let valid_boundary = {
                    let mut next_must_match = None;
                    let pre_char_ok = if let Some(prev_char) = line.chars().nth(name_pos - 1) {
                        if VALID_SURROUNDING_CHARS_IF_EQ.contains(&prev_char) {
                            next_must_match = Some(prev_char);
                        }
                        prev_char.is_whitespace() || next_must_match.is_some()
                    } else {
                        true
                    };

                    let post_char_ok = if let Some(next_char) = line.chars().nth(end_of_name_pos) {
                        if let Some(must_match) = next_must_match {
                            must_match == next_char
                        } else {
                            next_char.is_whitespace() || EXTRA_VALID_POST_CHARS.contains(&next_char)
                        }
                    } else {
                        true
                    };

                    pre_char_ok && post_char_ok
                };

                if valid_boundary {
                    // Parse old version from semver or SHA1
                    let curr_line = &line[end_of_name_pos..];
                    if let Some(old_ver) = find_old_ver_from_line(curr_line) {
                        log::trace!(
                            "Found old version: '{old_ver}' from current line: '{curr_line}'"
                        );
                        change.replace_old_version(old_ver);
                        let existing_dep = DependencyEntryLine {
                            line_start: current_pos,
                            line_len: line.len(),
                        };
                        existing_deps.push(existing_dep);
                        seen_changes.push(change.name);
                        break;
                    }
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
    use crate::test_util::*;
    use pretty_assertions::assert_str_eq;
    use test_log::test;

    #[test]
    fn test_find_old_version_docker_sha1() {
        let test_str = "- Bump `docker/login-action` from 3d58c274f17dffee475a5520cbe67f0a882c4dbb to 7ca345011ac4304463197fac0e56eab1bc7e6af0 ([#39](https://github.com/bumps_org/updates-versioner/pull/39))";
        let old_ver = find_old_ver_from_line(test_str).unwrap();
        assert_str_eq!(&old_ver, "3d58c274f17dffee475a5520cbe67f0a882c4dbb");
    }

    #[test]
    fn test_find_old_version_actions_toolkit_semver() {
        let test_str = " 0.40.0 to 0.42.0</li> ([#39](https://github.com/bumps_org/updates-versioner/pull/39))";
        let old_ver = find_old_ver_from_line(test_str).unwrap();
        assert_str_eq!(&old_ver, "0.40.0");
    }

    /// Non-trivial due to '@' as it has a utf-8 length of 3
    #[test]
    fn test_find_old_version_actions_toolkit_semver_non_trivial() {
        #[expect(
            clippy::invisible_characters,
            reason = "The '@' contains invisible characters, but this is in fact what we need to handle as it is verbetum content from a dependabot PR"
        )]
        let test_str = "- Bump `<code>@​docker/actions-toolkit</code>` from 0.40.0 to 0.42.0</li>";
        let old_ver = find_old_ver_from_line(test_str).unwrap();
        assert_str_eq!(&old_ver, "0.40.0");
    }

    #[test]
    fn test_find_old_version_docker_semver_arrow_sep() {
        let test_str = "- Bump `docker/login-action`: 0.11.5 → 0.11.6";
        let old_ver = find_old_ver_from_line(test_str).unwrap();
        assert_str_eq!(&old_ver, "0.11.5");
    }

    #[test]
    fn test_find_old_version_submodule_short_sha1() {
        let test_str =
            "Bumps [some-submodule](https://github.com/org/repo) from `b0c35f6` to `c8bd600`.";
        let old_ver = find_old_ver_from_line(test_str).unwrap();
        assert_str_eq!(&old_ver, "b0c35f6");
    }

    #[test]
    fn test_find_old_version_semver_in_code_blocks() {
        let test_str = "- Update _github/codeql-action_ from `3.28.17` to [`3.28.18`](https://github.com/github/codeql-action/releases/tag/v3.28.18). ([#17](https://github.com/foo-bar/build-workflows/pull/17)) _@dependabot_";
        let old_ver = find_old_ver_from_line(test_str).unwrap();
        assert_str_eq!(&old_ver, "3.28.17");
    }

    #[test]
    fn test_find_insert_position_empty_changelog() {
        let changelog_content = EXAMPLE_EMPTY_CHANGELOG_CONTENTS;
        let insert_pos =
            find_h2_insert_position(changelog_content, &VersionHeader::Unreleased).unwrap();
        assert_eq!(insert_pos, 269);

        let insert_h3_pos =
            find_existing_h3_insert_position(&changelog_content[insert_pos..], "Dependencies");
        assert_eq!(insert_h3_pos, None);
    }

    #[test]
    fn test_find_insert_position_version_empty_changelog() {
        let insert_pos = find_h2_insert_position(
            EXAMPLE_EMPTY_CHANGELOG_CONTENTS,
            &VersionHeader::new("0.1.0".to_owned()),
        );
        assert_eq!(insert_pos, None);
    }

    #[test]
    fn test_find_insert_position_used_changelog() {
        let changelog_content = EXAMPLE_USED_CHANGELOG_CONTENTS;
        let insert_pos =
            find_h2_insert_position(changelog_content, &VersionHeader::Unreleased).unwrap();
        assert_eq!(insert_pos, 269);

        let insert_h3_pos =
            find_existing_h3_insert_position(&changelog_content[insert_pos..], "Dependencies");
        assert_eq!(insert_h3_pos, None);
    }

    #[test]
    fn test_find_insert_position_version_used_changelog() {
        let insert_pos = find_h2_insert_position(
            EXAMPLE_USED_CHANGELOG_CONTENTS,
            &VersionHeader::Custom("1.3.5".to_owned()),
        )
        .unwrap();
        assert_eq!(insert_pos, 281);
    }

    #[test]
    fn test_find_insert_position_small_changelog() {
        let changelog_content = EXAMPLE_SMALL_CHANGELOG_CONTENTS_NO_NEWLINE;
        let insert_pos =
            find_h2_insert_position(changelog_content, &VersionHeader::Unreleased).unwrap();
        assert_eq!(insert_pos, 29);

        let insert_h3_pos =
            find_existing_h3_insert_position(&changelog_content[insert_pos..], "Dependencies");
        assert_eq!(insert_h3_pos, None);
    }

    #[test]
    fn test_find_existing_dependencies_to_replace_simple() {
        let changelog = EXAMPLE_CHANGELOG_CONTENTS_CONTAINS_DEPENDENCIES;
        let mut changes = EXAMPLE_CHANGES_SMALL.to_vec();
        let to_replace = find_existing_dependency_lines_to_replace(changelog, &mut changes);
        assert_eq!(changes[0], EXAMPLE_CHANGES_SMALL[0]);
        assert_eq!(
            changes[1].old_version(),
            "0.11.5",
            "Expected env_logger version to be replaced by the existing entry from the changelog"
        );

        assert_eq!(to_replace.len(), 1);
        assert_eq!(to_replace[0].line_start, 345);
        assert_eq!(to_replace[0].line_len, 34);
    }

    #[test]
    fn test_find_insert_pos_issue51() {
        let changelog_content = ISSUE_51_CHANGELOG;
        let version_header = VersionHeader::Unreleased;
        let section_header = "Dependencies";

        let expect_h2_pos = 29;

        let insert_pos = find_h2_insert_position(&changelog_content, &version_header).unwrap();
        assert_eq!(insert_pos, expect_h2_pos);
        eprintln!("{}", &changelog_content[..insert_pos - 1]);
        assert!(changelog_content[..insert_pos - 1].ends_with("## [unreleased]"));

        let (expect_h3_rel_pos_start, expect_h3_rel_pos_end) = (18, 104);

        let insert_h3_pos =
            find_existing_h3_insert_position(&changelog_content[insert_pos..], section_header);
        assert_eq!(
            insert_h3_pos,
            Some((expect_h3_rel_pos_start, expect_h3_rel_pos_end))
        );

        let abs_h3_pos_start = expect_h2_pos + expect_h3_rel_pos_start;
        let until_abs_h3_start = &changelog_content[..abs_h3_pos_start - 1];
        eprintln!("{until_abs_h3_start}");
        assert!(until_abs_h3_start.ends_with(&format!("### {}", section_header)));

        let abs_h3_pos_end = expect_h2_pos + expect_h3_rel_pos_end;
        let end_of_h3_plus_11_chars = &changelog_content[abs_h3_pos_end..abs_h3_pos_end + 11];
        eprintln!("{}", end_of_h3_plus_11_chars);
        assert!(end_of_h3_plus_11_chars.ends_with("## [0.4.2]"));
    }

    #[test]
    fn test_find_existing_dependencies_to_replace() {
        let changelog = r##"# Changelog

## [Unreleased]

### Added

- Some feature about the environment

### Dependencies

- `chrono`: 0.4.38 → 0.4.39
- `env_logger`: 0.11.5 → 0.11.6
- env: 0.1.5 → 0.3.2
- envdir: 0.1.0 → 0.2.0
- direnv: 1.1.0 → 2.1.0
- env_loggerif: 1.1.5 → 1.1.6
- `semver`: 1.0.23 → 1.0.24
- update senvir from 0.2.1 to 0.2.3
- update _envy_ from `0.1.1` to [0.1.2](https://github.com/foo/envy/releases/tag/v0.1.2)

### Fix

- Some issue
"##;
        let mut changes = vec![
            DependabotChange::new("env", "0.3.2", "0.3.33"),
            DependabotChange::new("env_logger", "0.11.8", "0.12.0"),
        ];
        let to_replace = find_existing_dependency_lines_to_replace(changelog, &mut changes);
        assert_str_eq!(
            changes[0].old_version(),
            "0.1.5",
            "expected old 'env' version to be replaced by the existing entry from the changelog"
        );
        assert_str_eq!(
            changes[1].old_version(),
            "0.11.5",
            "Expected old 'env_logger' version to be replaced by the existing entry from the changelog"
        );
        pretty_assertions::assert_eq!(
            to_replace,
            vec![
                DependencyEntryLine {
                    line_start: 127,
                    line_len: 34
                },
                DependencyEntryLine {
                    line_start: 161,
                    line_len: 23
                }
            ]
        );
    }

    #[test]
    fn test_find_existing_dependencies_to_replace_issue90() {
        let changelog = r##"# Foo Workflows for GitHub Actions Changelog

<!-- markdownlint-disable-next-line MD052 -->
> [!NOTE]
> All notable changes to this project will be documented in this file; the format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!--
### Added - For new features.
### Changed - For changes in existing functionality.
### Deprecated - For soon-to-be removed features.
### Removed - For now removed features.
### Fixed - For any bug fixes.
### Security - In case of vulnerabilities.
-->

## [UNRELEASED]

### Changed

- Update _github/codeql-action_ from `3.28.17` to [`3.28.18`](https://github.com/github/codeql-action/releases/tag/v3.28.18). ([#17](https://github.com/foo-bar/build-workflows/pull/17)) _@dependabot_
- Update _docker/build-push-action_ from `6.16.0` to [`6.17.0`](https://github.com/docker/build-push-action/releases/tag/v6.17.0). ([#17](https://github.com/foo-bar/build-workflows/pull/17)) _@dependabot_
- Update _github/codeql-action_ from `3.28.18` to [`3.28.19`](https://github.com/github/codeql-action/releases/tag/v3.28.19). ([#18](https://github.com/foo-bar/build-workflows/pull/18)) _@dependabot_
- Update _docker/build-push-action_ from `6.17.0` to [`6.18.0`](https://github.com/docker/build-push-action/releases/tag/v6.17.0). ([#18](https://github.com/foo-bar/build-workflows/pull/18)) _@dependabot_
"##;
        let mut changes = CHANGES_ISSUE_90.to_vec();
        let to_replace = find_existing_dependency_lines_to_replace(changelog, &mut changes);
        // Actually the changelog has 2 entries with older versions, but we don't really support this so we just replace the first one seen
        let expect_replaced_version = "3.28.17";

        assert_str_eq!(
            changes[0].old_version(),
            expect_replaced_version,
            "expected old 'github/codeql-action' version to be replaced by the existing entry from the changelog"
        );
        pretty_assertions::assert_eq!(
            to_replace,
            vec![DependencyEntryLine {
                line_start: 629,
                line_len: 200
            }]
        );
    }
}
