use replay::header::{Error, Result};
use std::io::Read;
use utils;

/// The result of a game.
#[derive(Debug, PartialEq)]
pub enum GameResult {
    MissionsWin,
    SpyTimeout,
    SpyShot,
    CivilianShot,
    InProgress,
}

impl Default for GameResult {
    fn default() -> GameResult {
        GameResult::InProgress
    }
}

#[derive(Debug, Default)]
pub struct ResultData {
    pub version: u32,
    pub simple_rules: bool,
    pub game_result: GameResult,
}

/// The result data contained in the header of a replay.
impl ResultData {
    /// Create a new header from a reader.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<ResultData> {
        let mut result_data: ResultData = Default::default();

        result_data.set_flags(reader)?;
        result_data.set_game_result(reader)?;

        // Skip the rest
        if result_data.version == 1 {
            let mut id = [0; 20];
            reader.read_exact(&mut id)?;
        } else {
            let mut id = [0; 28];
            reader.read_exact(&mut id)?;
        }

        Ok(result_data)
    }

    /// Read and set the result data flags.
    ///
    /// These flags include the version and whether the game was played with simple fules. Currently versions 1 and 2 are supported.
    fn set_flags<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let flags = utils::read_u32(reader)?;

        let version = flags & 0x0f;
        let simple = (flags & 0xf0) == 0x10;

        ensure!(
            version == 1 || version == 2,
            Error::UnsupportedResultVersion(version)
        );

        self.version = version;
        self.simple_rules = simple;

        Ok(())
    }

    /// Read and set the game result.
    fn set_game_result<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let result = utils::read_u32(reader)?;

        self.game_result = match result {
            0 => GameResult::MissionsWin,
            1 => GameResult::SpyTimeout,
            2 => GameResult::SpyShot,
            3 => GameResult::CivilianShot,
            4 => GameResult::InProgress,
            _ => bail!(Error::InvalidGameResult(result)),
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_version_not_simple() {
        let mut input: &[u8] = &[0x01, 0, 0, 0];
        let mut data: ResultData = Default::default();
        data.set_flags(&mut input).unwrap();

        assert_eq!(data.version, 1);
        assert_eq!(data.simple_rules, false);
    }

    #[test]
    fn flags_version_simple() {
        let mut input: &[u8] = &[0x11, 0, 0, 0];
        let mut data: ResultData = Default::default();
        data.set_flags(&mut input).unwrap();

        assert_eq!(data.version, 1);
        assert_eq!(data.simple_rules, true);
    }

    #[test]
    fn unsupported_version() {
        let mut input: &[u8] = &[0x03, 0, 0, 0];
        let mut data: ResultData = Default::default();
        let validated = data.set_flags(&mut input);

        match validated {
            Err(Error::UnsupportedResultVersion(3)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn valid_game_result() {
        let mut input: &[u8] = &[0, 0, 0, 0];
        let mut data: ResultData = Default::default();
        data.set_game_result(&mut input).unwrap();

        assert_eq!(data.game_result, GameResult::MissionsWin);
    }

    #[test]
    fn invalid_game_result() {
        let mut input: &[u8] = &[5, 0, 0, 0];
        let mut data: ResultData = Default::default();
        let validated = data.set_game_result(&mut input);

        match validated {
            Err(Error::InvalidGameResult(5)) => assert!(true),
            _ => assert!(false),
        }
    }
}
