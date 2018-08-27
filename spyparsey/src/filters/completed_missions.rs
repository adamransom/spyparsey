use clap::ArgMatches;
use filters::Filter;
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
            false
        }
    }
}

impl Filter for CompletedMissions {
    basic_or!("completed-missions", Self::predicate);
}
