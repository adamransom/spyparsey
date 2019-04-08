use super::Filter;
use crate::utils::has_player;
use clap::ArgMatches;
use spyparty::Replay;

/// Filters replays that contain a pair of players.
pub struct Pair {}

impl Pair {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        has_player(replay, arg)
    }
}
impl Filter for Pair {
    basic_and!("pair", Self::predicate);
}
