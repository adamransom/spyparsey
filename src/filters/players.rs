use super::Filter;
use crate::utils::has_player;
use clap::ArgMatches;
use spyparty::Replay;

/// Filters replays that contain a specific player (as either the sniper or the spy).
pub struct Players {}

impl Players {
    fn predicate(arg: &str, replay: &Replay) -> bool {
        has_player(replay, arg)
    }
}
impl Filter for Players {
    basic_or!("players", Self::predicate);
}
