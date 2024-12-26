@_default:
    just --list

run \
    $GITHUB_OUTPUT \
    $GH_TOKEN \
    $PUSH_TOKEN \
    $GITHUB_EVENT_PATH \
    CHANGELOG \
    COMMIT_MSG \
    COMMITTER_NAME \
    COMMITTER_EMAIL \
    SECTION_VERSION \
    SECTION_HEADER \
    PUSH_CHANGES:
    cargo run -- "{{CHANGELOG}}" "{{COMMIT_MSG}}" "{{COMMITTER_NAME}}" "{{COMMITTER_EMAIL}}" "{{SECTION_VERSION}}" "{{SECTION_HEADER}}" "{{PUSH_CHANGES}}"

run-fake: (run \
            "test_github_output_path" \
            "test_github_token" \
            "test_push_token" \
            "Cargo.toml" \
            "CHANGELOG.md" \
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