use clap::ArgMatches;
use filters::Filter;
use spyparty::Replay;

/// Filters replays that contain a specific spy.
pub struct Spies {}

impl Spies {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        replay.has_spy(arg)
    }
}
impl Filter for Spies {
    basic_or!("spies", Self::predicate);
}
