// Copyright 2026 dywoq - Apache License 2.0
// https://github.com/dywoq/crate-env

/// Represents a crate version information.
pub struct Version {
    major: Option<String>,
    minor: Option<String>,
    patch: Option<String>,
    pre: Option<String>,
    full: Option<String>,
}

/// Represents a crate information, which copies the environment variables from
/// [this page](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates).
pub struct Information {
    cargo_path: Option<String>,
    manifest_dir: Option<String>,
    manifest_path: Option<String>,
    version: Version,
    authors: Option<Vec<String>>,
    name: Option<String>,
    description: Option<String>,
    homepage: Option<String>,
    repository: Option<String>,
    license: Option<String>,
    license_file: Option<String>,
    rust_version: Option<String>,
    readme: Option<String>,
    crate_name: Option<String>,
    bin_name: Option<String>,
    out_dir: Option<String>,
    primary_package: Option<String>,
    target_tmpdir: Option<String>,
}
