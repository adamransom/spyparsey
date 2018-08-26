use clap::ArgMatches;
use filters::Filter;
use spyparty::Replay;

/// Filters replays that contain a specific player (as either the sniper or the spy).
pub struct Players {}

impl Players {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        replay.has_name(arg)
    }
}
impl Filter for Players {
    basic_or!("players", Self::predicate);
}
