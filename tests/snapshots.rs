#[macro_use]
mod utils;

test_snapshot!(no_args, &[]);
test_snapshot!(player, &["--player", "checker"]);
test_snapshot!(pair, &["--pair", "checker", "lazybear"]);
test_snapshot!(
    spy_sniper,
    &["--spy", "canadianbacon", "--sniper", "krazycaley"]
);
test_snapshot!(map, &["--map", "teien"]);
test_snapshot!(
    missions_players_mode,
    &[
        "--completed-missions",
        "bug",
        "--players",
        "lazybear",
        "checker",
        "--mode",
        "a4/8"
    ]
);
