use clap::ArgMatches;
use spyparty::Replay;

/// Trait to be used by filters on replays.
pub trait Filter {
    fn filter(&self, replay: &Replay, matches: &ArgMatches) -> bool;
}

// Macros have to come before the separate filter modules!

/// Macro to create a very simple OR-type filter.
macro_rules! basic_or {
    ($arg:expr, $pred:path) => {
        fn filter(&self, replay: &Replay, matches: &ArgMatches) -> bool {
            if let Some(mut players) = matches.values_of($arg) {
                players.any(|p| $pred(p, replay))
            } else {
                true
            }
        }
    }
}

/// Macro to create a very simple AND-type filter.
macro_rules! basic_and {
    ($arg:expr, $pred:path) => {
        fn filter(&self, replay: &Replay, matches: &ArgMatches) -> bool {
            if let Some(mut players) = matches.values_of($arg) {
                players.all(|p| $pred(p, replay))
            } else {
                true
            }
        }
    }
}

mod players;
mod pair;
mod maps;

pub use self::players::Players;
pub use self::pair::Pair;
pub use self::maps::Maps;
