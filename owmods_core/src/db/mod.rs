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
    use crate::mods::remote::RemoteMod;
    use crate::search::Searchable;

    pub struct LocalModWithRemoteSearchData<'a> {
        pub local_mod: &'a UnsafeLocalMod,
        remote: Option<RemoteMod>,
    }

    impl<'a> LocalModWithRemoteSearchData<'a> {
        pub fn new(local_mod: &'a UnsafeLocalMod, remote: Option<RemoteMod>) -> Self {
            Self { local_mod, remote }
        }
    }

    impl Searchable for LocalModWithRemoteSearchData<'_> {
        fn get_values(&self) -> Vec<String> {
            if let Some(ref remote) = self.remote {
                self.local_mod
                    .get_values()
                    .into_iter()
                    .chain(remote.get_values())
                    .collect()
            } else {
                self.local_mod.get_values()
            }
        }

        fn break_tie(&self, other: &Self) -> std::cmp::Ordering {
            if let (Some(self_remote), Some(other_remote)) = (&self.remote, &other.remote) {
                self_remote.break_tie(other_remote)
            } else {
                self.remote.is_some().cmp(&other.remote.is_some()).reverse()
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
