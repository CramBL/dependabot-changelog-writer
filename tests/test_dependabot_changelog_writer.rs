use assert_cmd::Command;
use predicates::prelude::*;
use testresult::TestResult;

pub const BIN_NAME: &str = "dependabot-changelog-writer";

#[test]
pub fn test_get_version() -> TestResult {
    let changelog_path = "./test_data/changelogs/ISSUE_51.md";
    let changes_pattern = "[dep]: [old] → [new] ([pr-link])";
    let commit_msg = "Updated changelog with updated dependencies";
    let commit_author = "github-actions[bot]";
    let commit_mail = "github-actions[bot]@users.noreply.github.com";
    let changelog_section = "unreleased";
    let changelog_header3 = "Dependencies";
    let push_changes = "false";

    // TODO: Ensure it's not actually modifying the file.
    // let mut cmd = Command::cargo_bin(BIN_NAME)?;
    // cmd.env("USE_FAKE_EVENT_JSON", "test_data/event_json/issue_51.json")
    //     .args([
    //         changelog_path,
    //         changes_pattern,
    //         commit_msg,
    //         commit_author,
    //         commit_mail,
    //         changelog_section,
    //         changelog_header3,
    //         push_changes,
    //     ]);

    // let out = cmd.output()?;
    // let out = String::from_utf8(out.stdout)?;

    Ok(())
}
