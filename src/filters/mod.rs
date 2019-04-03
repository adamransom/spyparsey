use clap::ArgMatches;
use spyparty::Replay;

/// Trait to be used by filters on replays.
pub trait Filter {
    fn filter(&self, replay: &Replay, matches: &ArgMatches) -> bool;
}

// Macros have to come before the separate filter modules!

/// Macro to create a very simple OR-type filter for multiple values.
macro_rules! basic_or {
    ($arg:expr, $pred:path) => {
        fn filter(&self, replay: &Replay, matches: &ArgMatches) -> bool {
            if let Some(mut values) = matches.values_of($arg) {
                values.any(|p| $pred(p, replay))
            } else {
                true
            }
        }
    }
}

/// Macro to create a very simple AND-type filter for multiple values.
macro_rules! basic_and {
    ($arg:expr, $pred:path) => {
        fn filter(&self, replay: &Replay, matches: &ArgMatches) -> bool {
            if let Some(mut values) = matches.values_of($arg) {
                values.all(|p| $pred(p, replay))
            } else {
                true
            }
        }
    }
}

/// Macro to create a very simple filter for single values.
macro_rules! basic_presence {
    ($arg:expr, $pred:path) => {
        fn filter(&self, replay: &Replay, matches: &ArgMatches) -> bool {
            if matches.is_present($arg) {
                $pred(replay)
            } else {
                true
            }
        }
    }
}

mod completed_missions;
mod completed_missions_all;
mod game_modes;
mod maps;
mod pair;
mod players;
mod results;
mod sniper_win;
mod snipers;
mod spies;
mod spy_win;

pub use completed_missions::CompletedMissions;
pub use completed_missions_all::CompletedMissionsAll;
pub use game_modes::GameModes;
pub use maps::Maps;
pub use pair::Pair;
pub use players::Players;
pub use results::Results;
pub use sniper_win::SniperWin;
pub use snipers::Snipers;
pub use spies::Spies;
pub use spy_win::SpyWin;
