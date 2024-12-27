use crate::dependabot_changes::DependabotChange;

pub const EXAMPLE_CHANGES: &[DependabotChange<'_>] = &[
    DependabotChange::new("`serde`", "1.0.215", "1.0.216"),
    DependabotChange::new("`chrono`", "0.4.38", "0.4.39"),
    DependabotChange::new("`semver`", "1.0.23", "1.0.24"),
    DependabotChange::new("`env_logger`", "0.11.5", "0.11.6"),
    DependabotChange::new("`zip`", "2.2.1", "2.2.2"),
    DependabotChange::new("`wasm-bindgen-futures`", "0.4.47", "0.4.49"),
    DependabotChange::new("`web-sys`", "0.3.74", "0.3.76"),
    DependabotChange::new("`thiserror`", "2.0.4", "2.0.9"),
];

pub const EXAMPLE_CHANGES_SMALL: &[DependabotChange<'_>] = &[
    DependabotChange::new("`serde`", "1.0.215", "1.0.216"),
    DependabotChange::new("`env_logger`", "0.11.8", "0.12.0"),
];
