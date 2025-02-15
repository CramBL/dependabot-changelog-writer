# Changelog

## [unreleased]

### Dependencies

- `clap`: 4.5.26 → 4.5.27 ([#52](https://github.com/CramBL/fidelityfetch/pull/52))

## [0.4.2]

### Dependencies

- `mdns-sd`: 0.13.0 → 0.13.1
- `crambl/dependabot-changelog-writer`: 0.4.0 → 0.5.0
- `crate-ci/typos`: 1.28.3 → 1.29.4 ([#47](https://github.com/CramBL/fidelityfetch/pull/47))
- `axum`: 0.7.9 → 0.8.1 ([#48](https://github.com/CramBL/fidelityfetch/pull/48))
- `clap`: 4.5.23 → 4.5.26 ([#49](https://github.com/CramBL/fidelityfetch/pull/49))
- `clap_complete`: 4.5.38 → 4.5.42 ([#49](https://github.com/CramBL/fidelityfetch/pull/49))
- `tokio`: 1.42.0 → 1.43.0 ([#49](https://github.com/CramBL/fidelityfetch/pull/49))
- `thiserror`: 2.0.7 → 2.0.11 ([#49](https://github.com/CramBL/fidelityfetch/pull/49))

## [0.4.1]

### Changed

- Reduce response size by trimming white space
- Make the file list more compact

## [0.4.0]

### Added

- Allow logging to journald explicitly
- Allow setting logging destination to either `journald` (unix only), `stderr`, or `stdout`.

### Fix

- Main process now exits with a non-zero exit when failing to bind to a port.

### Misc.

- Update dependencies

## [0.3.3]

### Changed

- Update dependencies

### Internals

- Refactors
- More tests

## [0.3.2]

### Added

- npm publish job

## [0.3.1]

### Changed

- Update dependencies to get some critical fixes to some async dependencies.

## [0.3.0]

### Added

- Recognize and add icons to many more filetypes
- Better error messages

## [0.2.0]

### Added

- Completions for bash, zsh, fish, powershell, elvish.
- Favicon.ico
- Various tweaks

## [0.1.1]

### Fix

- Paths with spaces and non-English letters not being recognized
