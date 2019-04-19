use crate::errors::*;
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
mod countdown;
mod game_modes;
mod maps;
mod pair;
mod players;
mod results;
mod sniper_win;
mod snipers;
mod spies;
mod spy_win;

use completed_missions::CompletedMissions;
use completed_missions_all::CompletedMissionsAll;
use countdown::Countdown;
use game_modes::GameModes;
use maps::Maps;
use pair::Pair;
use players::Players;
use results::Results;
use sniper_win::SniperWin;
use snipers::Snipers;
use spies::Spies;
use spy_win::SpyWin;

macro_rules! register_filters {
    ($filters:ident, $($filter:ident),*) => {
        let $filters: &[&Filter] = &[$(&$filter {}),*];
    };
}

/// Filters the replays based on various command line arguments.
pub fn filter(replay: &Replay, matches: &ArgMatches) -> Result<bool> {
    register_filters!(
        filters,
        CompletedMissions,
        CompletedMissionsAll,
        Countdown,
        GameModes,
        Maps,
        Pair,
        Players,
        Results,
        SniperWin,
        Snipers,
        Spies,
        SpyWin
    );

    Ok(filters.iter().all(|f| f.filter(replay, matches)))
}
