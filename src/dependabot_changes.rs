use crate::config::EntrySeparator;
use crate::event_json::PullRequestInfo;
use dependabot_change::DependabotChange;
use entry_pattern::EntryPattern;

pub mod dependabot_change;
pub mod entry_pattern;
pub mod old_version;

pub fn extract_markdown_links(text: &str) -> Vec<(&str, &str)> {
    let mut links = Vec::new();
    let mut rest = text;

    while let Some(start_bracket) = rest.find('[') {
        let after_start = &rest[start_bracket + 1..];
        if let Some(end_bracket) = after_start.find(']') {
            let label = &after_start[..end_bracket];
            let after_bracket = &after_start[end_bracket + 1..];
            if let Some(stripped) = after_bracket.strip_prefix('(') {
                if let Some(end_paren) = stripped.find(')') {
                    let url = &stripped[..end_paren];
                    links.push((label.trim(), url.trim()));
                    rest = &stripped[end_paren + 1..];
                    continue;
                }
            }
        }
        rest = after_start;
    }

    links
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || matches!(c, '_' | '-' | '/' | '@')
}

fn is_boundary_match(label: &str, dep_name: &str) -> bool {
    if dep_name.is_empty() || label.len() < dep_name.len() {
        return false;
    }

    let mut start = 0;
    while let Some(rel_pos) = label[start..].find(dep_name) {
        let match_start = start + rel_pos;
        let match_end = match_start + dep_name.len();

        let prev_ok = match_start == 0
            || label[..match_start]
                .chars()
                .last()
                .is_some_and(|c| !is_identifier_char(c));

        let next_ok = match_end == label.len()
            || label[match_end..]
                .chars()
                .next()
                .is_some_and(|c| !is_identifier_char(c));

        if prev_ok && next_ok {
            return true;
        }

        let next_char_len = label[match_start..]
            .chars()
            .next()
            .map_or(1, |c| c.len_utf8());
        start = match_start + next_char_len;
    }

    false
}

pub fn parse_body(body: &str) -> Vec<DependabotChange<'_>> {
    let mut changes = parse_changes(body);
    let links = extract_markdown_links(body);

    for change in &mut changes {
        let dep_name = change.dep_name();
        let matched_url = links
            .iter()
            .find(|&&(label, _)| label.trim().trim_matches('`') == dep_name)
            .or_else(|| {
                links
                    .iter()
                    .find(|&&(label, _)| is_boundary_match(label, dep_name))
            })
            .map(|&(_, url)| url);

        if let Some(url) = matched_url {
            change.set_url(url);
        }
        log::debug!("{change:?}");
    }
    changes
}

const UPDATE_LINE_KEYWORD_A: &str = "Updates";
const UPDATE_LINE_KEYWORD_B: &str = "Bumps";

const UPDATE_KEYWORDS: [(&str, usize); 2] = [
    (UPDATE_LINE_KEYWORD_A, UPDATE_LINE_KEYWORD_A.len()),
    (UPDATE_LINE_KEYWORD_B, UPDATE_LINE_KEYWORD_B.len()),
];

fn parse_changes(body: &str) -> Vec<DependabotChange<'_>> {
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

pub fn format_changes(
    changes: &[DependabotChange],
    entry_pattern: &EntryPattern,
    entry_separator: EntrySeparator,
    pr_info: &PullRequestInfo,
) -> String {
    let mut result = String::new();
    let newline = entry_separator.newline();
    for (i, change) in changes.iter().enumerate() {
        if i > 0 {
            result.push_str(entry_separator.as_str());
        }
        let formatted = entry_pattern.format_change(change, pr_info);
        let trimmed = formatted.trim_end_matches(['\r', '\n']);
        result.push_str(trimmed);
    }
    if !result.is_empty() {
        result.push_str(newline);
    }
    result
}

