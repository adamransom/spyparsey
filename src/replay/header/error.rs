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
    UnsupportedReplayVersion(u32),
    UnsupportedResultVersion(u32),
    MissingSpyUsername,
    MissingSniperUsername,
    InvalidString(std::string::FromUtf8Error),
    InvalidGameResult(u32),
    InvalidTotalMissions(u16),
    InvalidGameMode(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO error ({})", err),
            Error::InvalidIdentifier => write!(f, "invalid identifier"),
            Error::UnsupportedReplayVersion(v) => write!(f, "unsupported replay version ({})", v),
            Error::UnsupportedResultVersion(v) => {
                write!(f, "unsupported result data version ({})", v)
            }
            Error::MissingSpyUsername => write!(f, "missing spy username"),
            Error::MissingSniperUsername => write!(f, "missing sniper username"),
            Error::InvalidString(err) => write!(f, "invalid UTF8 string ({})", err),
            Error::InvalidGameResult(result) => write!(f, "invalid game result ({})", result),
            Error::InvalidTotalMissions(total) => write!(f, "invalid total missions ({})", total),
            Error::InvalidGameMode(mode) => write!(f, "invalid game mode ({})", mode),
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

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::InvalidString(err)
    }
}

/// Simple macro to allow returning early with an error.
#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return Err($e);
    };
}

/// Simple macro to allow returning early with an error if a condition isn't satisfied.
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            bail!($e);
        }
    };
}
