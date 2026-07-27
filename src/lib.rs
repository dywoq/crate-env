// Copyright 2026 dywoq - Apache License 2.0
// https://github.com/dywoq/crate-env

/// Represents the crate version information.
pub struct Version {
    pub major: Option<String>,
    pub minor: Option<String>,
    pub patch: Option<String>,
    pub pre: Option<String>,
    pub full: Option<String>,
}

/// Represents the crate information, which copies the environment variables from
/// [this page](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates).
pub struct Information {
    pub cargo_path: Option<String>,
    pub manifest_dir: Option<String>,
    pub manifest_path: Option<String>,
    pub version: Version,
    pub authors: Option<Vec<String>>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub rust_version: Option<String>,
    pub readme: Option<String>,
    pub crate_name: Option<String>,
    pub bin_name: Option<String>,
    pub out_dir: Option<String>,
    pub primary_package: Option<String>,
    pub target_tmpdir: Option<String>,
}
