use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    SerdeError(serde_json::Error),
    Msg(String),
    Exit,
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Msg(value.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "{e}"),
            Error::SerdeError(e) => write!(f, "{e}"),
            Error::Msg(msg) => write!(f, "{msg}"),
            Error::Exit => write!(f, "exit"),
        }
    }
}
