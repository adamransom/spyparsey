use spyparty::Replay;

/// Checks if a replay has a particular player, automatically checking if the Steam version of
/// the name as well.
pub fn has_player(replay: &Replay, player: &str) -> bool {
    replay.has_player(player) || replay.has_player(&[player, "/steam"].join(""))
}

/// Checks if a replay has a particular spy, automatically checking if the Steam version of
/// the name as well.
pub fn has_spy(replay: &Replay, player: &str) -> bool {
    replay.has_spy(player) || replay.has_spy(&[player, "/steam"].join(""))
}

/// Checks if a replay has a particular sniper, automatically checking if the Steam version of
/// the name as well.
pub fn has_sniper(replay: &Replay, player: &str) -> bool {
    replay.has_sniper(player) || replay.has_sniper(&[player, "/steam"].join(""))
}

/// Calculates the percentage of a value out of a total.
pub fn percentage(value: u32, total: u32) -> f32 {
    (value as f32 / total as f32) * 100f32
}
