use std::fmt;

#[derive(Debug)]
pub enum AocError {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    InvalidArg(String),
    Custom(String),
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::ParseInt(err) => write!(f, "Parse int error: {err}"),
            Self::InvalidArg(arg) => {
                write!(
                    f,
                    "Invalid part argument '{arg}' (expected 1, 2, both, or empty)"
                )
            }
            Self::Custom(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AocError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::ParseInt(err) => Some(err),
            Self::InvalidArg(_) | Self::Custom(_) => None,
        }
    }
}

impl From<std::io::Error> for AocError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<std::num::ParseIntError> for AocError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl From<String> for AocError {
    fn from(msg: String) -> Self {
        Self::Custom(msg)
    }
}

impl From<&str> for AocError {
    fn from(msg: &str) -> Self {
        Self::Custom(msg.to_owned())
    }
}

pub type AocResult<T> = Result<T, AocError>;
