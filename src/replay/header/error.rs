use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

/// The error type for errors that occur whilst reading the header.
#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidIdentifier,
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

/// Trait allowing something to be chained into a Result.
pub trait ResultExt<T> {
    fn chain_err(self) -> Result<T>;
}

/// Allow std::io::Result to be chained into a header Result.
impl<T> ResultExt<T> for std::io::Result<T> {
    fn chain_err(self) -> Result<T> {
        self.map_err(move |e| Error::Io(e))
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
