use super::Filter;
use clap::ArgMatches;
use spyparty::Replay;

/// Filters replays that contain a specific sniper.
pub struct Snipers {}

impl Snipers {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        replay.has_sniper(arg)
    }
}
impl Filter for Snipers {
    basic_or!("snipers", Self::predicate);
}
