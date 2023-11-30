mod local;
mod remote;

/// Work with the local database of mods
pub use local::LocalDatabase;

/// Query the remote database of mods
pub use remote::RemoteDatabase;

fn fix_version(version: &str) -> &str {
    version.trim().trim_start_matches('v')
}

mod combined_search {
    use crate::mods::local::UnsafeLocalMod;
    use crate::search::Searchable;

    pub struct LocalModWithRemoteName<'a> {
        pub local_mod: &'a UnsafeLocalMod,
        remote_name: Option<String>,
    }

    impl<'a> LocalModWithRemoteName<'a> {
        pub fn new(local_mod: &'a UnsafeLocalMod, remote_name: Option<String>) -> Self {
            Self {
                local_mod,
                remote_name,
            }
        }
    }

    impl Searchable for LocalModWithRemoteName<'_> {
        fn get_values(&self) -> Vec<String> {
            if let Some(name) = &self.remote_name {
                self.local_mod
                    .get_values()
                    .into_iter()
                    .chain(vec![name.clone()])
                    .collect()
            } else {
                self.local_mod.get_values()
            }
        }
    }
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
