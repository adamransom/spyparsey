use super::Filter;
use clap::ArgMatches;
use spyparty::Replay;

/// Filters replays that end in a spy win.
pub struct SpyWin {}

impl SpyWin {
    fn predicate(replay: &Replay) -> bool {
        replay.is_spy_win()
    }
}

impl Filter for SpyWin {
    basic_presence!("spywin", Self::predicate);
}
