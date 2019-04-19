use super::{increment, StatCollection};
use clap::ArgMatches;
use spyparty::{Mission, Replay};
use std::collections::HashMap;

/// A collection for keeping track of how many times each mission was completed.
#[derive(Default)]
pub struct MissionStatCollection {
    total: HashMap<&'static str, u32>,
    stats: HashMap<&'static str, u32>,
}

impl StatCollection for MissionStatCollection {
    fn aggregate(&mut self, replay: &Replay, _: &ArgMatches) {
        increment_missions(
            &mut self.stats,
            &replay.header.result_data.completed_missions,
        );
        increment_missions(
            &mut self.total,
            &replay.header.result_data.selected_missions,
        );
    }

    fn print(&self) {
        let mut collection: Vec<_> = self.stats.iter().collect();
        collection.sort_by(|(_, a), (_, b)| b.cmp(a));

        println!("Missions Completed:");
        for (name, value) in collection {
            println!(
                "    {}: {} ({:.1}%)",
                name,
                value,
                (*value as f32 / self.total[name] as f32) * 100f32
            );
        }
    }
}

/// Increment a counter of how many times a mission appeared in a particular list.
fn increment_missions<'a>(mut stats: &mut HashMap<&'a str, u32>, missions: &[Mission]) {
    if missions.contains(&Mission::BugAmbassador) {
        increment(&mut stats, "Bug Ambassador")
    }
    if missions.contains(&Mission::ContactDoubleAgent) {
        increment(&mut stats, "Contact Double Agent")
    }
    if missions.contains(&Mission::FingerprintAmbassador) {
        increment(&mut stats, "Fingerprint Ambassador")
    }
    if missions.contains(&Mission::InspectStatues) {
        increment(&mut stats, "Inspect Statues")
    }
    if missions.contains(&Mission::PurloinGuestList) {
        increment(&mut stats, "Purloin Guest List")
    }
    if missions.contains(&Mission::SeduceTarget) {
        increment(&mut stats, "Seduce Target")
    }
    if missions.contains(&Mission::SwapStatue) {
        increment(&mut stats, "Swap Statue")
    }
    if missions.contains(&Mission::TransferMicrofilm) {
        increment(&mut stats, "Transfer Microfilm")
    }
}
