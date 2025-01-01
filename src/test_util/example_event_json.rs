/// Minimal event JSON from an opened PR with the body from a dependabot update
/// contains only the fields we use:
/// - ["number"]
/// - ["pull_request"]["head"]["ref"]
/// - ["pull_request"]["body"]
/// - ["pull_request"]["html_url"]
pub const EXAMPLE_PR_OPENED_EVENT_JSON: &str =
    include_str!("../../test_data/event_json/minimal_pr_opened.json");
