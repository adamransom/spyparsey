#![feature(try_from)]

extern crate spyparty;
#[macro_use]
extern crate error_chain;
extern crate walkdir;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate stderrlog;

mod filters;

use clap::{App, ArgMatches};
use filters::*;
use spyparty::Replay;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use errors::*;

struct MatchedReplay {
    _replay: Replay,
    path: String,
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "{}", e).expect(errmsg);

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            for e in e.iter().skip(1) {
                writeln!(stderr, "caused by: {}", e).expect(errmsg);
            }

            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Start logging
    let verbose = matches.occurrences_of("verbosity") as usize;
    stderrlog::new()
        .verbosity(verbose)
        .color(stderrlog::ColorChoice::Never)
        .init()
        .chain_err(|| "Failed to start logging.")?;

    if matches.is_present("paths") {
        process_replays(matches.values_of("paths").unwrap(), &matches)
    } else {
        let default_path = get_default_path().chain_err(|| "Could not locate the default SpyParty replays directory. Consider using --path to specify a directory instead.")?;
        process_replays(vec![default_path], &matches)
    }
}

#[cfg(windows)]
fn get_default_path() -> Result<PathBuf> {
    if let Some(app_data) = std::env::var_os("LOCALAPPDATA") {
        let mut path = PathBuf::from(app_data);
        path.push("SpyParty");
        path.push("replays");

        if path.is_dir() {
            Ok(path)
        } else {
            bail!("cannot find SpyParty replays directory")
        }
    } else {
        bail!("could not find local application data");
    }
}

#[cfg(not(windows))]
fn get_default_path() -> Result<PathBuf> {
    bail!("default directory searching only available on Windows");
}

fn process_replays<I, P>(paths: I, matches: &ArgMatches) -> Result<()>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut parsed = 0;
    let mut total = 0;
    let mut replays: Vec<MatchedReplay> = Vec::new();

    for path in paths {
        for entry in WalkDir::new(path) {
            // Ignore failed file reads
            if let Ok(entry) = entry {
                if let Some(ext) = entry.path().extension() {
                    if ext == "replay" {
                        // We have a possible replay, let's parse it!
                        if let Some(replay) = parse(entry.path()) {
                            parsed += 1;

                            if filter(&replay, matches).chain_err(|| "failed to apply filter")? {
                                replays.push(MatchedReplay {
                                    _replay: replay,
                                    path: entry.path().display().to_string(),
                                });
                            }
                        }

                        total += 1;
                    }
                }
            } else {
                match entry.err().unwrap().path() {
                    Some(path) => warn!("failed to read file '{}'", path.display()),
                    None => warn!("failed to read file"),
                }
            }
        }
    }

    output(&replays, matches);

    info!("Found {} replays", total);
    info!("Parsed {} replays", parsed);
    info!("Matched {} replays", replays.len());

    Ok(())
}

fn parse(path: &Path) -> Option<Replay> {
    // Ignore failed file reads
    if let Ok(file) = File::open(path) {
        let mut reader = BufReader::new(file);
        // Ignore failed parses
        match Replay::from_reader(&mut reader) {
            Ok(replay) => {
                return Some(replay);
            }
            Err(e) => {
                warn!("failed to parse replay '{}' ({})", path.display(), e);
            }
        }
    } else {
        warn!("failed to read file '{}'", path.display());
    }

    None
}

macro_rules! register_filters {
    ($filters:ident, $($filter:ident),*) => {
        let $filters: &[&Filter] = &[$(&$filter {}),*];
    };
}

fn filter(replay: &Replay, matches: &ArgMatches) -> Result<bool> {
    register_filters!(
        filters,
        CompletedMissions,
        CompletedMissionsAll,
        GameModes,
        Maps,
        Pair,
        Players,
        Results,
        SniperWin,
        Snipers,
        Spies,
        SpyWin
    );

    Ok(filters.iter().all(|f| f.filter(replay, matches)))
}

fn output(replays: &Vec<MatchedReplay>, matches: &ArgMatches) {
    if matches.is_present("count") {
        println!("{}", replays.len())
    } else if matches.is_present("show-paths") {
        for replay in replays {
            println!("{}", replay.path);
        }
    }
}
