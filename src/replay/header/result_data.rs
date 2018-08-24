use replay::header::{Error, Result};
use std::io::Read;
use utils;

#[derive(Debug, Default)]
pub struct ResultData {
    pub version: u32,
    pub simple_rules: bool,
}

/// The result data contained in the header of a replay.
impl ResultData {
    /// Create a new header from a reader.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<ResultData> {
        let mut result_data: ResultData = Default::default();

        result_data.set_flags(reader)?;

        // Skip the rest
        if result_data.version == 1 {
            let mut id = [0; 24];
            reader.read_exact(&mut id)?;
        } else {
            let mut id = [0; 32];
            reader.read_exact(&mut id)?;
        }

        Ok(result_data)
    }

    /// Read and set the result data flags.
    ///
    /// These flags include the version and whether the game was played with simple fules. Currently versions 1 and 2 are supported.
    fn set_flags<R: Read>(&mut self, reader: &mut R) -> Result<()> {
        let flags = utils::read_u32(reader)?;

        let version = flags & 0x0f;
        let simple = (flags & 0xf0) == 0x10;

        ensure!(
            version == 1 || version == 2,
            Error::UnsupportedResultVersion(version)
        );

        self.version = version;
        self.simple_rules = simple;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_version_not_simple() {
        let mut input: &[u8] = &[0x01, 00, 00, 00];
        let mut data: ResultData = Default::default();
        data.set_flags(&mut input).unwrap();

        assert_eq!(data.version, 1);
        assert_eq!(data.simple_rules, false);
    }

    #[test]
    fn flags_version_simple() {
        let mut input: &[u8] = &[0x11, 00, 00, 00];
        let mut data: ResultData = Default::default();
        data.set_flags(&mut input).unwrap();

        assert_eq!(data.version, 1);
        assert_eq!(data.simple_rules, true);
    }

    #[test]
    fn unsupported_version() {
        let mut input: &[u8] = &[0x03, 00, 00, 00];
        let mut data: ResultData = Default::default();
        let validated = data.set_flags(&mut input);

        match validated {
            Err(Error::UnsupportedResultVersion(3)) => assert!(true),
            _ => assert!(false),
        }
    }
}
