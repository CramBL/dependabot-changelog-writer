pub mod example_changelogs;
pub mod example_dependabot_pr_bodies;
pub mod example_event_json;
pub mod example_parsed_changes;

pub use {
    example_changelogs::*, example_dependabot_pr_bodies::*, example_event_json::*,
    example_parsed_changes::*,
};

pub const EXAMPLE_MARKDOWN_PR_LINK: &str = "[#1](https://github.com/user/repo/pull/1)";
