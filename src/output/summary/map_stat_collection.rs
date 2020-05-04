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
            Map::CrowdedPub => increment(&mut self.stats, "Crowded Pub"),
            Map::DoubleModern => increment(&mut self.stats, "Double Modern"),
            Map::Gallery => increment(&mut self.stats, "Gallery"),
            Map::HighRise => increment(&mut self.stats, "High-Rise"),
            Map::Library => increment(&mut self.stats, "Library"),
            Map::Modern => increment(&mut self.stats, "Modern"),
            Map::Moderne => increment(&mut self.stats, "Moderne"),
            Map::OldBalcony => increment(&mut self.stats, "Old Balcony"),
            Map::OldBallroom => increment(&mut self.stats, "Old Ballroom"),
            Map::OldCourtyard1 => increment(&mut self.stats, "Old Courtyard 1"),
            Map::OldCourtyard2 => increment(&mut self.stats, "Old Courtyard 2"),
            Map::OldGallery => increment(&mut self.stats, "Old Gallery"),
            Map::OldVeranda => increment(&mut self.stats, "Old Veranda"),
            Map::Panopticon => increment(&mut self.stats, "Panopticon"),
            Map::Pub => increment(&mut self.stats, "Pub"),
            Map::Redwoods => increment(&mut self.stats, "Redwoods"),
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
