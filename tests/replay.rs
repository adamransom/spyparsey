extern crate spyparty;

use spyparty::Replay;
use std::fs::File;

#[test]
fn valid_replay() {
    let mut file = File::open("tests/basic.replay").unwrap();

    let replay = Replay::from_reader(&mut file).unwrap();

    assert_eq!(replay.header().replay_version, 5);
}

#[test]
fn invalid_replay() {
    let mut file = File::open("tests/broken.replay").unwrap();

    let replay = Replay::from_reader(&mut file);

    assert!(replay.is_err());
}
