#[macro_use]
pub mod error;

pub use self::error::{Error, Result};

use std::io::Read;
use utils;

/// The header of a replay.
#[derive(Debug, Default)]
pub struct Header {
    pub replay_version: u32,
}

impl Header {
    /// Create a new header from a reader.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Header> {
        let mut header: Header = Default::default();

        header.validate_identifier(reader)?;
        header.set_replay_version(reader)?;

        Ok(header)
    }

    /// Checks the first 4 bytes of the header to make sure the header is valid.
    fn validate_identifier<R: Read>(&self, reader: &mut R) -> Result<()> {
        let mut id = [0; 4];
        reader.read_exact(&mut id)?;

        ensure!(&id == b"RPLY", Error::InvalidIdentifier);

        Ok(())
    }

    /// Read and set the replay version.
    fn set_replay_version<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let version = utils::read_u32(reader)?;

        self.replay_version = version;

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
        let header: Header = Default::default();
        let validated = header.validate_identifier(&mut input);

        assert!(validated.is_ok());
    }

    #[test]
    fn invalid_identifier() {
        let mut input: &[u8] = b"NOPE";
        let header: Header = Default::default();
        let validated = header.validate_identifier(&mut input);

        match validated {
            Err(Error::InvalidIdentifier) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn fail_read_identifier() {
        let mut input: &[u8] = b"RPL";
        let header: Header = Default::default();
        let validated = header.validate_identifier(&mut input);

        match validated {
            Err(Error::Io(_)) => assert!(true),
            _ => assert!(false),
        }
    }
}
