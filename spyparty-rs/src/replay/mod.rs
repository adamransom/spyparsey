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

    /// Checks if the replay contains a particular player.
    pub fn has_player(&self, name: &str) -> bool {
        self.has_spy(name) || self.has_sniper(name)
    }

    /// Checks if the spy in this replay is a particular player.
    pub fn has_spy(&self, name: &str) -> bool {
        if self.header.spy_user_name == name {
            return true;
        }

        if let Some(ref display_name) = self.header.spy_display_name {
            return display_name == name;
        }

        false
    }

    /// Checks if the sniper in this replay is a particular player.
    pub fn has_sniper(&self, name: &str) -> bool {
        if self.header.sniper_user_name == name {
            return true;
        }

        if let Some(ref display_name) = self.header.sniper_display_name {
            return display_name == name;
        }

        false
    }

    /// Checks if the replay ends in a spy win.
    pub fn is_spy_win(&self) -> bool {
        self.header.result_data.game_result == GameResult::MissionsWin
            || self.header.result_data.game_result == GameResult::CivilianShot
    }

    /// Checks if the replay ends in a sniper win.
    pub fn is_sniper_win(&self) -> bool {
        self.header.result_data.game_result == GameResult::SpyShot
            || self.header.result_data.game_result == GameResult::SpyTimeout
    }

    /// Checks if the replay ends with a win for a particular player.
    pub fn is_win_for(&self, name: &str) -> bool {
        self.has_spy(name) && self.is_spy_win() || self.has_sniper(name) && self.is_sniper_win()
    }

    /// Checks if the replay ends with a loss for a particular player.
    ///
    /// This is not simply the inverse of `is_win_for` because replays can be in an unfinished
    /// state.
    pub fn is_loss_for(&self, name: &str) -> bool {
        self.has_spy(name) && self.is_sniper_win() || self.has_sniper(name) && self.is_spy_win()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_player_spy_user_name() {
        let mut replay: Replay = Default::default();
        replay.header.spy_user_name = "test".to_string();

        assert!(replay.has_player("test"));
    }

    #[test]
    fn has_player_spy_display_name() {
        let mut replay: Replay = Default::default();
        replay.header.spy_display_name = Some("test".to_string());

        assert!(replay.has_player("test"));
    }

    #[test]
    fn has_player_sniper_user_name() {
        let mut replay: Replay = Default::default();
        replay.header.sniper_user_name = "test".to_string();

        assert!(replay.has_player("test"));
    }

    #[test]
    fn has_player_sniper_display_name() {
        let mut replay: Replay = Default::default();
        replay.header.sniper_display_name = Some("test".to_string());

        assert!(replay.has_player("test"));
    }

    #[test]
    fn has_spy_user_name() {
        let mut replay: Replay = Default::default();
        replay.header.spy_user_name = "test".to_string();

        assert!(replay.has_spy("test"));
    }

    #[test]
    fn has_spy_display_name() {
        let mut replay: Replay = Default::default();
        replay.header.spy_display_name = Some("test".to_string());

        assert!(replay.has_spy("test"));
    }

    #[test]
    fn has_spy_not_sniper() {
        let mut replay: Replay = Default::default();
        replay.header.sniper_display_name = Some("test".to_string());

        assert!(!replay.has_spy("test"));
    }

    #[test]
    fn has_sniper_user_name() {
        let mut replay: Replay = Default::default();
        replay.header.sniper_user_name = "test".to_string();

        assert!(replay.has_sniper("test"));
    }

    #[test]
    fn has_sniper_display_name() {
        let mut replay: Replay = Default::default();
        replay.header.sniper_display_name = Some("test".to_string());

        assert!(replay.has_sniper("test"));
    }

    #[test]
    fn has_sniper_not_spy() {
        let mut replay: Replay = Default::default();
        replay.header.spy_display_name = Some("test".to_string());

        assert!(!replay.has_sniper("test"));
    }

    #[test]
    fn is_spy_win() {
        let mut replay: Replay = Default::default();
        replay.header.result_data.game_result = GameResult::MissionsWin;

        assert!(replay.is_spy_win());
    }

    #[test]
    fn is_sniper_win() {
        let mut replay: Replay = Default::default();
        replay.header.result_data.game_result = GameResult::SpyTimeout;

        assert!(replay.is_sniper_win());
    }

    #[test]
    fn is_win_for_spy() {
        let mut replay: Replay = Default::default();
        replay.header.spy_user_name = "test".to_string();
        replay.header.result_data.game_result = GameResult::MissionsWin;

        assert!(replay.is_win_for("test"));
    }

    #[test]
    fn is_win_for_sniper() {
        let mut replay: Replay = Default::default();
        replay.header.sniper_user_name = "test".to_string();
        replay.header.result_data.game_result = GameResult::SpyShot;

        assert!(replay.is_win_for("test"));
    }
}
