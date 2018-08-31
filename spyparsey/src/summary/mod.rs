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
mod mission_stat_collection;
mod mode_stat_collection;
mod player_stat_collection;
mod result_stat_collection;

use self::map_stat_collection::MapStatCollection;
use self::mission_stat_collection::MissionStatCollection;
use self::mode_stat_collection::ModeStatCollection;
use self::player_stat_collection::PlayerStatCollection;
use self::result_stat_collection::ResultStatCollection;
use super::MatchedReplay;
use clap::ArgMatches;
use spyparty::Replay;
use std::collections::HashMap;

/// A trait defining a collection of stats.
trait StatCollection {
    /// Adds the stats of a replay to the collection.
    fn aggregate<'a>(&mut self, replay: &Replay, matches: &'a ArgMatches);
    /// Prints the stats to stdout.
    fn print(&self);
}

/// Shows a summary of the filtered replays.
///
/// What is shown is based on which filters were used when querying the replays.
pub fn show(replays: &Vec<MatchedReplay>, matches: &ArgMatches) {
    let mut map_stats: MapStatCollection = Default::default();
    let mut mission_stats: MissionStatCollection = Default::default();
    let mut mode_stats: ModeStatCollection = Default::default();
    let mut result_stats: ResultStatCollection = Default::default();
    let mut player_stats: PlayerStatCollection = Default::default();

    let mut all_stats: Vec<&mut StatCollection> = Vec::new();

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

    // Show mode stats if not filtered by modes
    if !matches.is_present("modes") {
        all_stats.push(&mut mode_stats);
    }

    // Show result stats if not filtered by results
    if !matches.is_present("results") {
        all_stats.push(&mut result_stats);
    }

    for replay in replays {
        for ref mut stats in &mut all_stats {
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
