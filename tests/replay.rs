extern crate spyparty;

use spyparty::Replay;
use std::fs::File;

#[test]
fn valid_replay() {
    let mut file = File::open("tests/basic.replay").unwrap();

    let replay = Replay::from_reader(&mut file);

    assert!(replay.is_ok());
}

#[test]
fn invalid_replay() {
    let mut file = File::open("tests/broken.replay").unwrap();

    let replay = Replay::from_reader(&mut file);

    assert!(replay.is_err());
}
