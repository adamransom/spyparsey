use spyparty::Replay;

pub fn has_player(replay: &Replay, player: &str) -> bool {
    replay.has_player(player) || replay.has_player(&[player, "/steam"].join(""))
}

pub fn has_spy(replay: &Replay, player: &str) -> bool {
    replay.has_spy(player) || replay.has_spy(&[player, "/steam"].join(""))
}

pub fn has_sniper(replay: &Replay, player: &str) -> bool {
    replay.has_sniper(player) || replay.has_sniper(&[player, "/steam"].join(""))
}
