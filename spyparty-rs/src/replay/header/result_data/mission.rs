use replay::header::{Error, Result};
use std::convert::TryFrom;
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

impl Mission {
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

    /// Return the shorthand for a mission.
    pub fn short_display(&self) -> &str {
        match self {
            Mission::BugAmbassador => "Bug",
            Mission::ContactDoubleAgent => "BB",
            Mission::FingerprintAmbassador => "Fingerprint",
            Mission::InspectStatues => "Inspect",
            Mission::PurloinGuestList => "Purloin",
            Mission::SeduceTarget => "Seduce",
            Mission::SwapStatue => "Swap",
            Mission::TransferMicrofilm => "Transfer MF",
        }
    }
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

impl<'a> TryFrom<&'a str> for Mission {
    type Error = Error;

    fn try_from(string: &'a str) -> Result<Self> {
        let stripped = string.to_ascii_lowercase().replace(" ", "");

        Ok(match stripped.as_ref() {
            "bugambassador" | "bug" => Mission::BugAmbassador,
            "contactdoubleagent" | "contactda" | "contact" | "bb" => Mission::ContactDoubleAgent,
            "fingerprintambassador" | "fingerprint" | "fp" => Mission::FingerprintAmbassador,
            "inspectstatues" | "inspect" => Mission::InspectStatues,
            "purloinguestlist" | "purloin" => Mission::PurloinGuestList,
            "seducetarget" | "seduce" => Mission::SeduceTarget,
            "swapstatue" | "swap" | "paws" => Mission::SwapStatue,
            "transfermicrofilm" | "transfermf" | "mf" => Mission::TransferMicrofilm,
            _ => bail!(Error::UnknownMission(string.to_string())),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn string_into_mission() {
        let result: Mission = "seduce".try_into().unwrap();
        assert_eq!(result, Mission::SeduceTarget);
    }

    #[test]
    fn string_into_invalid_mission() {
        let validated: Result<Mission> = "nope".try_into();

        match validated {
            Err(Error::UnknownMission(mission)) => assert!(mission == "nope"),
            _ => assert!(false),
        }
    }
}
