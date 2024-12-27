use std::ops;

use crate::dependabot_changes::DependabotChange;

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
                    progress = 1;
                    old_ver_start_pos = i;
                    code_block = true;
                } else if ch == '`' && code_block {
                    if progress > 7 {
                        let line_offset = old_ver_start_pos + extra_offset;
                        return Some(line[line_offset..line_offset + progress].to_owned());
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

pub fn find_h2_insert_position(changelog_content: &str, version: &str) -> Option<usize> {
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

#[derive(Debug)]
pub struct DependencyEntryLine {
    line_start: usize,
    line_len: usize,
}

impl DependencyEntryLine {
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
    for line in changelog.split_inclusive('\n') {
        for change in &mut *changes {
            if let Some(name_pos) = line.find(change.name) {
                let end_of_name_pos = name_pos + change.name.len();
                // Parse old version from semver or SHA1
                if let Some(old_ver) = find_old_ver_from_line(&line[end_of_name_pos..]) {
                    change.replace_old_version(old_ver);
                    let existing_dep = DependencyEntryLine {
                        line_start: current_pos,
                        line_len: line.len(),
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
    use crate::test_util::*;
    use pretty_assertions::assert_str_eq;

    #[test]
    fn test_find_old_version_docker_sha1() {
        let test_str = "- Bump `docker/login-action` from 3d58c274f17dffee475a5520cbe67f0a882c4dbb to 7ca345011ac4304463197fac0e56eab1bc7e6af0 ([#39](https://github.com/luftkode/settings-manager/pull/39))";
        let old_ver = find_old_ver_from_line(&test_str).unwrap();
        assert_str_eq!(&old_ver, "3d58c274f17dffee475a5520cbe67f0a882c4dbb");
    }

    #[test]
    fn test_find_old_version_actions_toolkit_semver() {
        let test_str =
            " 0.40.0 to 0.42.0</li> ([#39](https://github.com/luftkode/settings-manager/pull/39))";
        let old_ver = find_old_ver_from_line(&test_str).unwrap();
        assert_str_eq!(&old_ver, "0.40.0");
    }

    /// Non-trivial due to '@' as it has a utf-8 length of 3
    #[test]
    fn test_find_old_version_actions_toolkit_semver_non_trivial() {
        let test_str = "- Bump `<code>@​docker/actions-toolkit</code>` from 0.40.0 to 0.42.0</li>";
        let old_ver = find_old_ver_from_line(&test_str).unwrap();
        assert_str_eq!(&old_ver, "0.40.0");
    }

    #[test]
    fn test_find_old_version_docker_semver_arrow_sep() {
        let test_str = "- Bump `docker/login-action`: 0.11.5 → 0.11.6";
        let old_ver = find_old_ver_from_line(&test_str).unwrap();
        assert_str_eq!(&old_ver, "0.11.5");
    }

    #[test]
    fn test_find_old_version_submodule_short_sha1() {
        let test_str =
            "Bumps [some-submodule](https://github.com/org/repo) from `b0c35f6` to `c8bd600`.";
        let old_ver = find_old_ver_from_line(&test_str).unwrap();
        assert_str_eq!(&old_ver, "`b0c35f6`");
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
}
