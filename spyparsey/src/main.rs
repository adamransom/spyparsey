extern crate spyparty;
#[macro_use]
extern crate error_chain;
extern crate walkdir;
#[macro_use]
extern crate clap;

use clap::App;
use spyparty::Replay;
use std::fs::File;
use std::io::BufReader;
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

    // It's safe to unwrap here as INPUT is required
    for path in matches.values_of("INPUT").unwrap() {
        for entry in WalkDir::new(path) {
            // Ignore failed file reads
            if let Ok(entry) = entry {
                if let Some(ext) = entry.path().extension() {
                    if ext == "replay" {
                        // Ignore failed file reads
                        if let Ok(mut file) = File::open(entry.path()) {
                            let mut reader = BufReader::new(file);
                            if let Ok(_replay) = Replay::from_reader(&mut reader) {
                                parsed += 1;
                            }
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
