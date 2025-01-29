# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- When `push-changes` was set to false, the would-be changelog diff was printed but the changelog was not actually changed, it is now. 

### Dependencies

- `serde`: 1.0.216 → 1.0.217
- `git2`: 0.19.0 → 0.20.0 ([#43](https://github.com/CramBL/dependabot-changelog-writer/pull/43))
- `tempfile`: 3.14.0 → 3.15.0 ([#43](https://github.com/CramBL/dependabot-changelog-writer/pull/43))
- `log`: 0.4.22 → 0.4.25 ([#47](https://github.com/CramBL/dependabot-changelog-writer/pull/47))
- `serde_json`: 1.0.134 → 1.0.137 ([#47](https://github.com/CramBL/dependabot-changelog-writer/pull/47))
- `similar`: 2.6.0 → 2.7.0 ([#47](https://github.com/CramBL/dependabot-changelog-writer/pull/47))

### Misc

- Fix `dependabot_changelog.yml` used the pre-v1 spelling of `push_token` instead of `push-token`
