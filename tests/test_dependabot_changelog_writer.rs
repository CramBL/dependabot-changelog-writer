use std::process::Output;

use assert_cmd::Command;
use pretty_assertions::assert_str_eq;
use testresult::TestResult;

pub const BIN_NAME: &str = "dependabot-changelog-writer";

fn get_diff(cmd_out: &Output) -> String {
    let stdout = String::from_utf8_lossy(&cmd_out.stdout);
    let start_of_diff = stdout
        .find("%%%%% START OF DIFF %%%%%")
        .expect("Unable to locate start of diff");
    let end_of_diff = stdout
        .find("@@@@@ END OF DIFF @@@@@")
        .expect("Unable to locate end of diff");

    stdout[start_of_diff..end_of_diff].to_owned()
}

#[test]
pub fn test_diff_issue51() -> TestResult {
    let changelog_path = "./test_data/changelogs/ISSUE_51.md";
    let changes_pattern = "[dep]: [old] → [new] ([pr-link])";
    let commit_msg = "Updated changelog with updated dependencies";
    let commit_author = "github-actions[bot]";
    let commit_mail = "github-actions[bot]@users.noreply.github.com";
    let changelog_section = "unreleased";
    let changelog_header3 = "Dependencies";
    let push_changes = "false";
    let dry_run = "dry-run";

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.env("USE_FAKE_EVENT_JSON", "test_data/event_json/ISSUE_51.json")
        .env("RUST_LOG", "debug")
        .args([
            changelog_path,
            changes_pattern,
            commit_msg,
            commit_author,
            commit_mail,
            changelog_section,
            changelog_header3,
            push_changes,
            dry_run,
        ]);

    let out = cmd.output()?;

    let diff = get_diff(&out);
    println!("{diff}");

    let expect_diff_contents = r##"%%%%% START OF DIFF %%%%%
 # Changelog
 
 ## [unreleased]
 
 ### Dependencies
 
-- `clap`: 4.5.26 → 4.5.27 ([#52](https://github.com/CramBL/fidelityfetch/pull/52))
+- `clap`: 4.5.26 → 4.5.28 ([#9](https://github.com/CramBL/dependabot-changelog-writer/pull/1))
+- `clap_complete`: 4.5.42 → 4.5.44 ([#9](https://github.com/CramBL/dependabot-changelog-writer/pull/1))
+- `mdns-sd`: 0.13.1 → 0.13.2 ([#9](https://github.com/CramBL/dependabot-changelog-writer/pull/1))
+- `strum`: 0.26.3 → 0.27.0 ([#9](https://github.com/CramBL/dependabot-changelog-writer/pull/1))"##;

    assert!(diff.starts_with("%%%%% START OF DIFF %%%%%"));
    for (i, (expected, actual)) in expect_diff_contents.lines().zip(diff.lines()).enumerate() {
        println!("Expecting: '{expected}'");
        println!("Actual   : '{actual}'");
        assert_str_eq!(expected, actual, "line #{i} mismatch");
    }
    assert!(out.status.success());

    Ok(())
}

#[test]
pub fn test_diff_ill_advised_section_header() -> TestResult {
    let changelog_path = "./test_data/changelogs/ISSUE_51.md";
    let changes_pattern = "[dep]: [old] → [new] ([pr-link])";
    let commit_msg = "Updated changelog with updated dependencies";
    let commit_author = "github-actions[bot]";
    let commit_mail = "github-actions[bot]@users.noreply.github.com";
    let changelog_section = "unreleased";
    let changelog_header3 = "### Dependencies";
    let push_changes = "false";

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.env("USE_FAKE_EVENT_JSON", "test_data/event_json/ISSUE_51.json")
        .env("RUST_LOG", "debug")
        .args([
            changelog_path,
            changes_pattern,
            commit_msg,
            commit_author,
            commit_mail,
            changelog_section,
            changelog_header3,
            push_changes,
        ]);

    let out = cmd.output()?;
    assert!(!out.status.success());

    let stderr = String::from_utf8_lossy(&out.stderr);

    println!("{stderr}");

    let last_three_lines: String = stderr
        .lines()
        .rev()
        .take(3)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("\n");
    let expected_err = "Error: Invalid section header '### Dependencies', expected a section header such as 'Changes'.
NOTE: section header is assumed to be an h3 header, meaning it implies a prefix of '###' such as '### Changes'
HINT: Try removing the '###' prefix";

    assert_str_eq!(expected_err, last_three_lines);

    Ok(())
}
