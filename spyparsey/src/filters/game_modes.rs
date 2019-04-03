use super::Filter;
use clap::ArgMatches;
use spyparty::{GameMode, Replay};
use std::convert::TryInto;

/// Filters replays that are specific game modes.
pub struct GameModes {}

impl GameModes {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        match arg {
            "any" | "a" => {
                if let GameMode::Any(..) = replay.header.result_data.game_mode {
                    return true;
                }
            }
            "pick" | "p" => {
                if let GameMode::Pick(..) = replay.header.result_data.game_mode {
                    return true;
                }
            }
            "known" | "k" => {
                if let GameMode::Known(..) = replay.header.result_data.game_mode {
                    return true;
                }
            }
            _ => {
                if let Ok(mode) = arg.try_into() {
                    return replay.header.result_data.game_mode == mode;
                }
            }
        }

        false
    }
}

impl Filter for GameModes {
    basic_or!("modes", Self::predicate);
}
