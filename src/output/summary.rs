/// Macro used by a StatCollection which only focuses on one value (like map count) to print the
/// results to stdout.
macro_rules! print_single {
    ($self:ident, $title:expr) => {
        let mut collection: Vec<_> = $self.stats.iter().collect();
        collection.sort_by(|(_, a), (_, b)| b.cmp(a));

        println!("{}:", $title);
        for (name, value) in collection {
            println!(
                "    {}: {} ({:.1}%)",
                name,
                value,
                (*value as f32 / $self.total as f32) * 100f32
            );
        }
    };
}

mod map_stat_collection;
mod mission_set_stat_collection;
mod mission_stat_collection;
mod mode_stat_collection;
mod player_stat_collection;
mod result_stat_collection;

use crate::MatchedReplay;
use clap::ArgMatches;
use map_stat_collection::MapStatCollection;
use mission_set_stat_collection::MissionSetStatCollection;
use mission_stat_collection::MissionStatCollection;
use mode_stat_collection::ModeStatCollection;
use player_stat_collection::PlayerStatCollection;
use result_stat_collection::ResultStatCollection;
use spyparty::Replay;
use std::collections::HashMap;

/// A trait defining a collection of stats.
trait StatCollection {
    /// Adds the stats of a replay to the collection.
    fn aggregate(&mut self, replay: &Replay, matches: &ArgMatches);
    /// Prints the stats to stdout.
    fn print(&self);
}

/// Shows a summary of the filtered replays.
///
/// What is shown is based on which filters were used when querying the replays.
pub fn show(replays: &[MatchedReplay], matches: &ArgMatches) {
    let mut map_stats: MapStatCollection = Default::default();
    let mut mission_stats: MissionStatCollection = Default::default();
    let mut mission_set_stats: MissionSetStatCollection = Default::default();
    let mut mode_stats: ModeStatCollection = Default::default();
    let mut result_stats: ResultStatCollection = Default::default();
    let mut player_stats: PlayerStatCollection = Default::default();

    let mut all_stats: Vec<&mut StatCollection> = Vec::new();

    // Early return if no replays!
    if replays.is_empty() {
        println!("No replays found.");
        return;
    }

    // Show player stats if filtering on players
    if matches.is_present("pair")
        || matches.is_present("players")
        || matches.is_present("spies")
        || matches.is_present("snipers")
    {
        all_stats.push(&mut player_stats);
    }

    // Show map stats if not filtered by maps
    if !matches.is_present("maps") {
        all_stats.push(&mut map_stats);
    }

    // Always show missions stats
    all_stats.push(&mut mission_stats);
    all_stats.push(&mut mission_set_stats);

    // Show mode stats if not filtered by modes
    if !matches.is_present("modes") {
        all_stats.push(&mut mode_stats);
    }

    // Show result stats if not filtered by results
    if !matches.is_present("results") {
        all_stats.push(&mut result_stats);
    }

    for replay in replays {
        for stats in &mut all_stats {
            stats.aggregate(&replay.inner, matches);
        }
    }

    println!("Total Replays: \n    {}", replays.len());

    for stats in all_stats {
        stats.print();
    }
}

/// A helper function which increments a key in a hashmap or initializes it to 1 if it doesn't
/// exist.
fn increment<'a>(stats: &mut HashMap<&'a str, u32>, name: &'a str) {
    stats.entry(name).and_modify(|s| *s += 1).or_insert(1);
}
