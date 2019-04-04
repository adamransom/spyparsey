use spyparty::Replay;
use std::cmp::Ordering;

/// A struct representing a combination of a replay and the path it was found at.
pub struct MatchedReplay {
    /// The parsed replay.
    pub inner: Replay,
    /// The path the replay was found at.
    pub path: String,
}

impl Ord for MatchedReplay {
    fn cmp(&self, other: &MatchedReplay) -> Ordering {
        self.inner
            .header
            .start_time
            .cmp(&other.inner.header.start_time)
    }
}

impl PartialOrd for MatchedReplay {
    fn partial_cmp(&self, other: &MatchedReplay) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MatchedReplay {
    fn eq(&self, other: &MatchedReplay) -> bool {
        self.inner.header.game_id == other.inner.header.game_id
    }
}

impl Eq for MatchedReplay {}

/// A struct representing a combination of a replay and the path it was found at.
pub struct MatchedReplayCollection {
    /// The parsed replay.
    pub replays: Vec<MatchedReplay>,
    /// The path the replay was found at.
    pub total: isize,
    pub parsed: isize,
}
