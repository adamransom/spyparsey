use super::Filter;
use clap::ArgMatches;
use log::error;
use spyparty::{GameMode, Replay};
use std::convert::TryInto;

/// Filters replays that are specific game modes.
pub struct GameModes {}

impl GameModes {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        match arg {
            "any" | "a" => match replay.header.result_data.game_mode {
                GameMode::Any(..) => true,
                _ => false,
            },
            "pick" | "p" => match replay.header.result_data.game_mode {
                GameMode::Pick(..) => true,
                _ => false,
            },
            "known" | "k" => match replay.header.result_data.game_mode {
                GameMode::Known(..) => true,
                _ => false,
            },
            _ => {
                if let Ok(mode) = arg.try_into() {
                    replay.header.result_data.game_mode == mode
                } else {
                    error!("'{}' is not a valid option for the game mode filter", arg);
                    false
                }
            }
        }
    }
}

impl Filter for GameModes {
    basic_or!("modes", Self::predicate);
}
