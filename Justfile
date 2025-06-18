alias l := lint
alias t := test

@_default:
    just --list

ci: lint test
    cargo fmt --check

lint *ARGS:
    cargo clippy --all

test *ARGS:
    cargo nextest run --all
