extern crate spyparty;

use spyparty::Replay;
use std::fs::File;

#[test]
fn valid_replay_v5() {
    let mut file = File::open("tests/basicv5.replay").unwrap();

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
    assert_eq!(replay.header().latency, 0.75);
    assert_eq!(replay.header().data_size, 92399);
}

#[test]
fn valid_replay_v4() {
    let mut file = File::open("tests/basicv4.replay").unwrap();

    let replay = Replay::from_reader(&mut file).unwrap();

    assert_eq!(replay.header().replay_version, 4);
    assert_eq!(replay.header().protocol_version, 23);
    assert_eq!(replay.header().spyparty_version, 6015);
    assert_eq!(replay.header().flags, 0);
    assert_eq!(replay.header().duration, 155.875);
    assert_eq!(replay.header().game_id, 0x034bef9023a8ae82934b2e4cf8d97854);
    assert_eq!(replay.header().start_time, 1523357331);
    assert_eq!(replay.header().play_id, 2);
    assert_eq!(replay.header().spy_user_len, 11);
    assert_eq!(replay.header().sniper_user_len, 9);
    assert_eq!(replay.header().spy_display_len, 0);
    assert_eq!(replay.header().sniper_display_len, 0);
    assert_eq!(replay.header().latency, 0.75);
    assert_eq!(replay.header().data_size, 125564);
}

#[test]
fn invalid_replay() {
    let mut file = File::open("tests/broken.replay").unwrap();

    let replay = Replay::from_reader(&mut file);

    assert!(replay.is_err());
}
