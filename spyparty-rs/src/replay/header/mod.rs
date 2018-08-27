#[macro_use]
pub mod error;
pub mod result_data;

pub use self::error::{Error, Result};
pub use self::result_data::{GameMode, GameResult, Map, Mission, ResultData};

use std::io::Read;
use utils;

/// The header of a replay.
#[derive(Debug, Default)]
pub struct Header {
    /// The version of the replay.
    ///
    /// Currently only versions 3, 4 and 5 are supported.
    pub replay_version: u32,
    /// The verion of the protocol.
    pub protocol_version: u32,
    /// The version of the SpyParty build that created the replay.
    pub spyparty_version: u32,
    /// Some kind of flags.
    ///
    /// This is currently unused/unknown.
    pub flags: u32,
    /// The duration of the replay in seconds.
    pub duration: f32,
    /// The unique ID for this game.
    pub game_id: u128,
    /// The time this game started, as a UNIX timestamp.
    pub start_time: u32,
    /// The ID of this particular game in relation to an entire match.
    pub play_id: u16,
    /// The length of the spy's username.
    pub spy_user_len: u8,
    /// The length of the sniper's username.
    pub sniper_user_len: u8,
    /// The length of the spy's display name.
    ///
    /// This was introduced to distinguish steam ID "names" from display names.
    pub spy_display_len: u8,
    /// The length of the sniper's display name.
    ///
    /// This was introduced to distinguish steam ID "names" from display names.
    pub sniper_display_len: u8,
    /// Data relating to the result of the game.
    pub result_data: ResultData,
    /// The client latency when the game was played.
    pub latency: f32,
    /// The size of the packet data that comes after the names.
    pub data_size: u32,
    /// The spy's username.
    pub spy_user_name: String,
    /// The sniper's username.
    pub sniper_user_name: String,
    /// The spy's display name.
    ///
    /// This was introduced to distinguish steam ID "names" from display names.
    pub spy_display_name: Option<String>,
    /// The sniper's display name.
    ///
    /// This was introduced to distinguish steam ID "names" from display names.
    pub sniper_display_name: Option<String>,
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

        if header.replay_version == 5 {
            header.set_spy_display_len(reader)?;
            header.set_sniper_display_len(reader)?;
            header.skip_unused(reader)?;
        }

        header.set_result_data(reader)?;
        header.set_latency(reader)?;
        header.set_data_size(reader)?;
        header.set_spy_user_name(reader)?;
        header.set_sniper_user_name(reader)?;
        header.set_spy_display_name(reader)?;
        header.set_sniper_display_name(reader)?;

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
    ///
    /// Currently versions 3, 4 and 5 are supported.
    fn set_replay_version<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let version = utils::read_u32(reader)?;

        ensure!(
            version == 3 || version == 4 || version == 5,
            Error::UnsupportedReplayVersion(version)
        );

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

