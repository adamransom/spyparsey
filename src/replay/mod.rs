mod header;

pub use self::header::GameResult;
pub use self::header::Header;

use self::header::Result;
use std::io::Read;

pub struct Replay {
    /// The header of the replay.
    ///
    /// This contains all the information describing the game.
    pub header: Header,
}

impl Replay {
    /// Create a new replay from a reader.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Replay> {
        let header = Header::from_reader(reader)?;

        Ok(Replay { header })
    }
}
