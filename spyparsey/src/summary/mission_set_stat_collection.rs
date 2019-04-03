use super::StatCollection;
use clap::ArgMatches;
use spyparty::{GameMode, Mission, Replay};
use std::collections::HashMap;

/// A collection for keeping track of how many times a set of missions counts as a completion.
#[derive(Default)]
pub struct MissionSetStatCollection {
    total: u32,
    stats: HashMap<u32, u32>,
}

impl StatCollection for MissionSetStatCollection {
    fn aggregate(&mut self, replay: &Replay, _: &ArgMatches) {
        let required_missions = match replay.header.result_data.game_mode {
            GameMode::Any(x, ..) | GameMode::Pick(x, ..) | GameMode::Known(x) => x,
        };
        let completed_missions = replay.header.result_data.completed_missions.len() as u8;

        if required_missions == completed_missions {
            let completed_missions_raw = replay.header.result_data.completed_missions_raw;
            self.stats
                .entry(completed_missions_raw)
                .and_modify(|s| *s += 1)
                .or_insert(1);
            self.total += 1;
        }
    }

    fn print(&self) {
        let mut collection: Vec<_> = self.stats.iter().collect();
        collection.sort_by(|(_, a), (_, b)| b.cmp(a));

        println!("Completed Mission Sets:");
        for (missions_raw, value) in collection.iter().take(10) {
            let missions = Mission::unpack_missions(**missions_raw);
            let mut mission_summary = String::new();

            for mission in missions {
                mission_summary.push_str(&format!("{}, ", mission.short_display()));
            }

            println!(
                "    {}: {} ({:.1}%)",
                mission_summary.trim_right_matches(", "),
                value,
                (**value as f32 / self.total as f32) * 100f32
            );
        }
    }
}
