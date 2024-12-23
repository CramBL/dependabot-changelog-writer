@_default:
    just --list

run CHANGELOG COMMIT_MSG $GITHUB_OUTPUT $GITHUB_TOKEN $GITHUB_EVENT_PATH:
    cargo run -- "{{CHANGELOG}}" "{{COMMIT_MSG}}"

run-fake: (run "CHANGELOG.md" "fake commit" "test_github_output_path" "test_github_token" "test_github_event_path")