#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO Error:{0}")]
    IO(String),
}

impl From<rmp_serde::decode::Error> for Error {
    fn from(e: rmp_serde::decode::Error) -> Self {
        Error::IO(e.to_string())
    }
}

impl From<rmp_serde::encode::Error> for Error {
    fn from(e: rmp_serde::encode::Error) -> Self {
        Error::IO(e.to_string())
    }
}
