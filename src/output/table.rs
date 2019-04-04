use crate::errors::*;
use crate::MatchedReplay;
use clap::ArgMatches;
use prettytable::{cell, row};
use prettytable::{Cell, Row, Table};
use spyparty::{GameMode, GameResult, Replay};

/// Shows a table and CSV for use in a very specific Google Sheet.
///
/// It requires that a player argument has been passed and outputs stats specifically for that
/// player. If a pair argument is passed, it outputs stats for the first player in the pair.
///
/// This is still WIP, as it panics if you don't supply one of "player" or "pair".
pub fn show(replays: &[MatchedReplay], matches: &ArgMatches) -> Result<()> {
    assert!(matches.is_present("players") || matches.is_present("pair"));

    let mut spy_table = Table::new();
    let mut sniper_table = Table::new();
    let mut mission_table = Table::new();

    let name = matches
        .value_of("players")
        .unwrap_or_else(|| matches.value_of("pair").unwrap());

    add_headers(&mut spy_table, &mut sniper_table, &mut mission_table);

    for replay in replays {
        add_to_table(
            &replay.inner,
            name,
            &mut spy_table,
            &mut sniper_table,
            &mut mission_table,
        );
    }

    sniper_table.printstd();
    spy_table.printstd();

    sniper_table
        .to_csv(std::io::stdout())
        .chain_err(|| "failed to write sniper table")?;
    spy_table
        .to_csv(std::io::stdout())
        .chain_err(|| "failed to write spy table")?;
    mission_table
        .to_csv(std::io::stdout())
        .chain_err(|| "failed to write mission table")?;

    Ok(())
}

/// Creates the headers for the table.
fn add_headers(spy_table: &mut Table, sniper_table: &mut Table, mission_table: &mut Table) {
    sniper_table.add_row(row![
        "SNIPER",
        "MAP",
        "W/L",
        "MISSION COMPLETE",
        "OUTCOME",
        "CHARACTER",
        "CIVILIAN SHOT",
        "OPPONENT"
    ]);
    spy_table.add_row(row![
        "SPY",
        "MAP",
        "CHARACTER",
        "AMBA",
        "W/L",
        "MISSION COMPLETE",
        "OUTCOME",
        "OPPONENT"
    ]);
    mission_table.add_row(row![
        "MAP",
        "MISSION #1",
        "MISSION #2",
        "MISSION #3",
        "MISSION #4",
        "MISSION #5",
        "MISSION #6",
        "MISSION #7",
        "MISSION #8"
    ]);
}

/// Adds a replay to the table.
fn add_to_table(
    replay: &Replay,
    name: &str,
    spy_table: &mut Table,
    sniper_table: &mut Table,
    mission_table: &mut Table,
) {
    let mut row = Row::empty();
    let header = &replay.header;
    let result_data = &header.result_data;

    let required_missions = match result_data.game_mode {
        GameMode::Known(required) => required,
        GameMode::Pick(required, _) => required,
        GameMode::Any(required, _) => required,
    };

    row.add_cell(Cell::new(""));
    row.add_cell(Cell::new(&result_data.map.to_string()));

    if replay.sniper_name() == name {
        row.add_cell(Cell::new(match result_data.game_result {
            GameResult::MissionsWin | GameResult::CivilianShot => "L",
            _ => "W",
        }));
        if result_data.completed_missions.len() >= required_missions as usize {
            row.add_cell(Cell::new("Y"));
        } else {
            row.add_cell(Cell::new("N"));
        }
        row.add_cell(Cell::new(&format!("{:?}", &result_data.game_result)));
        row.add_cell(Cell::new(""));
        row.add_cell(Cell::new(""));
        row.add_cell(Cell::new(&replay.spy_name()));
        sniper_table.add_row(row);
    } else {
        row.add_cell(Cell::new(""));
        row.add_cell(Cell::new(""));
        row.add_cell(Cell::new(match result_data.game_result {
            GameResult::MissionsWin | GameResult::CivilianShot => "W",
            _ => "L",
        }));
        if result_data.completed_missions.len() >= required_missions as usize {
            row.add_cell(Cell::new("Y"));
        } else {
            row.add_cell(Cell::new("N"));
        }
        row.add_cell(Cell::new(&format!("{:?}", &result_data.game_result)));
        row.add_cell(Cell::new(&replay.sniper_name()));
        spy_table.add_row(row);

        row = Row::empty();
        row.add_cell(Cell::new(&result_data.map.to_string()));
        for mission in &result_data.completed_missions {
            row.add_cell(Cell::new(&mission.to_string()));
        }

        for _ in 0..(8 - result_data.completed_missions.len()) {
            row.add_cell(Cell::new(""));
        }

        mission_table.add_row(row);
    }
}
