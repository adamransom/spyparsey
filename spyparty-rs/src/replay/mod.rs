mod header;

pub use self::header::Header;
pub use self::header::{GameMode, GameResult, Map, Mission};

use self::header::Result;
use std::io::Read;

#[derive(Default)]
pub struct Replay {
    /// The header of the replay.
    ///
    /// This contains all the information describing the game.
    pub header: Header,
}

impl Replay {
    /// Create a new replay from a reader.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Replay> {
        let header = Header::from_reader(reader)?;

        Ok(Replay { header })
    }

    pub fn has_name(&self, name: &str) -> bool {
        if self.header.spy_user_name == name || self.header.sniper_user_name == name {
            return true;
        }

        if let Some(ref display_name) = self.header.spy_display_name {
            return display_name == name;
        }

        if let Some(ref display_name) = self.header.sniper_display_name {
            return display_name == name;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_name_spy_user_name() {
        let mut replay: Replay = Default::default();
        replay.header.spy_user_name = "test".to_string();

        assert!(replay.has_name("test"));
    }

    #[test]
    fn has_name_spy_display_name() {
        let mut replay: Replay = Default::default();
        replay.header.spy_display_name = Some("test".to_string());

        assert!(replay.has_name("test"));
    }

    #[test]
    fn has_name_sniper_user_name() {
        let mut replay: Replay = Default::default();
        replay.header.sniper_user_name = "test".to_string();

        assert!(replay.has_name("test"));
    }

    #[test]
    fn has_name_sniper_display_name() {
        let mut replay: Replay = Default::default();
        replay.header.sniper_display_name = Some("test".to_string());

        assert!(replay.has_name("test"));
    }
}
