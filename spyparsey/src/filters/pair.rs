use clap::ArgMatches;
use filters::Filter;
use spyparty::Replay;

/// Filters replays that contain a pair of players.
pub struct Pair {}

impl Pair {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        replay.has_name(arg)
    }
}
impl Filter for Pair {
    basic_and!("pair", Self::predicate);
}
