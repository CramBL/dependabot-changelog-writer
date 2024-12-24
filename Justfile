@_default:
    just --list

run \
    $GITHUB_OUTPUT \
    $GITHUB_TOKEN \
    $GITHUB_EVENT_PATH \
    CHANGELOG \
    COMMIT_MSG \
    COMMITTER_NAME \
    COMMITTER_EMAIL \
    SECTION_VERSION \
    SECTION_HEADER:
    cargo run -- "{{CHANGELOG}}" "{{COMMIT_MSG}}" "{{COMMITTER_NAME}}" "{{COMMITTER_EMAIL}}" "{{SECTION_VERSION}}" "{{SECTION_HEADER}}"
run-fake: (run \
            "test_github_output_path" \
            "test_github_token" \
            "Cargo.toml" \
            "CHANGELOG.md" \
            "fake commit message" \
            "fake commit name" \
            "fake commit email" \
            "Unreleased" \
            "Dependencies")