#![feature(try_from)]

extern crate spyparty;
#[macro_use]
extern crate error_chain;
extern crate walkdir;
#[macro_use]
extern crate clap;

mod filters;

use clap::{App, ArgMatches};
use filters::*;
use spyparty::Replay;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use walkdir::WalkDir;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use errors::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut parsed = 0;
    let mut total = 0;

    // It's safe to unwrap here as "paths" is required
    for path in matches.values_of("paths").unwrap() {
        for entry in WalkDir::new(path) {
            // Ignore failed file reads
            if let Ok(entry) = entry {
                if let Some(ext) = entry.path().extension() {
                    if ext == "replay" {
                        // We have a possible replay, let's parse it!
                        if parse(entry.path(), &matches)? {
                            parsed += 1;
                        }

                        total += 1;
                    }
                }
            }
        }
    }

    println!("Parsed {} out of {} replays!", parsed, total);

    Ok(())
}

fn parse(path: &Path, matches: &ArgMatches) -> Result<bool> {
    // Ignore failed file reads
    if let Ok(file) = File::open(path) {
        let mut reader = BufReader::new(file);
        // Ignore failed parses
        if let Ok(replay) = Replay::from_reader(&mut reader) {
            // We handle things in 3 steps: filter, aggregate and ouput
            return filter(&replay, matches).chain_err(|| "failed to apply filter");
        }
    }

    Ok(false)
}

macro_rules! register_filters {
    ($filters:ident, $($filter:ident),*) => {
        let $filters: &[&Filter] = &[$(&$filter {}),*];
    };
}

fn filter(replay: &Replay, matches: &ArgMatches) -> Result<bool> {
    register_filters!(filters, Players, Pair, Maps, Spies, Snipers, Results, SpyWin, SniperWin);

    Ok(filters.iter().all(|f| f.filter(replay, matches)))
}
