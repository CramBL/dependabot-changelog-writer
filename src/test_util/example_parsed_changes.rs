use crate::dependabot_changes::dependabot_change::DependabotChange;

pub const EXAMPLE_CHANGES: &[DependabotChange<'_>] = &[
    DependabotChange::new("serde", "1.0.215", "1.0.216"),
    DependabotChange::new("chrono", "0.4.38", "0.4.39"),
    DependabotChange::new("semver", "1.0.23", "1.0.24"),
    DependabotChange::new("env_logger", "0.11.5", "0.11.6"),
    DependabotChange::new("zip", "2.2.1", "2.2.2"),
    DependabotChange::new("wasm-bindgen-futures", "0.4.47", "0.4.49"),
    DependabotChange::new("web-sys", "0.3.74", "0.3.76"),
    DependabotChange::new("thiserror", "2.0.4", "2.0.9"),
];

pub const EXAMPLE_CHANGES_SMALL: &[DependabotChange<'_>] = &[
    DependabotChange::new("serde", "1.0.215", "1.0.216"),
    DependabotChange::new("env_logger", "0.11.8", "0.12.0"),
];

pub const EXAMPLE_CHANGES_SMALL_WITH_SHA1: &[DependabotChange<'_>] = &[
    DependabotChange::new("serde", "1.0.215", "1.0.216"),
    DependabotChange::new(
        "docker/login-action",
        "3d58c274f17dffee475a5520cbe67f0a882c4dbb",
        "7ca345011ac4304463197fac0e56eab1bc7e6af0",
    ),
];

pub const CHANGES_ISSUE_51: &[DependabotChange<'_>] = &[
    DependabotChange::new("clap", "4.5.27", "4.5.28"),
    DependabotChange::new("clap_complete", "4.5.42", "4.5.44"),
    DependabotChange::new("mdns-sd", "0.13.1", "0.13.2"),
    DependabotChange::new("strum", "0.26.3", "0.27.0"),
];

/// | Package | From | To |
/// | --- | --- | --- |
/// | [github/codeql-action](https://github.com/github/codeql-action) | `3.28.19` | `3.29.0` |
/// | [docker/setup-buildx-action](https://github.com/docker/setup-buildx-action) | `3.10.0` | `3.11.1` |
/// | [actions/attest-build-provenance](https://github.com/actions/attest-build-provenance) | `2.3.0` | `2.4.0` |
/// | [actions/attest-sbom](https://github.com/actions/attest-sbom) | `2.2.0` | `2.4.0` |
/// | [sigstore/cosign-installer](https://github.com/sigstore/cosign-installer) | `3.8.2` | `3.9.0` |
pub const CHANGES_ISSUE_90: &[DependabotChange<'_>] = &[
    DependabotChange::new("github/codeql-action", "3.28.19", "3.29.0"),
    DependabotChange::new("docker/setup-buildx-action", "3.10.0", "3.11.1"),
    DependabotChange::new("actions/attest-build-provenance", "2.3.0", "2.4.0"),
    DependabotChange::new("actions/attest-sbom", "2.2.0", "2.4.0"),
    DependabotChange::new("sigstore/cosign-installer", "3.8.2", "3.9.0"),
];
