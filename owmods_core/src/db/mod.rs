mod local;
mod remote;

/// Work with the local database of mods
pub use local::LocalDatabase;

/// Query the remote database of mods
pub use remote::RemoteDatabase;

fn fix_version(version: &str) -> &str {
    version.trim().trim_start_matches('v')
}

#[cfg(test)]
mod tests {

    use super::fix_version;

    #[test]
    fn test_fix_version() {
        assert_eq!(fix_version("v0.1.0"), "0.1.0");
        assert_eq!(fix_version("vvvvv0.1.0"), "0.1.0");
        assert_eq!(fix_version("0.1.0"), "0.1.0");
        assert_eq!(fix_version("asdf"), "asdf");
    }

    #[test]
    fn test_fix_version_whitespace() {
        assert_eq!(fix_version(" v1.0.0 "), "1.0.0");
        assert_eq!(fix_version(" v1.0.0"), "1.0.0");
        assert_eq!(fix_version("v1.0.0 "), "1.0.0");
    }
}
