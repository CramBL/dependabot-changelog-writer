@_default:
    just --list

run \
    $GITHUB_OUTPUT \
    $GH_TOKEN \
    $PUSH_TOKEN \
    $USE_FAKE_EVENT_JSON \
    CHANGELOG \
    CHANGELOG_ENTRY_PATTERN \
    COMMIT_MSG \
    COMMITTER_NAME \
    COMMITTER_EMAIL \
    SECTION_VERSION \
    SECTION_HEADER \
    PUSH_CHANGES:
    cargo run -- "{{CHANGELOG}}" "{{CHANGELOG_ENTRY_PATTERN}}" "{{COMMIT_MSG}}" "{{COMMITTER_NAME}}" "{{COMMITTER_EMAIL}}" "{{SECTION_VERSION}}" "{{SECTION_HEADER}}" "{{PUSH_CHANGES}}"

run-fake: (run \
            "test_github_output_path" \
            "test_github_token" \
            "test_push_token" \
            "test_data/event_json/minimal_pr_opened.json" \
            "CHANGELOG.md" \
            "[dep]: [old] → [new]" \
            "fake commit message" \
            "fake commit name" \
            "fake commit email" \
            "Unreleased" \
            "Dependencies" \
            "FALSE")

build-container:
    docker build -t rust-musl-builder .

build-musl-bin *ARGS="":
    docker run -v $(pwd):/app rust-musl-builder {{ARGS}}