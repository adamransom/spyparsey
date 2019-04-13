use crate::errors::*;
use crate::MatchedReplay;
use spyparty::{GameMode, MapVariant, Mission};

/// Outputs almost all the data in the replay header in a CSV format.
pub fn show(replays: &[MatchedReplay]) -> Result<()> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    wtr.write_record(&[
        "replay_version",
        "protocol_version",
        "spyparty_version",
        "duration",
        "game_id",
        "start_time",
        "play_id",
        "latency",
        "spy_user_name",
        "spy_display_name",
        "sniper_user_name",
        "sniper_display_name",
        "simple_rules",
        "result",
        "mode",
        "map",
        "map_variant",
        "selected_missions",
        "picked_missions",
        "completed_missions",
        "completed_missions_hash",
        "guests",
        "clock_start",
    ])
    .chain_err(|| "failed to write CSV record")?;

    for replay in replays {
        let header = &replay.inner.header;
        let result_data = &header.result_data;

        wtr.write_record(&[
            &header.replay_version.to_string(),
            &header.protocol_version.to_string(),
            &header.spyparty_version.to_string(),
            &header.duration.to_string(),
            &format!("{:x}", &header.game_id),
            &header.start_time.to_string(),
            &header.play_id.to_string(),
            &header.latency.to_string(),
            &header.spy_user_name,
            &replay.inner.spy_name(),
            &header.sniper_user_name,
            &replay.inner.sniper_name(),
            &match result_data.simple_rules {
                Some(simple_rules) => simple_rules.to_string(),
                None => "".to_string(),
            },
            &format!("{:?}", result_data.game_result),
            &mode_to_string(&result_data.game_mode),
            &format!("{}", result_data.map),
            &match &result_data.map_variant {
                MapVariant::Teien(variant) => format!("{:?}", variant),
                MapVariant::None => "None".to_string(),
            },
            &join_missions(&result_data.selected_missions),
            &join_missions(&result_data.picked_missions),
            &join_missions(&result_data.completed_missions),
            &result_data.completed_missions_raw.to_string(),
            &match result_data.guests {
                Some(guests) => guests.to_string(),
                None => "".to_string(),
            },
            &match result_data.clock_start {
                Some(clock_start) => clock_start.to_string(),
                None => "".to_string(),
            },
        ])
        .chain_err(|| "failed to write CSV record")?;
    }

    wtr.flush().chain_err(|| "failed to write to stdout")?;
    Ok(())
}

/// Join a list of missions together, separated by a comma.
fn join_missions(missions: &[Mission]) -> String {
    missions
        .iter()
        .map(|map| format!("{}", map))
        .collect::<Vec<String>>()
        .join(",")
}

/// Convert a game mode into its short version.
fn mode_to_string(mode: &GameMode) -> String {
    match mode {
        GameMode::Known(x) => format!("k{}", x),
        GameMode::Pick(x, y) => format!("p{}/{}", x, y),
        GameMode::Any(x, y) => format!("a{}/{}", x, y),
    }
}
