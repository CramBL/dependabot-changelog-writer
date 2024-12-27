/// Attempts to find the "old version" in a line describing a dependency update
/// will attempt to find semver or SHA1
/// The results will be most reliable if the line to start from after the name of the dependency
/// e.g. a line such as 'update `docker` from 1.0.2 to 1.0.5' should passed as 'from 1.0.2 to 1.0.5'
/// Guaranteed to correctly identify the version from strings such as:
/// - 'from 1.0.2 to 1.0.5'
/// - 'from 3d58c274f17dffee475a5520cbe67f0a882c4dbb to 7ca345011ac4304463197fac0e56eab1bc7e6af0'
/// - ' 0.11.5 → 0.11.6'
/// - 'from `b0c35f6` to `c8bd600`'
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

#[cfg(test)]
mod tests {
    use super::*;
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
}
