use replay::header::{Error, Result};
use std::convert::TryFrom;

/// The game mode of a game.
#[derive(Debug, PartialEq)]
pub enum GameMode {
    /// The Known mode, including the total missions for completion.
    Known(u8),
    /// The Pick mode, including the required and total missions for completion.
    Pick(u8, u16),
    /// The Any mode, including the required and total missions for completion.
    Any(u8, u16),
}

impl Default for GameMode {
    fn default() -> GameMode {
        GameMode::Known(4)
    }
}

impl TryFrom<u32> for GameMode {
    type Error = Error;

    fn try_from(mode: u32) -> Result<Self> {
        let required = (mode & 0x000000ff) as u8;
        let is_known = mode & 0xff000000 == 0;
        let total = match mode & 0x00ffff00 {
            0x00_00C0_00 => 3,
            0x00_0100_00 => 4,
            0x00_0140_00 => 5,
            0x00_0180_00 => 6,
            0x00_01C0_00 => 7,
            0x00_0200_00 => 8,
            _ => {
                if is_known {
                    // We don't care about total missions for the "Known" mode.
                    0
                } else {
                    bail!(Error::InvalidGameMode(mode));
                }
            }
        } as u16;

        match mode & 0xff000000 {
            0x00_000000 => Ok(GameMode::Known(required)),
            0x10_000000 => Ok(GameMode::Pick(required, total)),
            0x20_000000 => Ok(GameMode::Any(required, total)),
            _ => Err(Error::InvalidGameMode(mode)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn known_game_mode() {
        let result: GameMode = 0x00_0000_04.try_into().unwrap();
        assert_eq!(result, GameMode::Known(4));
    }

    #[test]
    fn any_pick_mode() {
        let result: GameMode = 0x20_01c0_04.try_into().unwrap();
        assert_eq!(result, GameMode::Any(4, 7));
    }

    #[test]
    fn any_game_mode() {
        let result: GameMode = 0x10_0200_04.try_into().unwrap();
        assert_eq!(result, GameMode::Pick(4, 8));
    }

    #[test]
    fn invalid_game_mode() {
        let validated: Result<GameMode> = 0x30_000000.try_into();

        match validated {
            Err(Error::InvalidGameMode(0x30_000000)) => assert!(true),
            _ => assert!(false),
        }
    }
}
