#[macro_use]
extern crate error_chain;

mod filters;
mod matched_replay;
mod output;
mod utils;

use crate::matched_replay::{MatchedReplay, MatchedReplayCollection};
use clap::load_yaml;
use clap::{App, ArgMatches};
use log::{info, warn};
use rayon::prelude::*;
use spyparty::{Map, Replay};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicIsize, Ordering};
use std::time::{Duration, SystemTime};
use walkdir::WalkDir;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {}
}

use crate::errors::*;

fn main() {
    if let Err(e) = run() {
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

/// Steps recursively through a path and tries to parse and filter replays.
fn process_replays<I, P>(paths: I, matches: &ArgMatches) -> Result<()>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut now = SystemTime::now();

    let replay_paths = find_replays(paths)?;

    let find_time = now.elapsed().unwrap_or_else(|_| Duration::new(0, 0));

    now = SystemTime::now();

    let mut replay_collection = parse_and_filter_replays(replay_paths, matches)?;

    replay_collection.dedup_and_sort();

    let parse_time = now.elapsed().unwrap_or_else(|_| Duration::new(0, 0));

    output::show(&replay_collection.replays, matches)?;

    info!(
        "Found {} replays ({}.{}s)",
        replay_collection.total,
        find_time.as_secs(),
        find_time.subsec_millis()
    );
    info!(
        "Parsed {} replays ({}.{}s)",
        replay_collection.parsed,
        parse_time.as_secs(),
        parse_time.subsec_millis()
    );
    info!("Matched {} replays", replay_collection.replays.len());

    Ok(())
}

fn find_replays<I, P>(paths: I) -> Result<Vec<PathBuf>>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut replay_paths = vec![];

    for path in paths {
        for entry in WalkDir::new(path) {
            // Ignore failed file reads
            if let Ok(entry) = entry {
                if let Some(ext) = entry.path().extension() {
                    if ext == "replay" {
                        replay_paths.push(entry.into_path());
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

    Ok(replay_paths)
}

fn parse_and_filter_replays(
    paths: Vec<PathBuf>,
    matches: &ArgMatches,
) -> Result<MatchedReplayCollection> {
    let parsed = AtomicIsize::new(0);
    let total = AtomicIsize::new(0);

    let replays = paths
        .par_iter()
        .filter_map(|path| {
            let mut matched_replay = None;

            // We have a possible replay, let's parse it!
            if let Some(replay) = parse(path) {
                parsed.fetch_add(1, Ordering::SeqCst);

                if filters::filter(&replay, matches).unwrap_or(false) {
                    matched_replay = Some(MatchedReplay {
                        inner: replay,
                        path: path.display().to_string(),
                    });
                }
            }

            total.fetch_add(1, Ordering::SeqCst);

            matched_replay
        })
        .collect::<Vec<_>>();

    Ok(MatchedReplayCollection {
        replays,
        total: total.into_inner(),
        parsed: parsed.into_inner(),
    })
}

/// Tries to parse a replay at a specific path.
fn parse(path: &Path) -> Option<Replay> {
    // Ignore failed file reads
    if let Ok(file) = File::open(path) {
        let mut reader = BufReader::new(file);
        // Ignore failed parses
        match Replay::from_reader(&mut reader) {
            Ok(replay) => {
                if let Map::Unknown(x) = replay.header.result_data.map {
                    warn!("unrecognised map in '{}' (0x{:x})", path.display(), x);
                }
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
