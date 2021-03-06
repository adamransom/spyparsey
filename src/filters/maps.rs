use super::Filter;
use clap::ArgMatches;
use log::error;
use spyparty::Map;
use spyparty::Replay;
use std::convert::TryInto;

/// Filters replays that take place on a specific map.
pub struct Maps {}

impl Maps {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        if let Ok(map) = arg.try_into() {
            replay.header.result_data.map == map
        } else if arg.to_ascii_lowercase() == "unknown" {
            matches!(replay.header.result_data.map, Map::Unknown(_))
        } else {
            error!("'{}' is not a valid option for the map filter", arg);
            false
        }
    }
}

impl Filter for Maps {
    basic_or!("maps", Self::predicate);
}
