use assert_cmd::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

#[test]
fn no_args() {
    let mut file = File::open("tests/snapshots/no_args.snapshot").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut cmd = Command::cargo_bin("spyparsey").unwrap();

    let output = cmd.arg("--path").arg("tests/replays").output().unwrap();

    assert_eq!(String::from_utf8_lossy(&output.stdout), contents);
}
