use clap::ArgMatches;
use filters::Filter;
use spyparty::Replay;
use std::convert::TryInto;

/// Filters replays that end in a specific result.
pub struct Results {}

impl Results {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        if let Ok(result) = arg.try_into() {
            replay.header.result_data.game_result == result
        } else {
            false
        }
    }
}

impl Filter for Results {
    basic_or!("results", Self::predicate);
}
