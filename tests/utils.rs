use assert_cmd::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

macro_rules! test_snapshot {
    ($snapshot:ident, $args:expr) => {
        #[test]
        fn $snapshot() {
            crate::utils::assert_snapshot(stringify!($snapshot), $args);
        }
    };
}

pub fn assert_snapshot(snapshot: &str, args: &[&str]) {
    let mut snapshot_file = File::open(format!("tests/snapshots/{}.snapshot", snapshot)).unwrap();
    let mut snapshot_contents = String::new();

    snapshot_file
        .read_to_string(&mut snapshot_contents)
        .unwrap();

    let stdout = Command::cargo_bin("spyparsey")
        .unwrap()
        .arg("--path")
        .arg("tests/replays")
        .args(args)
        .output()
        .unwrap()
        .stdout;

    let output_contents = String::from_utf8_lossy(&stdout);

    assert_eq!(output_contents, snapshot_contents);
}
