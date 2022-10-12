use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ClientError {
    NotInitialized,
    TokenExpired,
    IO(std::io::Error),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "Client is not initialized"),
            Self::TokenExpired => write!(f, "Token expired"),
            Self::IO(e) => write!(f, "IO Error: {}", e),
        }
    }
}

impl From<std::io::Error> for ClientError {
    fn from(e: std::io::Error) -> Self {
        Self::IO(e)
    }
}

impl Error for ClientError {}
