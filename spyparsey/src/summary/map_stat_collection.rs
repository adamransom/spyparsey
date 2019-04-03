use super::{increment, StatCollection};
use clap::ArgMatches;
use spyparty::{Map, Replay};
use std::collections::HashMap;

/// A collection for keeping track of how many times each map was played.
#[derive(Default)]
pub struct MapStatCollection {
    total: u32,
    stats: HashMap<&'static str, u32>,
}

impl StatCollection for MapStatCollection {
    fn aggregate(&mut self, replay: &Replay, _: &ArgMatches) {
        match replay.header.result_data.map {
            Map::Aquarium => increment(&mut self.stats, "Aquarium"),
            Map::Balcony => increment(&mut self.stats, "Balcony"),
            Map::Ballroom => increment(&mut self.stats, "Ballroom"),
            Map::Courtyard => increment(&mut self.stats, "Courtyard"),
            Map::Gallery => increment(&mut self.stats, "Gallery"),
            Map::HighRise => increment(&mut self.stats, "HighRise"),
            Map::Library => increment(&mut self.stats, "Library"),
            Map::Moderne => increment(&mut self.stats, "Moderne"),
            Map::Pub => increment(&mut self.stats, "Pub"),
            Map::Teien => increment(&mut self.stats, "Teien"),
            Map::Terrace => increment(&mut self.stats, "Terrace"),
            Map::Veranda => increment(&mut self.stats, "Veranda"),
            _ => {}
        }

        self.total += 1;
    }

    fn print(&self) {
        print_single!(self, "Maps Played");
    }
}