        if len > 0 {
            self.spy_user_len = len;

            Ok(())
        } else {
            Err(Error::MissingSpyUsername)
        }
    }

    /// Read and set the sniper's username length.
    fn set_sniper_user_len<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let len = utils::read_u8(reader)?;

        if len > 0 {
            self.sniper_user_len = len;

            Ok(())
        } else {
            Err(Error::MissingSniperUsername)
        }
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

    /// Skip an unused part of the header.
    fn skip_unused<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let mut id = [0; 2];
        reader.read_exact(&mut id)?;

        Ok(())
    }

    /// Read and set the result data.
    fn set_result_data<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        self.result_data = ResultData::from_reader(reader, self.replay_version)?;

        Ok(())
    }

    /// Read and set the client latency.
    ///
    /// Note: This always seems to be 0.75 in my tests.
    fn set_latency<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let latency = utils::read_f32(reader)?;

        self.latency = latency;

        Ok(())
    }

    /// Read and set the data size (remaining data after the names).
    fn set_data_size<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let size = utils::read_u32(reader)?;

        self.data_size = size;

        Ok(())
    }

    /// Read and set the spy's username.
    ///
    /// This assumes the name is a valid UTF8 string (which according to checker, it should be).
    fn set_spy_user_name<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let mut buf = vec![0u8; self.spy_user_len as usize];
        reader.read_exact(&mut buf)?;

        self.spy_user_name = String::from_utf8(buf)?;

        Ok(())
    }

    /// Read and set the sniper's username.
    ///
    /// This assumes the name is a valid UTF8 string (which according to checker, it should be).
    fn set_sniper_user_name<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let mut buf = vec![0u8; self.sniper_user_len as usize];
        reader.read_exact(&mut buf)?;

        self.sniper_user_name = String::from_utf8(buf)?;

        Ok(())
    }

    /// Read and set the spy's display name.
    ///
    /// This assumes the name is a valid UTF8 string (which according to checker, it should be).
    fn set_spy_display_name<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        if self.spy_display_len > 0 {
            let mut buf = vec![0u8; self.spy_display_len as usize];
            reader.read_exact(&mut buf)?;

            self.spy_display_name = Some(String::from_utf8(buf)?);
        } else {
            self.spy_display_name = None;
        }

        Ok(())
    }

    /// Read and set the sniper's display name.
    ///
    /// This assumes the name is a valid UTF8 string (which according to checker, it should be).
    fn set_sniper_display_name<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        if self.sniper_display_len > 0 {
            let mut buf = vec![0u8; self.sniper_display_len as usize];
            reader.read_exact(&mut buf)?;

            self.sniper_display_name = Some(String::from_utf8(buf)?);
        } else {
            self.sniper_display_name = None;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn unsupported_version() {
        let mut input: &[u8] = &[2, 0, 0, 0];
        let mut header: Header = Default::default();
        let validated = header.set_replay_version(&mut input);

        match validated {
            Err(Error::UnsupportedReplayVersion(2)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn missing_spy_user_name() {
        let mut input: &[u8] = &[0];
        let mut header: Header = Default::default();
        let validated = header.set_spy_user_len(&mut input);

        match validated {
            Err(Error::MissingSpyUsername) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn missing_sniper_user_name() {
        let mut input: &[u8] = &[0];
        let mut header: Header = Default::default();
        let validated = header.set_sniper_user_len(&mut input);

        match validated {
            Err(Error::MissingSniperUsername) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn valid_spy_user_name() {
        let mut input: &[u8] = b"adam";
        let mut header: Header = Default::default();
        header.spy_user_len = input.len() as u8;
        header.set_spy_user_name(&mut input).unwrap();

        assert_eq!(header.spy_user_name, "adam");
    }

    #[test]
    fn valid_sniper_user_name() {
        let mut input: &[u8] = b"adam";
        let mut header: Header = Default::default();
        header.sniper_user_len = input.len() as u8;
        header.set_sniper_user_name(&mut input).unwrap();

        assert_eq!(header.sniper_user_name, "adam");
    }

    #[test]
    fn invalid_spy_user_name() {
        let mut input: &[u8] = b"Hello \xF0\x90\x80World";
        let mut header: Header = Default::default();
        header.spy_user_len = input.len() as u8;
        let validated = header.set_spy_user_name(&mut input);

        match validated {
            Err(Error::InvalidString(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn invalid_sniper_user_name() {
        let mut input: &[u8] = b"Hello \xF0\x90\x80World";
        let mut header: Header = Default::default();
        header.sniper_user_len = input.len() as u8;
        let validated = header.set_sniper_user_name(&mut input);

        match validated {
            Err(Error::InvalidString(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn has_spy_display_name() {
        let mut input: &[u8] = b"adam";
        let mut header: Header = Default::default();
        header.spy_display_len = 4;
        header.set_spy_display_name(&mut input).unwrap();

        assert_eq!(header.spy_display_name, Some("adam".to_string()));
    }

    #[test]
    fn no_spy_display_name() {
        let mut input: &[u8] = &[];
        let mut header: Header = Default::default();
        header.spy_display_len = 0;
        header.set_spy_display_name(&mut input).unwrap();

        assert_eq!(header.spy_display_name, None);
    }

    #[test]
    fn invalid_spy_display_name() {
        let mut input: &[u8] = b"Hello \xF0\x90\x80World";
        let mut header: Header = Default::default();
        header.spy_display_len = input.len() as u8;
        let validated = header.set_spy_display_name(&mut input);

        match validated {
            Err(Error::InvalidString(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn has_sniper_display_name() {
        let mut input: &[u8] = b"adam";
        let mut header: Header = Default::default();
        header.sniper_display_len = 4;
        header.set_sniper_display_name(&mut input).unwrap();

        assert_eq!(header.sniper_display_name, Some("adam".to_string()));
    }

    #[test]
    fn no_sniper_display_name() {
        let mut input: &[u8] = &[];
        let mut header: Header = Default::default();
        header.sniper_display_len = 0;
        header.set_sniper_display_name(&mut input).unwrap();

        assert_eq!(header.sniper_display_name, None);
    }

    #[test]
    fn invalid_sniper_display_name() {
        let mut input: &[u8] = b"Hello \xF0\x90\x80World";
        let mut header: Header = Default::default();
        header.sniper_display_len = input.len() as u8;
        let validated = header.set_sniper_display_name(&mut input);

        match validated {
            Err(Error::InvalidString(_)) => assert!(true),
            _ => assert!(false),
        }
    }
}
