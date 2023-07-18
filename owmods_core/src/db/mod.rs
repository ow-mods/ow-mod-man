mod local;
mod remote;

/// Work with the local database of mods
pub use local::LocalDatabase;

/// Query the remote database of mods
pub use remote::RemoteDatabase;
