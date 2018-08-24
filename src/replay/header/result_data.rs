use replay::header::{Error, Result};
use std::fmt;
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

/// The maps of SpyParty.
#[derive(Debug, PartialEq)]
pub enum Map {
    Balcony,
    Ballroom,
    Courtyard,
    Gallery,
    HighRise,
    Library,
    Moderne,
    Pub,
    Terrace,
    Veranda,
    Unknown(u32),
}

impl Default for Map {
    fn default() -> Map {
        Map::Unknown(0)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Map::Balcony => "Balcony",
                Map::Ballroom => "Ballroom",
                Map::Courtyard => "Courtyard",
                Map::Gallery => "Gallery",
                Map::HighRise => "High-Rise",
                Map::Library => "Library",
                Map::Moderne => "Moderne",
                Map::Pub => "Pub",
                Map::Terrace => "Terrace",
                Map::Veranda => "Veranda",
                Map::Unknown(_) => "Unknown",
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Mission {
    BugAmbassador,
    ContactDoubleAgent,
    FingerprintAmbassador,
    InspectStatues,
    PurloinGuestList,
    SeduceTarget,
    SwapStatue,
    TransferMicrofilm,
}

impl fmt::Display for Mission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mission::BugAmbassador => "Bug Ambassador",
                Mission::ContactDoubleAgent => "Contact Double Agent",
                Mission::FingerprintAmbassador => "Fingerprint Ambassador",
                Mission::InspectStatues => "Inspect Statues",
                Mission::PurloinGuestList => "Purloin Guest List",
                Mission::SeduceTarget => "Seduce Target",
                Mission::SwapStatue => "Swap Statue",
                Mission::TransferMicrofilm => "Transfer Microfilm",
            }
        )
    }
}

#[derive(Debug, Default)]
pub struct ResultData {
    /// The version of the result data.
    ///
    /// Currently only versions 1 and 2 are supported.
    pub version: u32,
    /// Whether or not this game was played with simple rules.
    pub simple_rules: bool,
    /// The result of the game.
    pub game_result: GameResult,
    /// The number of mission completed required to win.
    ///
    /// This is the X in "Any X of Y".
    pub missions_required: u8,
    /// The total number of missions available to complete.
    ///
    /// This is the Y in "Any X of Y".
    pub total_missions: u16,
    /// The mode of the game.
    pub game_mode: GameMode,
    /// The map the game was played on.
    pub map: Map,
    /// The missions that the spy selected.
    pub selected_missions: Vec<Mission>,
    /// The missions that the spy picked (for "Pick" game mode).
    pub picked_missions: Vec<Mission>,
    /// The missions the spy completed.
    pub completed_missions: Vec<Mission>,
    /// The number of guests at the party.
    ///
    /// This is optional because it's only available from version 2 onwards.
    pub guests: Option<u32>,
    /// The time on the clock at the start of the game in seconds.
    ///
    /// This is optional because it's only available from version 2 onwards.
    pub clock_start: Option<u32>,
}

/// The result data contained in the header of a replay.
impl ResultData {
    /// Create a new header from a reader.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<ResultData> {
        let mut result_data: ResultData = Default::default();

        result_data.set_flags(reader)?;
        result_data.set_game_result(reader)?;
        result_data.set_missions_required(reader)?;
        result_data.set_total_missions(reader)?;
        result_data.set_game_mode(reader)?;
        result_data.set_map(reader)?;
        result_data.set_selected_missions(reader)?;
        result_data.set_picked_missions(reader)?;
        result_data.set_completed_missions(reader)?;

        // Skip the rest
        if result_data.version == 2 {
            result_data.set_guests(reader)?;
            result_data.set_clock_start(reader)?;
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

    /// Read and set the number of mission completes required to win.
    fn set_missions_required<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let num = utils::read_u8(reader)?;

        self.missions_required = num;

        Ok(())
    }

    /// Read and set the total number of missions available to complete.
    fn set_total_missions<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let total = utils::read_u16(reader)?;

        self.total_missions = match total {
            0x00C0 => 3,
            0x0100 => 4,
            0x0140 => 5,
            0x0180 => 6,
            0x01C0 => 7,
            0x0200 => 8,
            _ => {
                // If we are using simple rules, its always "Known 4 of 4" so the total missions is
                // the same as the required missions.
                if self.simple_rules {
                    self.missions_required as u16
                } else {
                    bail!(Error::InvalidTotalMissions(total));
                }
            }
        };

