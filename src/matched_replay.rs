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

/// A struct representing a collection of replays
pub struct MatchedReplayCollection {
    /// The collection of replays (after filters)
    pub replays: Vec<MatchedReplay>,
    /// The total number of replays found
    pub total: isize,
    /// The total number of replays parsed
    pub parsed: isize,
}

impl MatchedReplayCollection {
    /// Removes duplicate replays (by game ID) and then sorts them by start time.
    pub fn dedup_and_sort(&mut self) {
        self.replays.sort_by(|a, b| {
            a.inner
                .header
                .game_id
                .partial_cmp(&b.inner.header.game_id)
                .unwrap()
        });
        self.replays.dedup();
        self.replays.sort_unstable();
    }
}
