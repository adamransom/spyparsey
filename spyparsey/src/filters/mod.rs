use clap::ArgMatches;
use spyparty::Replay;

pub trait Filter {
    fn filter(&self, replay: &Replay, matches: &ArgMatches) -> bool;
}

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

pub struct Players {}

impl Players {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        replay.has_name(arg)
    }
}
impl Filter for Players {
    basic_or!("players", Self::predicate);
}

pub struct Pair {}

impl Pair {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        replay.has_name(arg)
    }
}
impl Filter for Pair {
    basic_and!("pair", Self::predicate);
}
