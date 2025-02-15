macro_rules! dependabot_body {
    ($file:expr) => {
        concat!(
            include_str!(concat!("../../test_data/dependabot_pr_bodies/", $file)),
            include_str!("../../test_data/dependabot_pr_bodies/dependabot_commands_boilerplate.md")
        )
    };
}

pub const DEPENDABOT_BODY_2_ACTIONS_SHA_SEMVER: &str =
    dependabot_body!("2_actions_sha_and_semver.md");
pub const DEPENDABOT_BODY_7_CRATES_SEMVER: &str = dependabot_body!("7_crates_semver.md");
pub const DEPENDABOT_BODY_1_SUBMODULE_SHORT_SHA: &str =
    dependabot_body!("1_submodule_short_sha.md");
pub const DEPENDABOT_BODY_1_DOCKER_NOVEL_VERSION: &str =
    dependabot_body!("1_docker_novel_version.md");

pub const DEPENDABOT_BODY_ISSUE_51: &str = dependabot_body!("ISSUE_51.md");
