use reqwest;
use serde_json;

pub type RRAWResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    Parse(serde_json::Error),
    RateLimit(f32),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Network(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Parse(e)
    }
}
