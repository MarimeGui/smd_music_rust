use chrono::prelude::*;
use ez_io::ReadE;
use magic_number::check_magic_number;
use std::error::Error;
use std::io::{Read, Seek, SeekFrom};
use util::SeekAlign;

pub struct HeaderChunk {
    pub export_date: DateTime<Utc>,
    pub name: String,
}

impl HeaderChunk {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<Self, Box<Error>> {
        check_magic_number(reader, vec![b's', b'm', b'd', b'l'])?;
        reader.seek(SeekFrom::Current(20))?;
        let year = i32::from(reader.read_le_to_u16()?);
        let month = u32::from(reader.read_to_u8()?);
        let day = u32::from(reader.read_to_u8()?);
        let hour = u32::from(reader.read_to_u8()?);
        let minute = u32::from(reader.read_to_u8()?);
        let second = u32::from(reader.read_to_u8()?);
        let _csecond = reader.read_to_u8()?; // Need to add that later
        let export_date = Utc.ymd(year, month, day).and_hms(hour, minute, second);
        let name = reader.read_to_string_n(16)?; // Might not work, has 0s
        reader.seek(SeekFrom::Current(8))?;
        reader.align_16()?;
        Ok(HeaderChunk { export_date, name })
    }
}