pub fn get_formatted_changes(
    abs_h3_contents: &str,
    changes: &[DependabotChange],
    entry_pattern: &EntryPattern,
    entry_separator: EntrySeparator,
    pr_info: &PullRequestInfo,
) -> String {
    let newline = entry_separator.newline();
    let mut new_changes = Vec::new();
    for change in changes {
        let entry = entry_pattern.format_change(change, pr_info);
        let trimmed_entry = entry.trim_end_matches(['\r', '\n']);
        let mut already_exists = false;
        for line in abs_h3_contents.lines() {
            if line.trim_end_matches(['\r', '\n']) == trimmed_entry {
                already_exists = true;
                break;
            }
        }
        if !already_exists {
            new_changes.push(change.clone());
        }
    }

    if new_changes.is_empty() {
        return String::new();
    }

    let mut result = format_changes(&new_changes, entry_pattern, entry_separator, pr_info);

    if entry_separator.is_blank_line() && abs_h3_contents.contains(EntryPattern::LINE_PREFIX) {
        let trailing_newlines = if abs_h3_contents.ends_with("\r\n\r\n") {
            2
        } else if abs_h3_contents.ends_with("\r\n") || abs_h3_contents.ends_with('\n') {
            if abs_h3_contents.ends_with("\n\n") {
                2
            } else {
                1
            }
        } else {
            0
        };

        let missing_newlines = 2usize.saturating_sub(trailing_newlines);
        if missing_newlines > 0 {
            result.insert_str(0, &newline.repeat(missing_newlines));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::*;
    use pretty_assertions::{assert_eq, assert_str_eq};

    #[test]
    fn test_parse_body() {
        let changes = parse_body(DEPENDABOT_BODY_2_ACTIONS_SHA_SEMVER);
        let entry_pattern = &EntryPattern::default();
        assert_eq!(changes.len(), 2);
        assert_eq!(
            changes[0],
            DependabotChange::new("crate-ci/typos", "1.27.0", "1.28.4")
        );
        assert_eq!(
            changes[1],
            DependabotChange::new(
                "docker/login-action",
                "3d58c274f17dffee475a5520cbe67f0a882c4dbb",
                "7ca345011ac4304463197fac0e56eab1bc7e6af0"
            )
        );
        let changes_md = format_changes(
            &changes,
            entry_pattern,
            EntrySeparator::default(),
            &example_pr_info(),
        );
        let expect_md = "\
        - `crate-ci/typos`: 1.27.0 → 1.28.4 ([#1](https://github.com/user/repo/pull/1))\n\
        - `docker/login-action`: 3d58c274f17dffee475a5520cbe67f0a882c4dbb → 7ca345011ac4304463197fac0e56eab1bc7e6af0 ([#1](https://github.com/user/repo/pull/1))\n";
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
            DependabotChange::new("serde", "1.0.215", "1.0.216")
        );
        assert_eq!(
            changes[1],
            DependabotChange::new("chrono", "0.4.38", "0.4.39")
        );
        assert_eq!(
            changes[2],
            DependabotChange::new("semver", "1.0.23", "1.0.24")
        );
        assert_eq!(
            changes[3],
            DependabotChange::new("env_logger", "0.11.5", "0.11.6")
        );
        assert_eq!(changes[4], DependabotChange::new("zip", "2.2.1", "2.2.2"));
        assert_eq!(
            changes[5],
            DependabotChange::new("wasm-bindgen-futures", "0.4.47", "0.4.49")
        );
        assert_eq!(
            changes[6],
            DependabotChange::new("web-sys", "0.3.74", "0.3.76")
        );
        assert_eq!(
            changes[7],
            DependabotChange::new("thiserror", "2.0.4", "2.0.9")
        );

        let changes_md = format_changes(
            &changes,
            entry_pattern,
            EntrySeparator::default(),
            &example_pr_info(),
        );
        let expect_md = "\
        - `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))\n\
        - `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\n\
        - `semver`: 1.0.23 → 1.0.24 ([#1](https://github.com/user/repo/pull/1))\n\
        - `env_logger`: 0.11.5 → 0.11.6 ([#1](https://github.com/user/repo/pull/1))\n\
        - `zip`: 2.2.1 → 2.2.2 ([#1](https://github.com/user/repo/pull/1))\n\
        - `wasm-bindgen-futures`: 0.4.47 → 0.4.49 ([#1](https://github.com/user/repo/pull/1))\n\
        - `web-sys`: 0.3.74 → 0.3.76 ([#1](https://github.com/user/repo/pull/1))\n\
        - `thiserror`: 2.0.4 → 2.0.9 ([#1](https://github.com/user/repo/pull/1))\n";
        assert_str_eq!(changes_md, expect_md);
    }

    #[test]
    fn test_parse_example_to_changes_7_crates_semver_custom_pattern() {
        let changes = EXAMPLE_CHANGES.to_vec();
        let entry_pattern = &EntryPattern::new("Bump `[dep]` from [old] to [new]").unwrap();

        let changes_md = format_changes(
            &changes,
            entry_pattern,
            EntrySeparator::default(),
            &example_pr_info(),
        );
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

        let changes_md = format_changes(
            &changes,
            entry_pattern,
            EntrySeparator::default(),
            &example_pr_info(),
        );
        let expect_md = "\
        - 📝 Update serde from 🩺1.0.215🩺 🚀 🍄1.0.216🍄\n\
        - 📝 Update chrono from 🩺0.4.38🩺 🚀 🍄0.4.39🍄\n\
        - 📝 Update semver from 🩺1.0.23🩺 🚀 🍄1.0.24🍄\n\
        - 📝 Update env_logger from 🩺0.11.5🩺 🚀 🍄0.11.6🍄\n\
        - 📝 Update zip from 🩺2.2.1🩺 🚀 🍄2.2.2🍄\n\
        - 📝 Update wasm-bindgen-futures from 🩺0.4.47🩺 🚀 🍄0.4.49🍄\n\
        - 📝 Update web-sys from 🩺0.3.74🩺 🚀 🍄0.3.76🍄\n\
        - 📝 Update thiserror from 🩺2.0.4🩺 🚀 🍄2.0.9🍄\n";
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

    #[test]
    fn test_parse_body_issue51() {
        let changes = parse_changes(DEPENDABOT_BODY_ISSUE_51);
        assert_eq!(changes, CHANGES_ISSUE_51);
    }

    #[test]
    fn test_parse_body_issue90() {
        let changes = parse_changes(DEPENDABOT_BODY_ISSUE_90);
        assert_eq!(changes, CHANGES_ISSUE_90);
    }

    #[test]
    fn test_get_formatted_changes_blank_line_zero_trailing_newlines() {
        let existing = "### Dependencies\n\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))";
        let changes = vec![DependabotChange::new("chrono", "0.4.38", "0.4.39")];
        let entry_pattern = EntryPattern::default();

        let formatted = get_formatted_changes(
            existing,
            &changes,
            &entry_pattern,
            EntrySeparator::DOUBLE_LF,
            &example_pr_info(),
        );
        assert_str_eq!(
            formatted,
            "\n\n- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\n"
        );
        let combined = format!("{existing}{formatted}");
        assert_eq!(
            combined,
            "### Dependencies\n\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))\n\n- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\n"
        );
    }

    #[test]
    fn test_get_formatted_changes_blank_line_one_trailing_newline() {
        let existing = "### Dependencies\n\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))\n";
        let changes = vec![DependabotChange::new("chrono", "0.4.38", "0.4.39")];
        let entry_pattern = EntryPattern::default();

        let formatted = get_formatted_changes(
            existing,
            &changes,
            &entry_pattern,
            EntrySeparator::DOUBLE_LF,
            &example_pr_info(),
        );
        assert_str_eq!(
            formatted,
            "\n- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\n"
        );
        let combined = format!("{existing}{formatted}");
        assert_eq!(
            combined,
            "### Dependencies\n\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))\n\n- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\n"
        );
    }

    #[test]
    fn test_get_formatted_changes_crlf_blank_line_zero_trailing_newlines() {
        let existing = "### Dependencies\r\n\r\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))";
        let changes = vec![DependabotChange::new("chrono", "0.4.38", "0.4.39")];
        let entry_pattern = EntryPattern::default();

        let formatted = get_formatted_changes(
            existing,
            &changes,
            &entry_pattern,
            EntrySeparator::DOUBLE_CRLF,
            &example_pr_info(),
        );
        assert_str_eq!(
            formatted,
            "\r\n\r\n- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\r\n"
        );
        let combined = format!("{existing}{formatted}");
        assert_eq!(
            combined,
            "### Dependencies\r\n\r\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))\r\n\r\n- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\r\n"
        );
    }

    #[test]
    fn test_get_formatted_changes_crlf_blank_line_one_trailing_newline() {
        let existing = "### Dependencies\r\n\r\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))\r\n";
        let changes = vec![DependabotChange::new("chrono", "0.4.38", "0.4.39")];
        let entry_pattern = EntryPattern::default();

        let formatted = get_formatted_changes(
            existing,
            &changes,
            &entry_pattern,
            EntrySeparator::DOUBLE_CRLF,
            &example_pr_info(),
        );
        assert_str_eq!(
            formatted,
            "\r\n- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\r\n"
        );
        let combined = format!("{existing}{formatted}");
        assert_eq!(
            combined,
            "### Dependencies\r\n\r\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))\r\n\r\n- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\r\n"
        );
    }

    #[test]
    fn test_get_formatted_changes_sufficient_trailing_newlines() {
        // LF with 2 trailing newlines
        let existing_lf = "### Dependencies\n\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))\n\n";
        let changes = vec![DependabotChange::new("chrono", "0.4.38", "0.4.39")];
        let entry_pattern = EntryPattern::default();

        let formatted_lf = get_formatted_changes(
            existing_lf,
            &changes,
            &entry_pattern,
            EntrySeparator::DOUBLE_LF,
            &example_pr_info(),
        );
        assert_str_eq!(
            formatted_lf,
            "- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\n"
        );
        let combined_lf = format!("{existing_lf}{formatted_lf}");
        assert_eq!(
            combined_lf,
            "### Dependencies\n\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))\n\n- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\n"
        );

        // CRLF with 2 trailing CRLFs
        let existing_crlf = "### Dependencies\r\n\r\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))\r\n\r\n";
        let formatted_crlf = get_formatted_changes(
            existing_crlf,
            &changes,
            &entry_pattern,
            EntrySeparator::DOUBLE_CRLF,
            &example_pr_info(),
        );
        assert_str_eq!(
            formatted_crlf,
            "- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\r\n"
        );
        let combined_crlf = format!("{existing_crlf}{formatted_crlf}");
        assert_eq!(
            combined_crlf,
            "### Dependencies\r\n\r\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))\r\n\r\n- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\r\n"
        );

        // Default "\n" single newline separator should never add extra leading newlines
        let existing_single = "### Dependencies\n\n- `serde`: 1.0.215 → 1.0.216 ([#1](https://github.com/user/repo/pull/1))\n";
        let formatted_single = get_formatted_changes(
            existing_single,
            &changes,
            &entry_pattern,
            EntrySeparator::default(),
            &example_pr_info(),
        );
        assert_str_eq!(
            formatted_single,
            "- `chrono`: 0.4.38 → 0.4.39 ([#1](https://github.com/user/repo/pull/1))\n"
        );
    }

    #[test]
    fn test_parse_body_clap_and_clap_complete_link_matching() {
        let pr_body = r"Bumps the cargo_dependencies group with 2 updates: [clap](https://github.com/clap-rs/clap) and [clap_complete](https://github.com/clap-rs/clap_complete).

Updates `clap` from 4.5.27 to 4.5.28

Updates `clap_complete` from 4.5.42 to 4.5.44
";
        let changes = parse_body(pr_body);
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0].name, "clap");
        assert_eq!(
            changes[0].dep_url(),
            Some("https://github.com/clap-rs/clap")
        );
        assert_eq!(changes[1].name, "clap_complete");
        assert_eq!(
            changes[1].dep_url(),
            Some("https://github.com/clap-rs/clap_complete")
        );
    }

    #[test]
    fn test_parse_body_clap_and_clap_complete_reversed_link_order() {
        let pr_body = r"Bumps the cargo_dependencies group with 2 updates: [clap_complete](https://github.com/clap-rs/clap_complete) and [clap](https://github.com/clap-rs/clap).

Updates `clap` from 4.5.27 to 4.5.28

Updates `clap_complete` from 4.5.42 to 4.5.44
";
        let changes = parse_body(pr_body);
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0].name, "clap");
        assert_eq!(
            changes[0].dep_url(),
            Some("https://github.com/clap-rs/clap")
        );
        assert_eq!(changes[1].name, "clap_complete");
        assert_eq!(
            changes[1].dep_url(),
            Some("https://github.com/clap-rs/clap_complete")
        );
    }

    #[test]
    fn test_parse_body_clap_does_not_match_clap_complete_url() {
        let pr_body = r"Bumps the cargo_dependencies group: [clap_complete](https://github.com/clap-rs/clap_complete).

Updates `clap` from 4.5.27 to 4.5.28

Updates `clap_complete` from 4.5.42 to 4.5.44
";
        let changes = parse_body(pr_body);
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0].name, "clap");
        assert_eq!(changes[0].dep_url(), None);
        assert_eq!(changes[1].name, "clap_complete");
        assert_eq!(
            changes[1].dep_url(),
            Some("https://github.com/clap-rs/clap_complete")
        );
    }

    #[test]
    fn test_parse_body_multi_byte_utf8_boundary_advance() {
        // Label contains an initial rejected occurrence (preceded by an identifier character '_')
        // followed by a valid boundary occurrence.
        // Slicing by byte increment (+1) panics if dep_name starts with a multi-byte UTF-8 character.
        let pr_body = r"Bumps group: [my_🦀crate and 🦀crate](https://example.com/crate).

Updates `🦀crate` from 1.0.0 to 1.1.0
";
        let changes = parse_body(pr_body);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].name, "🦀crate");
        assert_eq!(changes[0].dep_url(), Some("https://example.com/crate"));
    }
}
