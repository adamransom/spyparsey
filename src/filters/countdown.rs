use super::Filter;
use clap::ArgMatches;
use spyparty::{GameMode, Replay};

/// Filters replays that contain a mission win countdown.
pub struct Countdown {}

impl Countdown {
    fn predicate(replay: &Replay) -> bool {
        let missions_required = match replay.header.result_data.game_mode {
            GameMode::Any(required, _) => required,
            GameMode::Pick(required, _) => required,
            GameMode::Known(required) => required,
        };

        missions_required as usize == replay.header.result_data.completed_missions.len()
    }
}

impl Filter for Countdown {
    basic_presence!("countdown", Self::predicate);
}
