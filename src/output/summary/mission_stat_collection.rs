use super::{increment, StatCollection};
use clap::ArgMatches;
use spyparty::{Mission, Replay};
use std::collections::HashMap;

/// A collection for keeping track of how many times each mission was completed.
#[derive(Default)]
pub struct MissionStatCollection {
    total: u32,
    stats: HashMap<&'static str, u32>,
}

impl StatCollection for MissionStatCollection {
    fn aggregate(&mut self, replay: &Replay, _: &ArgMatches) {
        let missions = &replay.header.result_data.completed_missions;

        if missions.contains(&Mission::BugAmbassador) {
            increment(&mut self.stats, "Bug Ambassador")
        }
        if missions.contains(&Mission::ContactDoubleAgent) {
            increment(&mut self.stats, "Contact Double Agent")
        }
        if missions.contains(&Mission::FingerprintAmbassador) {
            increment(&mut self.stats, "Fingerprint Ambassador")
        }
        if missions.contains(&Mission::InspectStatues) {
            increment(&mut self.stats, "Inspect Statues")
        }
        if missions.contains(&Mission::PurloinGuestList) {
            increment(&mut self.stats, "Purloin Guest List")
        }
        if missions.contains(&Mission::SeduceTarget) {
            increment(&mut self.stats, "Seduce Target")
        }
        if missions.contains(&Mission::SwapStatue) {
            increment(&mut self.stats, "Swap Statue")
        }
        if missions.contains(&Mission::TransferMicrofilm) {
            increment(&mut self.stats, "Transfer Microfilm")
        }

        self.total += missions.len() as u32;
    }

    fn print(&self) {
        print_single!(self, "Missions Completed");
    }
}
