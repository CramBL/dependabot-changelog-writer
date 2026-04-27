# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Dependencies

- `actions/upload-artifact`: 4 → 5 ([#105](https://github.com/CramBL/dependabot-changelog-writer/pull/105))
- `actions/download-artifact`: 5 → 6 ([#105](https://github.com/CramBL/dependabot-changelog-writer/pull/105))
- `actions/checkout`: 5 → 6 ([#107](https://github.com/CramBL/dependabot-changelog-writer/pull/107))
- `log`: 0.4.28 → 0.4.29 ([#109](https://github.com/CramBL/dependabot-changelog-writer/pull/109))
- `actions/upload-artifact`: 5 → 6 ([#110](https://github.com/CramBL/dependabot-changelog-writer/pull/110))
- `actions/download-artifact`: 6 → 7 ([#110](https://github.com/CramBL/dependabot-changelog-writer/pull/110))
- `serde_json`: 1.0.145 → 1.0.149 ([#112](https://github.com/CramBL/dependabot-changelog-writer/pull/112))
- `actions/upload-artifact`: 6 → 7 ([#117](https://github.com/CramBL/dependabot-changelog-writer/pull/117))
- `actions/download-artifact`: 7 → 8 ([#117](https://github.com/CramBL/dependabot-changelog-writer/pull/117))
- `tempfile`: 3.22.0 → 3.27.0 ([#118](https://github.com/CramBL/dependabot-changelog-writer/pull/118))
- `env_logger`: 0.11.8 → 0.11.10 ([#119](https://github.com/CramBL/dependabot-changelog-writer/pull/119))
- `similar`: 2.7.0 → 3.1.0 ([#122](https://github.com/CramBL/dependabot-changelog-writer/pull/122))
- `assert_cmd`: 2.0.17 → 2.2.1 ([#123](https://github.com/CramBL/dependabot-changelog-writer/pull/123))
- `test-log`: 0.2.18 → 0.2.20 ([#123](https://github.com/CramBL/dependabot-changelog-writer/pull/123))

## [1.3.1]

### Dependencies

- `cargo update`
- `test-log`: 0.2.17 → 0.2.18 ([#94](https://github.com/CramBL/dependabot-changelog-writer/pull/94))
- `actions/download-artifact`: 4 → 5 ([#97](https://github.com/CramBL/dependabot-changelog-writer/pull/97))
- `actions/checkout`: 4 → 5 ([#98](https://github.com/CramBL/dependabot-changelog-writer/pull/98))
- `serde_json`: 1.0.140 → 1.0.143 ([#99](https://github.com/CramBL/dependabot-changelog-writer/pull/99))
- `tempfile`: 3.20.0 → 3.21.0 ([#99](https://github.com/CramBL/dependabot-changelog-writer/pull/99))

## [1.3.0]

### Added

- The new `duplicate-entry-strategy` lets users configure the behaviour when an unreleased section already contains an entry where the same dependency is updated, see the [README](./README.md) for more on that.

### Fixed

- Fix some potential workflow security issues, this is part of an effort of fixing issues identified through static analysis of GitHub Actions workflow files.
- Fix erroneous changelog entry in 'overwrite' mode if multiple identical entries were present in the unreleased section AND they were listed with semantic versions in code blocks.

## [1.2.0]

### Changed

- `dependabot-changelog-writer` now creates signed commits using the author and email from the input `push-token` when run in CI
- The name of updated dependencies is now sanitized for backticks (\`) enabling more customization for changelog patterns. **NOTE:** The default pattern produces the same changelog entries as they always did, but if you rely on a custom changelog pattern, you will have to change '[dep]' to '\`[dep]\`' to maintain the same pattern
- Make `libgit2` obsolete, cutting binary size in half
- Optimize more, cut binary size in half again

## [1.1.4]

### Fixed

- When `push-changes` was set to false, the would-be changelog diff was printed but the changelog was not actually changed, it is now.

## [1.1.3]

### Fixed

- Fix invalid 'jq' command when action revision is specified as a git SHA

## [1.1.2]

### Changed

- Improved various error messages
- Disallow prefixing the `section-header` input with `###` as it is already implied and leads to unexpected behaviour

## [1.1.1]

### Fixed

- Ensure inputs passed as arguments to the binary are not interpreted by bash before hand, fixes [#75](https://github.com/CramBL/dependabot-changelog-writer/issues/75)

## [1.1.0]

### Changed

- Make it possible to use a git SHA to pin the action (the given SHA has to have an associated tag)

### Fixed

- Avoid accessing github context directly in the script of a workflow step

### Dependencies

- `log`: 0.4.26 → 0.4.27 ([#65](https://github.com/CramBL/dependabot-changelog-writer/pull/65))
- `env_logger`: 0.11.7 → 0.11.8 ([#66](https://github.com/CramBL/dependabot-changelog-writer/pull/66))
- `assert_cmd`: 2.0.16 → 2.0.17 ([#67](https://github.com/CramBL/dependabot-changelog-writer/pull/67))
- `git2`: 0.20.1 → 0.20.2 ([#68](https://github.com/CramBL/dependabot-changelog-writer/pull/68))
- `tempfile`: 3.19.1 → 3.20.0 ([#68](https://github.com/CramBL/dependabot-changelog-writer/pull/68))
- `auth-git2`: 0.5.7 → 0.5.8 ([#70](https://github.com/CramBL/dependabot-changelog-writer/pull/70))

## [1.0.3]

### Changed

- No longer statically link OpenSSL on the linux build. Instead we rely on the OpenSSL lib on the GitHub runner, ensuring continuous upgrade of OpenSSL and vastly reducing binary size.

### Dependencies

- `log`: 0.4.25 → 0.4.26 ([#60](https://github.com/CramBL/dependabot-changelog-writer/pull/60))
- `serde`: 1.0.217 → 1.0.219 ([#61](https://github.com/CramBL/dependabot-changelog-writer/pull/61))
- `serde_json`: 1.0.138 → 1.0.140 ([#61](https://github.com/CramBL/dependabot-changelog-writer/pull/61))
- `env_logger`: 0.11.6 → 0.11.7 ([#62](https://github.com/CramBL/dependabot-changelog-writer/pull/62))
- `git2`: 0.20.0 → 0.20.1 ([#63](https://github.com/CramBL/dependabot-changelog-writer/pull/63))
- `tempfile`: 3.16.0 → 3.19.1 ([#63](https://github.com/CramBL/dependabot-changelog-writer/pull/63))

## [1.0.2]

### Fixed

- [#51](https://github.com/CramBL/dependabot-changelog-writer/issues/51) issue where a previous h3 section with similar dependencies caused invalid position calculations.

### Changed

- If pushing changes fails, try again with force pushing. Resolves issue where dependabot force-pushes mid-workflow.

### Dependencies

- `auth-git2`: 0.5.5 → 0.5.7 ([#54](https://github.com/CramBL/dependabot-changelog-writer/pull/54))
- `serde_json`: 1.0.137 → 1.0.138 ([#54](https://github.com/CramBL/dependabot-changelog-writer/pull/54))
- `tempfile`: 3.15.0 → 3.16.0 ([#54](https://github.com/CramBL/dependabot-changelog-writer/pull/54))
- `OpenSSL`: 3.4.0 → 3.4.1
- `cargo update`

## [1.0.1]

### Changed

- When `push-changes` was set to false, the would-be changelog diff was printed but the changelog was not actually changed, it is now.
- When the actions runs, it now downloads to a uniquely named temporary directory, and cleans it up before the next step.

### Dependencies

- `serde`: 1.0.216 → 1.0.217
- `git2`: 0.19.0 → 0.20.0 ([#43](https://github.com/CramBL/dependabot-changelog-writer/pull/43))
- `tempfile`: 3.14.0 → 3.15.0 ([#43](https://github.com/CramBL/dependabot-changelog-writer/pull/43))
- `log`: 0.4.22 → 0.4.25 ([#47](https://github.com/CramBL/dependabot-changelog-writer/pull/47))
- `serde_json`: 1.0.134 → 1.0.137 ([#47](https://github.com/CramBL/dependabot-changelog-writer/pull/47))
- `similar`: 2.6.0 → 2.7.0 ([#47](https://github.com/CramBL/dependabot-changelog-writer/pull/47))

### Misc

- Fix `dependabot_changelog.yml` used the pre-v1 spelling of `push_token` instead of `push-token`
