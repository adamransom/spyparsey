use super::Filter;
use crate::utils::has_spy;
use clap::ArgMatches;
use spyparty::Replay;

/// Filters replays that contain a specific spy.
pub struct Spies {}

impl Spies {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        has_spy(replay, arg)
    }
}
impl Filter for Spies {
    basic_or!("spies", Self::predicate);
}
