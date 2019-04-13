use crate::errors::*;
use crate::MatchedReplay;
use clap::ArgMatches;

pub mod csv;
pub mod summary;
pub mod table;

/// Prints various representations of the filtered replays.
pub fn show(replays: &[MatchedReplay], matches: &ArgMatches) -> Result<()> {
    if matches.is_present("count") {
        println!("{}", replays.len());
    } else if matches.is_present("show-paths") {
        for replay in replays {
            println!("{}", replay.path);
        }
    } else if matches.is_present("csv") {
        csv::show(replays)?;
    } else if matches.is_present("special-csv") {
        table::show(replays, matches)?;
    } else {
        summary::show(replays, matches);
    }

    Ok(())
}
