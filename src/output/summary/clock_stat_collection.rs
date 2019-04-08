use super::StatCollection;
use clap::ArgMatches;
use spyparty::Replay;

/// A collection for keeping track lengths of games and clock usage.
#[derive(Default)]
pub struct ClockStatCollection {
    total: u32,
    total_duration: f32,
    total_duration_with_clock: f32,
    total_clock: u32,
}

impl StatCollection for ClockStatCollection {
    fn aggregate(&mut self, replay: &Replay, _: &ArgMatches) {
        let duration = replay.header.duration;

        if let Some(clock_start) = replay.header.result_data.clock_start {
            self.total_clock += clock_start;
            self.total_duration_with_clock += duration;
        }

        self.total_duration += duration;
        self.total += 1;
    }

    fn print(&self) {
        println!("Clock:");

        let average_duration = (self.total_duration / self.total as f32).round() as u32;
        let minutes = average_duration / 60;
        let seconds = average_duration % 60;

        println!("    Average Duration: {}m{}s", minutes, seconds);

        println!(
            "    Clock Usage: {:.1}%",
            (self.total_duration_with_clock / self.total_clock as f32) * 100f32
        );
    }
}
