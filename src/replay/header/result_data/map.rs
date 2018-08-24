use std::fmt;

/// The maps of SpyParty.
#[derive(Debug, PartialEq)]
pub enum Map {
    Balcony,
    Ballroom,
    Courtyard,
    Gallery,
    HighRise,
    Library,
    Moderne,
    Pub,
    Terrace,
    Veranda,
    Unknown(u32),
}

impl Default for Map {
    fn default() -> Map {
        Map::Unknown(0)
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Map::Balcony => "Balcony",
                Map::Ballroom => "Ballroom",
                Map::Courtyard => "Courtyard",
                Map::Gallery => "Gallery",
                Map::HighRise => "High-Rise",
                Map::Library => "Library",
                Map::Moderne => "Moderne",
                Map::Pub => "Pub",
                Map::Terrace => "Terrace",
                Map::Veranda => "Veranda",
                Map::Unknown(_) => "Unknown",
            }
        )
    }
}

impl From<u32> for Map {
    fn from(map: u32) -> Self {
        match map {
            0x1dbd8e41 => Map::Balcony,
            0x5b121925 => Map::Ballroom,
            0x9dc5bb5e => Map::Courtyard,
            0x7173b8bf => Map::Gallery,
            0x1a56c5a1 => Map::HighRise,
            0x168f4f62 => Map::Library,
            0x2e37f15b => Map::Moderne,
            0x3b85fff3 => Map::Pub,
            0x9032ce22 => Map::Terrace,
            0x6f81a558 => Map::Veranda,
            _ => Map::Unknown(map),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn known_map() {
        let result: Map = 0x1dbd8e41.try_into().unwrap();
        assert_eq!(result, Map::Balcony);
    }

    #[test]
    fn unknown_map() {
        let result: Map = 1u32.try_into().unwrap();
        assert_eq!(result, Map::Unknown(1));
    }
}
