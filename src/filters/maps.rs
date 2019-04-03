use super::Filter;
use clap::ArgMatches;
use spyparty::Replay;
use std::convert::TryInto;

/// Filters replays that take place on a specific map.
pub struct Maps {}

impl Maps {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        if let Ok(map) = arg.try_into() {
            replay.header.result_data.map == map
        } else {
            false
        }
    }
}

impl Filter for Maps {
    basic_or!("maps", Self::predicate);
}
