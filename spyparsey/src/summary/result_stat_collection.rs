use super::{increment, StatCollection};
use clap::ArgMatches;
use spyparty::replay::{GameResult, Replay};
use std::collections::HashMap;

/// A collection for keeping track of how many times each game ended in a particular result.
#[derive(Default)]
pub struct ResultStatCollection {
    total: u32,
    stats: HashMap<&'static str, u32>,
}

impl StatCollection for ResultStatCollection {
    fn aggregate(&mut self, replay: &Replay, _: &ArgMatches) {
        match replay.header.result_data.game_result {
            GameResult::MissionsWin => increment(&mut self.stats, "Missions Win"),
            GameResult::SpyShot => increment(&mut self.stats, "Spy Shot"),
            GameResult::SpyTimeout => increment(&mut self.stats, "Spy Timeout"),
            GameResult::CivilianShot => increment(&mut self.stats, "Civilian Shot"),
            GameResult::InProgress => increment(&mut self.stats, "Unfinished"),
        }

        self.total += 1;
    }

    fn print(&self) {
        print_single!(self, "Results");
    }
}
