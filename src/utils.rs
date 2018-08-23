use byteorder::{ByteOrder, LittleEndian};
use std::io::{Read, Result};

/// Read u32 values from a series of bytes.
///
/// All the numeric values in the replays are little endian, so that is what we use here.
pub fn read_u32<R: Read>(reader: &mut R) -> Result<u32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;

    let val = LittleEndian::read_u32(&buf);

    Ok(val)
}
