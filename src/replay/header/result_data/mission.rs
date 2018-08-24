use std::fmt;

/// The missions the spy tries to complete.
#[derive(Debug, PartialEq)]
pub enum Mission {
    BugAmbassador,
    ContactDoubleAgent,
    FingerprintAmbassador,
    InspectStatues,
    PurloinGuestList,
    SeduceTarget,
    SwapStatue,
    TransferMicrofilm,
}

impl fmt::Display for Mission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mission::BugAmbassador => "Bug Ambassador",
                Mission::ContactDoubleAgent => "Contact Double Agent",
                Mission::FingerprintAmbassador => "Fingerprint Ambassador",
                Mission::InspectStatues => "Inspect Statues",
                Mission::PurloinGuestList => "Purloin Guest List",
                Mission::SeduceTarget => "Seduce Target",
                Mission::SwapStatue => "Swap Statue",
                Mission::TransferMicrofilm => "Transfer Microfilm",
            }
        )
    }
}

/// Unpacks a bitfield of missions into a vector.
pub fn unpack_missions(data: u32) -> Vec<Mission> {
    let mut missions: Vec<Mission> = Vec::new();

    if data & (1 << 0) != 0 {
        missions.push(Mission::BugAmbassador);
    }

    if data & (1 << 1) != 0 {
        missions.push(Mission::ContactDoubleAgent);
    }

    if data & (1 << 2) != 0 {
        missions.push(Mission::TransferMicrofilm);
    }

    if data & (1 << 3) != 0 {
        missions.push(Mission::SwapStatue);
    }

    if data & (1 << 4) != 0 {
        missions.push(Mission::InspectStatues);
    }

    if data & (1 << 5) != 0 {
        missions.push(Mission::SeduceTarget);
    }

    if data & (1 << 6) != 0 {
        missions.push(Mission::PurloinGuestList);
    }

    if data & (1 << 7) != 0 {
        missions.push(Mission::FingerprintAmbassador);
    }

    missions
}
