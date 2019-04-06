use super::Filter;
use clap::ArgMatches;
use log::error;
use spyparty::Replay;
use std::convert::TryInto;

/// Filters replays that contain specific missions, any of which were completed.
pub struct CompletedMissions {}

impl CompletedMissions {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        if let Ok(mission) = arg.try_into() {
            replay
                .header
                .result_data
                .completed_missions
                .contains(&mission)
        } else {
            error!("'{}' is not a valid option for the mission filter", arg);
            false
        }
    }
}

impl Filter for CompletedMissions {
    basic_or!("completed-missions", Self::predicate);
}
