use super::Filter;
use crate::utils::has_sniper;
use clap::ArgMatches;
use spyparty::Replay;

/// Filters replays that contain a specific sniper.
pub struct Snipers {}

impl Snipers {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        has_sniper(replay, arg)
    }
}
impl Filter for Snipers {
    basic_or!("snipers", Self::predicate);
}