        Ok(())
    }

    /// Read and set the game mode.
    fn set_game_mode<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let mode = utils::read_u8(reader)?;

        self.game_mode = match mode {
            0x00 => GameMode::Known,
            0x10 => GameMode::Pick,
            0x20 => GameMode::Any,
            _ => bail!(Error::InvalidGameMode(mode)),
        };

        Ok(())
    }

    /// Read and set the map.
    ///
    /// This is a bit lenient and allows invalid maps, simply because there are a few unknown
    /// hashes floating around and I don't have all the replays available to test all the cases.
    ///
    /// Note: Currently we only handle new art maps properly.
    fn set_map<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let map = utils::read_u32(reader)?;

        self.map = match map {
            0x1dbd8e41 => Map::Balcony,
            0x5b121925 => Map::Ballroom,
            0x9dc5bb5e => Map::Courtyard,
            0x7173b8bf => Map::Gallery,
            0x1a56c5a1 => Map::HighRise,
            0x168f4f62 => Map::Library,
            0x2e37f15b => Map::Moderne,
            0x3b85fff3 => Map::Pub,
            0x9032ce22 => Map::Terrace,
            0x6f81a558 => Map::Veranda,
            _ => Map::Unknown(map),
        };

        Ok(())
    }

    /// Read and set the selected missions.
    fn set_selected_missions<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let missions = utils::read_u32(reader)?;

        self.selected_missions = unpack_missions(missions);

        Ok(())
    }

    /// Read and set the picked missions.
    fn set_picked_missions<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let missions = utils::read_u32(reader)?;

        self.picked_missions = unpack_missions(missions);

        Ok(())
    }

    /// Read and set the completed missions.
    fn set_completed_missions<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let missions = utils::read_u32(reader)?;

        self.completed_missions = unpack_missions(missions);

        Ok(())
    }

    /// Read and set the number of guests
    fn set_guests<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let guests = utils::read_u32(reader)?;

        self.guests = Some(guests);

        Ok(())
    }

    /// Read and set the clock start in seconds.
    fn set_clock_start<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let seconds = utils::read_u32(reader)?;

        self.clock_start = Some(seconds);

        Ok(())
    }
}

// Unpacks a bitfield of missions into a vector.
fn unpack_missions(data: u32) -> Vec<Mission> {
    let mut missions: Vec<Mission> = Vec::new();

    if data & (1 << 0) != 0 {
        missions.push(Mission::BugAmbassador);
    }

    if data & (1 << 1) != 0 {
        missions.push(Mission::ContactDoubleAgent);
    }

    if data & (1 << 2) != 0 {
        missions.push(Mission::TransferMicrofilm);
    }

    if data & (1 << 3) != 0 {
        missions.push(Mission::SwapStatue);
    }

    if data & (1 << 4) != 0 {
        missions.push(Mission::InspectStatues);
    }

    if data & (1 << 5) != 0 {
        missions.push(Mission::SeduceTarget);
    }

    if data & (1 << 6) != 0 {
        missions.push(Mission::PurloinGuestList);
    }

    if data & (1 << 7) != 0 {
        missions.push(Mission::FingerprintAmbassador);
    }

    missions
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

    #[test]
    fn valid_total_missions_not_simple() {
        let mut input: &[u8] = &[0x40, 0x01];
        let mut data: ResultData = Default::default();
        data.set_total_missions(&mut input).unwrap();

        assert_eq!(data.total_missions, 5);
    }

    #[test]
    fn valid_total_missions_simple() {
        let mut input: &[u8] = &[0, 0];
        let mut data: ResultData = Default::default();
        data.simple_rules = true;
        data.missions_required = 4;
        data.set_total_missions(&mut input).unwrap();

        assert_eq!(data.total_missions, 4);
    }

    #[test]
    fn invalid_total_missions_not_simple() {
        let mut input: &[u8] = &[0, 0];
        let mut data: ResultData = Default::default();
        let validated = data.set_total_missions(&mut input);

        match validated {
            Err(Error::InvalidTotalMissions(0)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn invalid_game_mode() {
        let mut input: &[u8] = &[9];
        let mut data: ResultData = Default::default();
        let validated = data.set_game_mode(&mut input);

        match validated {
            Err(Error::InvalidGameMode(9)) => assert!(true),
            _ => assert!(false),
        }
    }
}
