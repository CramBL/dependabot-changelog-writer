# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
