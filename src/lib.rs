// Copyright 2026 dywoq - Apache License 2.0
// https://github.com/dywoq/crate-env

/// Represents the crate version information.
pub struct Version {
    pub major: Option<&'static str>,
    pub minor: Option<&'static str>,
    pub patch: Option<&'static str>,
    pub pre: Option<&'static str>,
    pub full: Option<&'static str>,
}

impl Default for Version {
    fn default() -> Self {
        Version {
            major: None,
            minor: None,
            patch: None,
            pre: None,
            full: None,
        }
    }
}

/// Represents the crate package information.
pub struct Package {
    pub authors: Option<&'static str>,
    pub name: Option<&'static str>,
    pub description: Option<&'static str>,
    pub homepage: Option<&'static str>,
    pub repository: Option<&'static str>,
    pub license: Option<&'static str>,
    pub license_file: Option<&'static str>,
    pub rust_version: Option<&'static str>,
    pub readme: Option<&'static str>,
}

impl Default for Package {
    fn default() -> Self {
        Package {
            authors: None,
            name: None,
            description: None,
            homepage: None,
            repository: None,
            license: None,
            license_file: None,
            rust_version: None,
            readme: None,
        }
    }
}

/// Represents the crate information, which copies the environment variables structure from
/// [this page](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates).
pub struct Information {
    pub cargo_path: Option<&'static str>,
    pub manifest_dir: Option<&'static str>,
    pub manifest_path: Option<&'static str>,
    pub version: Version,
    pub package: Package,
    pub crate_name: Option<&'static str>,
    pub bin_name: Option<&'static str>,
    pub out_dir: Option<&'static str>,
    pub primary_package: Option<&'static str>,
    pub target_tmpdir: Option<&'static str>,
}

impl Default for Information {
    fn default() -> Self {
        Information {
            cargo_path: None,
            manifest_dir: None,
            manifest_path: None,
            version: Default::default(),
            package: Default::default(),
            crate_name: None,
            bin_name: None,
            out_dir: None,
            primary_package: None,
            target_tmpdir: None,
        }
    }
}

/// Captures the crate information at compile-time, storing the values into an [`Information`] instance
/// and returns it.
///
/// # Recommendation
///
/// Due to the heavy operations (such as allocating the strings within the static storage),
/// It's strongly recommended to use this macro once to avoid binary bloat.
///
/// # Example
///
/// ```
/// use crate_env::{crate_capture_information, Information};
/// let information = crate_capture_information!();
/// println!("Package name: {}, Crate name: {}", information.package.name.unwrap(), information.crate_name.unwrap());
/// ```
#[macro_export]
macro_rules! crate_capture_information {
    () => {
        Information {
            cargo_path: option_env!("CARGO_PATH"),
            manifest_dir: option_env!("CARGO_MANIFEST_DIR"),
            manifest_path: option_env!("CARGO_MANIFEST_PATH"),

            version: Version {
                major: option_env!("CARGO_PKG_VERSION_MAJOR"),
                minor: option_env!("CARGO_PKG_VERSION_MINOR"),
                patch: option_env!("CARGO_PKG_VERSION_PATCH"),
                pre: option_env!("CARGO_PKG_VERSION_PRE"),
                full: option_env!("CARGO_PKG_VERSION"),
            },

            package: Package {
                authors: option_env!("CARGO_PKG_AUTHORS"),
                name: option_env!("CARGO_PKG_NAME"),
                description: option_env!("CARGO_PKG_DESCRIPTION"),
                homepage: option_env!("CARGO_PKG_HOMEPAGE"),
                repository: option_env!("CARGO_PKG_REPOSITORY"),
                license: option_env!("CARGO_PKG_LICENSE"),
                license_file: option_env!("CARGO_PKG_LICENSE_FILE"),
                rust_version: option_env!("CARGO_PKG_RUST_VERSION"),
                readme: option_env!("CARGO_PKG_README"),
            },

            crate_name: option_env!("CARGO_CRATE_NAME"),
            bin_name: option_env!("CARGO_BIN_NAME"),
            out_dir: option_env!("OUT_DIR"),
            primary_package: option_env!("CARGO_PRIMARY_PACKAGE"),
            target_tmpdir: option_env!("CARGO_TARGET_TMPDIR"),
        }
    };
}
