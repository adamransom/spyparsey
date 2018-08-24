use replay::header::{Error, Result};
use std::convert::TryFrom;

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

impl TryFrom<u32> for GameResult {
    type Error = Error;

    fn try_from(result: u32) -> Result<Self> {
        match result {
            0 => Ok(GameResult::MissionsWin),
            1 => Ok(GameResult::SpyTimeout),
            2 => Ok(GameResult::SpyShot),
            3 => Ok(GameResult::CivilianShot),
            4 => Ok(GameResult::InProgress),
            _ => Err(Error::InvalidGameResult(result)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn valid_game_result() {
        let result: GameResult = 0u32.try_into().unwrap();
        assert_eq!(result, GameResult::MissionsWin);
    }

    #[test]
    fn invalid_game_result() {
        let validated: Result<GameResult> = 5u32.try_into();

        match validated {
            Err(Error::InvalidGameResult(5)) => assert!(true),
            _ => assert!(false),
        }
    }
}
