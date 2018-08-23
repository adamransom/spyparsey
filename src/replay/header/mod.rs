#[macro_use]
pub mod error;

pub use self::error::{Error, Result};

use self::error::ResultExt;
use std::io::Read;

/// The header of a replay.
#[derive(Debug)]
pub struct Header {}

impl Header {
    /// Create a new header from a reader.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Header> {
        let header = Header {};

        header.validate_identifier(reader)?;

        Ok(header)
    }

    /// Checks the first 4 bytes of the header to make sure the header is valid.
    fn validate_identifier<R: Read>(&self, reader: &mut R) -> Result<()> {
        let mut id = [0; 4];
        reader.read_exact(&mut id).chain_err()?;

        ensure!(&id == b"RPLY", Error::InvalidIdentifier);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Error;
    use super::Header;

    #[test]
    fn valid_identifier() {
        let mut input: &[u8] = b"RPLY";
        let header = Header::from_reader(&mut input);

        assert!(header.is_ok());
    }

    #[test]
    fn invalid_identifier() {
        let mut input: &[u8] = b"NOPE";
        let header = Header::from_reader(&mut input);

        match header {
            Err(Error::InvalidIdentifier) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn fail_read_identifier() {
        let mut input: &[u8] = b"NOP";
        let header = Header::from_reader(&mut input);

        match header {
            Err(Error::Io(_)) => assert!(true),
            _ => assert!(false),
        }
    }
}
