#[macro_use]
pub mod error;

pub use self::error::{Error, Result};

use std::io::Read;
use utils;

/// The header of a replay.
#[derive(Debug, Default)]
pub struct Header {
    pub replay_version: u32,
    pub protocol_version: u32,
    pub spyparty_version: u32,
    pub flags: u32,
    pub duration: f32,
    pub game_id: u128,
    pub start_time: u32,
    pub play_id: u16,
    pub spy_user_len: u8,
    pub sniper_user_len: u8,
    pub spy_display_len: u8,
    pub sniper_display_len: u8,
}

impl Header {
    /// Create a new header from a reader.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Header> {
        let mut header: Header = Default::default();

        header.validate_identifier(reader)?;
        header.set_replay_version(reader)?;
        header.set_protocol_version(reader)?;
        header.set_spyparty_version(reader)?;
        header.set_flags(reader)?;
        header.set_duration(reader)?;
        header.set_game_id(reader)?;
        header.set_start_time(reader)?;
        header.set_play_id(reader)?;
        header.set_spy_user_len(reader)?;
        header.set_sniper_user_len(reader)?;
        header.set_spy_display_len(reader)?;
        header.set_sniper_display_len(reader)?;

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

    /// Read and set the protocol version.
    fn set_protocol_version<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let version = utils::read_u32(reader)?;

        self.protocol_version = version;

        Ok(())
    }

    /// Read and set the SpyParty version.
    fn set_spyparty_version<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let version = utils::read_u32(reader)?;

        self.spyparty_version = version;

        Ok(())
    }

    /// Read and set the flags.
    fn set_flags<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let flags = utils::read_u32(reader)?;

        self.flags = flags;

        Ok(())
    }

    /// Read and set the replay duration, in seconds.
    fn set_duration<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let duration = utils::read_f32(reader)?;

        self.duration = duration;

        Ok(())
    }

    /// Read and set the game ID.
    fn set_game_id<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let game_id = utils::read_u128(reader)?;

        self.game_id = game_id;

        Ok(())
    }

    /// Read and set the start time, as a UNIX timestamp.
    fn set_start_time<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let time = utils::read_u32(reader)?;

        self.start_time = time;

        Ok(())
    }

    /// Read and set the play ID.
    fn set_play_id<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let play_id = utils::read_u16(reader)?;

        self.play_id = play_id;

        Ok(())
    }

    /// Read and set the spy's username length.
    fn set_spy_user_len<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let len = utils::read_u8(reader)?;

        self.spy_user_len = len;

        Ok(())
    }

    /// Read and set the sniper's username length.
    fn set_sniper_user_len<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let len = utils::read_u8(reader)?;

        self.sniper_user_len = len;

        Ok(())
    }

    /// Read and set the spy's display name length.
    fn set_spy_display_len<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let len = utils::read_u8(reader)?;

        self.spy_display_len = len;

        Ok(())
    }

    /// Read and set the sniper's display name length.
    fn set_sniper_display_len<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let len = utils::read_u8(reader)?;

        self.sniper_display_len = len;

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
