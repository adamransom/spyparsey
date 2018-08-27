use regex::Regex;
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
        // Thanks to LtHummus for tbis trick :)
        let total = ((mode & 0x0fffc00) >> 14) as u16;

        match mode & 0xff000000 {
            0x00_000000 => Ok(GameMode::Known(required)),
            0x10_000000 => Ok(GameMode::Pick(required, total)),
            0x20_000000 => Ok(GameMode::Any(required, total)),
            _ => Err(Error::InvalidGameMode(mode)),
        }
    }
}

impl<'a> TryFrom<&'a str> for GameMode {
    type Error = Error;

    fn try_from(string: &'a str) -> Result<Self> {
        let stripped = string.to_ascii_lowercase().replace(" ", "");
        // Matches 3 forms:
        //    Known 4 of 4
        //    Any 4/8
        //    p3/5
        let re = Regex::new(r"^(?P<mode>\w+)(?P<required>\d)(/|of)(?P<total>\d)$").unwrap();

        if let Some(caps) = re.captures(&stripped) {
            // Make sure all matches are present
            if caps.len() == 5 {
                let required: u8 = caps["required"]
                    .parse()
                    .map_err(|_| Error::UnknownGameMode(string.to_string()))?;
                let total: u16 = caps["total"]
                    .parse()
                    .map_err(|_| Error::UnknownGameMode(string.to_string()))?;

                return Ok(match &caps["mode"] {
                    "any" | "a" => GameMode::Any(required, total),
                    "pick" | "p" => GameMode::Pick(required, total),
                    "known" | "k" => GameMode::Known(required),
                    _ => bail!(Error::UnknownGameMode(string.to_string())),
                });
            }
        }

        Err(Error::UnknownGameMode(string.to_string()))
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

    #[test]
    fn string_into_any_game_mode() {
        let result: GameMode = "a4/8".try_into().unwrap();
        assert_eq!(result, GameMode::Any(4, 8));
    }

    #[test]
    fn string_into_any_game_mode_invalid() {
        let validated: Result<GameMode> = "a4/85".try_into();

        match validated {
            Err(Error::UnknownGameMode(mode)) => assert!(mode == "a4/85"),
            _ => assert!(false),
        }
    }

    #[test]
    fn string_into_pick_game_mode() {
        let result: GameMode = "pick 3 of 7".try_into().unwrap();
        assert_eq!(result, GameMode::Pick(3, 7));
    }

    #[test]
    fn string_into_known_game_mode() {
        let result: GameMode = "pick 5/5".try_into().unwrap();
        assert_eq!(result, GameMode::Pick(5, 5));
    }
}
