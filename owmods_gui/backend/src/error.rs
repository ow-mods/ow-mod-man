use anyhow::anyhow;
use serde::Serialize;

#[derive(Debug)]
pub struct Error(pub anyhow::Error);

pub type Result<T = ()> = std::result::Result<T, Error>;

impl Clone for Error {
    fn clone(&self) -> Self {
        Error(anyhow!("{:?}", self.0))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        self.0.fmt(f)
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error(e)
    }
}

impl From<Error> for anyhow::Error {
    fn from(e: Error) -> Self {
        e.0
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(format!("{:?}", self.0).as_str())
    }
}
