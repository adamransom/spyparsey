use super::{increment, StatCollection};
use clap::ArgMatches;
use spyparty::{GameMode, Replay};
use std::collections::HashMap;

/// A collection for keeping track of how many times each game mode was played.
#[derive(Default)]
pub struct ModeStatCollection {
    total: u32,
    stats: HashMap<&'static str, u32>,
}

impl StatCollection for ModeStatCollection {
    fn aggregate(&mut self, replay: &Replay, _: &ArgMatches) {
        match replay.header.result_data.game_mode {
            GameMode::Any(..) => increment(&mut self.stats, "Any"),
            GameMode::Pick(..) => increment(&mut self.stats, "Pick"),
            GameMode::Known(..) => increment(&mut self.stats, "Known"),
        }

        self.total += 1;
    }

    fn print(&self) {
        print_single!(self, "Modes Played");
    }
}
