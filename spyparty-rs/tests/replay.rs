extern crate spyparty;

use spyparty::replay::{GameMode, GameResult, Map, Mission};
use spyparty::Replay;
use std::fs::File;

#[test]
fn valid_replay_v5() {
    let mut file = File::open("tests/basicv5.replay").unwrap();

    let replay = Replay::from_reader(&mut file).unwrap();

    assert_eq!(replay.header.replay_version, 5);
    assert_eq!(replay.header.protocol_version, 23);
    assert_eq!(replay.header.spyparty_version, 6134);
    assert_eq!(replay.header.flags, 0);
    assert_eq!(replay.header.duration, 125.3125);
    assert_eq!(replay.header.game_id, 0x9dca1e19a581d2af884a4ff7b686b532);
    assert_eq!(replay.header.start_time, 1534431629);
    assert_eq!(replay.header.play_id, 1);
    assert_eq!(replay.header.spy_user_len, 9);
    assert_eq!(replay.header.sniper_user_len, 9);
    assert_eq!(replay.header.spy_display_len, 0);
    assert_eq!(replay.header.sniper_display_len, 0);
    assert_eq!(replay.header.latency, 0.75);
    assert_eq!(replay.header.data_size, 92399);
    assert_eq!(replay.header.spy_user_name, "turnipboy");
    assert_eq!(replay.header.sniper_user_name, "plastikqs");
    assert_eq!(replay.header.spy_display_name, None);
    assert_eq!(replay.header.sniper_display_name, None);
    assert_eq!(replay.header.result_data.version, 2);
    assert_eq!(replay.header.result_data.simple_rules, Some(false));
    assert_eq!(replay.header.result_data.game_result, GameResult::SpyShot);
    assert_eq!(replay.header.result_data.game_mode, GameMode::Any(4, 7));
    assert_eq!(replay.header.result_data.map, Map::Courtyard);
    assert_eq!(
        replay.header.result_data.selected_missions,
        vec![
            Mission::BugAmbassador,
            Mission::ContactDoubleAgent,
            Mission::SwapStatue,
            Mission::InspectStatues,
            Mission::SeduceTarget,
            Mission::PurloinGuestList,
            Mission::FingerprintAmbassador
        ]
    );
    assert_eq!(
        replay.header.result_data.picked_missions,
        vec![
            Mission::BugAmbassador,
            Mission::ContactDoubleAgent,
            Mission::SwapStatue,
            Mission::InspectStatues,
            Mission::SeduceTarget,
            Mission::PurloinGuestList,
            Mission::FingerprintAmbassador
        ]
    );
    assert_eq!(
        replay.header.result_data.completed_missions,
        vec![Mission::ContactDoubleAgent, Mission::PurloinGuestList]
    );
    assert_eq!(replay.header.result_data.guests, Some(16));
    assert_eq!(replay.header.result_data.clock_start, Some(180));
}

#[test]
fn valid_replay_v4() {
    let mut file = File::open("tests/basicv4.replay").unwrap();

    let replay = Replay::from_reader(&mut file).unwrap();

    assert_eq!(replay.header.replay_version, 4);
    assert_eq!(replay.header.protocol_version, 23);
    assert_eq!(replay.header.spyparty_version, 6015);
    assert_eq!(replay.header.flags, 0);
    assert_eq!(replay.header.duration, 155.875);
    assert_eq!(replay.header.game_id, 0x034bef9023a8ae82934b2e4cf8d97854);
    assert_eq!(replay.header.start_time, 1523357331);
    assert_eq!(replay.header.play_id, 2);
    assert_eq!(replay.header.spy_user_len, 11);
    assert_eq!(replay.header.sniper_user_len, 9);
    assert_eq!(replay.header.spy_display_len, 0);
    assert_eq!(replay.header.sniper_display_len, 0);
    assert_eq!(replay.header.latency, 0.75);
    assert_eq!(replay.header.data_size, 125564);
    assert_eq!(replay.header.spy_user_name, "adamintokyo");
    assert_eq!(replay.header.sniper_user_name, "plastikqs");
    assert_eq!(replay.header.spy_display_name, None);
    assert_eq!(replay.header.sniper_display_name, None);
    assert_eq!(replay.header.result_data.version, 1);
    assert_eq!(replay.header.result_data.simple_rules, Some(true));
    assert_eq!(replay.header.result_data.game_result, GameResult::SpyShot);
    assert_eq!(replay.header.result_data.game_mode, GameMode::Known(4));
    assert_eq!(replay.header.result_data.map, Map::Ballroom);
    assert_eq!(
        replay.header.result_data.selected_missions,
        vec![
            Mission::BugAmbassador,
            Mission::ContactDoubleAgent,
            Mission::SwapStatue,
            Mission::SeduceTarget
        ]
    );
    assert_eq!(
        replay.header.result_data.picked_missions,
        vec![
            Mission::BugAmbassador,
            Mission::ContactDoubleAgent,
            Mission::SwapStatue,
            Mission::SeduceTarget
        ]
    );
    assert_eq!(
        replay.header.result_data.completed_missions,
        vec![Mission::BugAmbassador]
    );
    assert_eq!(replay.header.result_data.guests, None);
    assert_eq!(replay.header.result_data.clock_start, None);
}

