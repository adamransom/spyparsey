use super::StatCollection;
use clap::ArgMatches;
use spyparty::Replay;
use std::collections::HashMap;

/// A collection for keeping track of wins and losses for players.
#[derive(Default)]
pub struct PlayerStatCollection {
    totals: HashMap<String, u32>,
    stats: HashMap<String, (u32, u32)>,
}

impl StatCollection for PlayerStatCollection {
    fn aggregate(&mut self, replay: &Replay, matches: &ArgMatches) {
        let mut players: Vec<_> = Vec::new();
        let mut spies: Vec<_> = Vec::new();
        let mut snipers: Vec<_> = Vec::new();

        // --players and --pair are mutually exclusive, but can be totaled in a similar way.
        // --spies and --snipers need to be handled differently but they can never be used at the
        // same time as --players or --pair.
        if let Some(values) = matches.values_of("pair") {
            assert!(!matches.is_present("snipers") && !matches.is_present("spies"));
            players = values.collect();
        } else if let Some(values) = matches.values_of("players") {
            assert!(!matches.is_present("snipers") && !matches.is_present("spies"));
            players = values.collect()
        }

        if let Some(values) = matches.values_of("snipers") {
            snipers = values.collect();
        }

        if let Some(values) = matches.values_of("spies") {
            spies = values.collect();
        }

        // Add up wins and losses for games where a player won, regardless of role.
        for player in players {
            if replay.has_player(player) {
                if replay.is_win_for(player) {
                    add_win(&mut self.stats, player);
                } else if replay.is_loss_for(player) {
                    add_loss(&mut self.stats, player)
                }

                increment_total(&mut self.totals, player);
            }
        }

        // Only add up wins and losses where a player was a spy.
        for player in spies {
            if replay.has_spy(player) {
                if replay.is_spy_win() {
                    add_win(&mut self.stats, player);
                } else {
                    add_loss(&mut self.stats, player)
                }

                increment_total(&mut self.totals, player);
            }
        }

        // Only add up wins and losses where a player was a sniper.
        for player in snipers {
            if replay.has_sniper(player) {
                if replay.is_sniper_win() {
                    add_win(&mut self.stats, player);
                } else {
                    add_loss(&mut self.stats, player)
                }

                increment_total(&mut self.totals, player);
            }
        }
    }

    fn print(&self) {
        println!("Player Stats:");
        for (name, (wins, losses)) in &self.stats {
            println!(
                "    {}: {}W {}L ({:.1}%)",
                name,
                wins,
                losses,
                (*wins as f32 / self.totals[name] as f32) * 100f32
            );
        }
    }
}

/// Increment the "win" part of a tuple for a player.
fn add_win(stats: &mut HashMap<String, (u32, u32)>, player: &str) {
    stats
        .entry(player.to_string())
        .and_modify(|(w, _)| *w += 1)
        .or_insert((1, 0));
}

/// Increment the "loss" part of a tuple for a player.
fn add_loss(stats: &mut HashMap<String, (u32, u32)>, player: &str) {
    stats
        .entry(player.to_string())
        .and_modify(|(_, l)| *l += 1)
        .or_insert((0, 1));
}

/// Incrememt the total games played for a player.
fn increment_total(totals: &mut HashMap<String, u32>, player: &str) {
    totals
        .entry(player.to_string())
        .and_modify(|t| *t += 1)
        .or_insert(1);
}
