use super::Filter;
use clap::ArgMatches;
use spyparty::Replay;
use std::convert::TryInto;

/// Filters replays that contain specific missions, all of which were completed.
pub struct CompletedMissionsAll {}

impl CompletedMissionsAll {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        if let Ok(mission) = arg.try_into() {
            replay
                .header
                .result_data
                .completed_missions
                .contains(&mission)
        } else {
            false
        }
    }
}

impl Filter for CompletedMissionsAll {
    basic_and!("completed-missions-all", Self::predicate);
}
