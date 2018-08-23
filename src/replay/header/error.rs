use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

/// The error type for errors that occur whilst reading the header.
///
/// # Notes
///
/// I've opted for specific error messages related to the format of the header so, for example, you
/// would be able to tell the parsing failed because of an unuspported version. However, for IO
/// errors there is just a single error type and it won't be clear which part of the file caused it
/// (because usually it won't matter).
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidIdentifier,
    UnsupportedVersion(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error ({})", err),
            _ => {
                let msg = match self {
                    Error::InvalidIdentifier => "invalid identifier",
                    _ => "unknown error",
                };

                write!(f, "{}", msg)
            }
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

/// Simple macro to allow returning early with an error if a condition isn't satisfied.
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            return Err($e);
        }
    };
}