#[test]
fn valid_replay_v3() {
    let mut file = File::open("tests/basicv3.replay").unwrap();

    let replay = Replay::from_reader(&mut file).unwrap();

    assert_eq!(replay.header.replay_version, 3);
    assert_eq!(replay.header.protocol_version, 20);
    assert_eq!(replay.header.spyparty_version, 5138);
    assert_eq!(replay.header.flags, 0);
    assert_eq!(replay.header.duration, 192.375);
    assert_eq!(replay.header.game_id, 0x6739d1d6709b81a2d44e20785467d360);
    assert_eq!(replay.header.start_time, 1497124602);
    assert_eq!(replay.header.play_id, 2);
    assert_eq!(replay.header.spy_user_len, 13);
    assert_eq!(replay.header.sniper_user_len, 10);
    assert_eq!(replay.header.spy_display_len, 0);
    assert_eq!(replay.header.sniper_display_len, 0);
    assert_eq!(replay.header.latency, 0.75);
    assert_eq!(replay.header.data_size, 132918);
    assert_eq!(replay.header.spy_user_name, "canadianbacon");
    assert_eq!(replay.header.sniper_user_name, "krazycaley");
    assert_eq!(replay.header.spy_display_name, None);
    assert_eq!(replay.header.sniper_display_name, None);
    assert_eq!(replay.header.result_data.version, 0);
    assert_eq!(replay.header.result_data.simple_rules, None);
    assert_eq!(replay.header.result_data.game_result, GameResult::SpyShot);
    assert_eq!(replay.header.result_data.game_mode, GameMode::Any(5, 8));
    assert_eq!(replay.header.result_data.map, Map::Veranda);
    assert_eq!(
        replay.header.result_data.selected_missions,
        vec![
            Mission::BugAmbassador,
            Mission::ContactDoubleAgent,
            Mission::TransferMicrofilm,
            Mission::SwapStatue,
            Mission::InspectStatues,
            Mission::SeduceTarget,
            Mission::PurloinGuestList,
            Mission::FingerprintAmbassador,
        ]
    );
    assert_eq!(
        replay.header.result_data.picked_missions,
        vec![
            Mission::BugAmbassador,
            Mission::ContactDoubleAgent,
            Mission::TransferMicrofilm,
            Mission::SwapStatue,
            Mission::InspectStatues,
            Mission::SeduceTarget,
            Mission::PurloinGuestList,
            Mission::FingerprintAmbassador,
        ]
    );
    assert_eq!(
        replay.header.result_data.completed_missions,
        vec![
            Mission::SwapStatue,
            Mission::InspectStatues,
            Mission::PurloinGuestList
        ]
    );
    assert_eq!(replay.header.result_data.guests, None);
    assert_eq!(replay.header.result_data.clock_start, None);
}

#[test]
fn invalid_replay() {
    let mut file = File::open("tests/broken.replay").unwrap();

    let replay = Replay::from_reader(&mut file);

    assert!(replay.is_err());
}
