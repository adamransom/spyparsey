use replay::header::{Error, Result};
use std::convert::TryFrom;

/// The game mode of a game.
#[derive(Debug, PartialEq)]
pub enum GameMode {
    Known,
    Pick,
    Any,
}

impl Default for GameMode {
    fn default() -> GameMode {
        GameMode::Known
    }
}

impl TryFrom<u8> for GameMode {
    type Error = Error;

    fn try_from(mode: u8) -> Result<Self> {
        match mode {
            0x00 => Ok(GameMode::Known),
            0x10 => Ok(GameMode::Pick),
            0x20 => Ok(GameMode::Any),
            _ => Err(Error::InvalidGameMode(mode)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn valid_game_mode() {
        let result: GameMode = 0x10.try_into().unwrap();
        assert_eq!(result, GameMode::Pick);
    }

    #[test]
    fn invalid_game_mode() {
        let validated: Result<GameMode> = 0x30.try_into();

        match validated {
            Err(Error::InvalidGameMode(0x30)) => assert!(true),
            _ => assert!(false),
        }
    }
}
