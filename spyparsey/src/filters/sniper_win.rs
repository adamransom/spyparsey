use clap::ArgMatches;
use filters::Filter;
use spyparty::Replay;

/// Filters replays that end in a sniper win.
pub struct SniperWin {}

impl SniperWin {
    fn predicate(replay: &Replay) -> bool {
        replay.is_sniper_win()
    }
}

impl Filter for SniperWin {
    basic_presence!("sniperwin", Self::predicate);
}
