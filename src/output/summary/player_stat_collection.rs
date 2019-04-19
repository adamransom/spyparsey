use super::StatCollection;
use crate::utils::{has_sniper, has_spy};
use clap::ArgMatches;
use spyparty::Replay;
use std::collections::HashMap;

/// A collection for keeping track of wins and losses for players.
#[derive(Default)]
pub struct PlayerStatCollection {
    totals: HashMap<String, u32>,
    stats: HashMap<String, (String, u32, u32)>,
}

impl StatCollection for PlayerStatCollection {
    fn aggregate(&mut self, replay: &Replay, matches: &ArgMatches) {
        let mut players: Vec<_> = Vec::new();
        let mut count_spy_wins = false;
        let mut count_sniper_wins = false;

        if let Some(values) = matches.values_of("pair") {
            count_spy_wins = true;
            count_sniper_wins = true;
            players = values.collect();
        } else if let Some(values) = matches.values_of("players") {
            count_spy_wins = true;
            count_sniper_wins = true;
            players = values.collect();
        }

        if let Some(values) = matches.values_of("spies") {
            count_spy_wins = true;
            players.append(&mut values.collect());
        }

        if let Some(values) = matches.values_of("snipers") {
            count_sniper_wins = true;
            players.append(&mut values.collect());
        }

        // Only add up wins and losses where a player was a spy.
        if count_spy_wins {
            for player in &players {
                if has_spy(&replay, player) {
                    let user_name = &replay.header.spy_user_name;
                    let display_name = &replay.spy_name();

                    if replay.is_spy_win() {
                        add_win(&mut self.stats, user_name, display_name);
                    } else if replay.is_sniper_win() {
                        add_loss(&mut self.stats, user_name, display_name)
                    }

                    increment_total(&mut self.totals, user_name);
                }
            }
        }

        // Only add up wins and losses where a player was a sniper.
        if count_sniper_wins {
            for player in &players {
                if has_sniper(&replay, player) {
                    let user_name = &replay.header.sniper_user_name;
                    let display_name = &replay.sniper_name();

                    if replay.is_sniper_win() {
                        add_win(&mut self.stats, user_name, display_name);
                    } else if replay.is_spy_win() {
                        add_loss(&mut self.stats, user_name, display_name)
                    }

                    increment_total(&mut self.totals, user_name);
                }
            }
        }
    }

    fn print(&self) {
        println!("Player Stats:");
        for (user_name, (display_name, wins, losses)) in &self.stats {
            println!(
                "    {}: {}W {}L ({:.1}%)",
                display_name,
                wins,
                losses,
                (*wins as f32 / self.totals[user_name] as f32) * 100f32
            );
        }
    }
}

/// Increment the "win" part of a tuple for a player.
fn add_win(stats: &mut HashMap<String, (String, u32, u32)>, user_name: &str, display_name: &str) {
    stats
        .entry(user_name.to_string())
        .and_modify(|(_, w, _)| *w += 1)
        .or_insert((display_name.to_string(), 1, 0));
}

/// Increment the "loss" part of a tuple for a player.
fn add_loss(stats: &mut HashMap<String, (String, u32, u32)>, user_name: &str, display_name: &str) {
    stats
        .entry(user_name.to_string())
        .and_modify(|(_, _, l)| *l += 1)
        .or_insert((display_name.to_string(), 0, 1));
}

/// Incrememt the total games played for a player.
fn increment_total(totals: &mut HashMap<String, u32>, user_name: &str) {
    totals
        .entry(user_name.to_string())
        .and_modify(|t| *t += 1)
        .or_insert(1);
}
