extern crate spyparty;

use spyparty::Replay;
use std::fs::File;

#[test]
fn valid_replay() {
    let mut file = File::open("tests/basic.replay").unwrap();

    let replay = Replay::from_reader(&mut file).unwrap();

    assert_eq!(replay.header().replay_version, 5);
    assert_eq!(replay.header().protocol_version, 23);
    assert_eq!(replay.header().spyparty_version, 6134);
    assert_eq!(replay.header().flags, 0);
    assert_eq!(replay.header().duration, 125.3125);
    assert_eq!(replay.header().game_id, 0x9dca1e19a581d2af884a4ff7b686b532);
    assert_eq!(replay.header().start_time, 1534431629);
    assert_eq!(replay.header().play_id, 1);
    assert_eq!(replay.header().spy_user_len, 9);
    assert_eq!(replay.header().sniper_user_len, 9);
    assert_eq!(replay.header().spy_display_len, 0);
    assert_eq!(replay.header().sniper_display_len, 0);
}

#[test]
fn invalid_replay() {
    let mut file = File::open("tests/broken.replay").unwrap();

    let replay = Replay::from_reader(&mut file);

    assert!(replay.is_err());
}
