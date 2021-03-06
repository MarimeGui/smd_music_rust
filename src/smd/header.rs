use ez_io::{MagicNumberCheck, ReadE};
use std::io::{Read, Seek, SeekFrom};
use util::SeekAlign;
use Result;

pub struct HeaderChunk {
    pub instrument_group_id: u8,
    pub creation_date: Date,
    pub name: String,
}

pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub centi_second: u8,
}

impl HeaderChunk {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        reader.check_magic_number(&[b's', b'm', b'd', b'l'])?;
        reader.seek(SeekFrom::Current(4))?; // Null
        reader.seek(SeekFrom::Current(4))?; // File size
        reader.seek(SeekFrom::Current(2))?; // ???
        let instrument_group_id = reader.read_to_u8()?; // Instrument Group ID
        reader.seek(SeekFrom::Current(1))?; // ?
        reader.seek(SeekFrom::Current(8))?; // Null
        let creation_date = Date::import(reader)?;
        let name = {
            let mut buf = vec![0u8; 16];
            reader.read_exact(&mut buf)?;
            let mut keep_buf = Vec::new();
            for byte in buf {
                if byte != 0 {
                    keep_buf.push(byte)
                } else {
                    break;
                }
            }
            String::from_utf8(keep_buf)?
        };
        reader.seek(SeekFrom::Current(8))?;
        reader.align_16()?;
        Ok(HeaderChunk {
            instrument_group_id,
            creation_date,
            name,
        })
    }
}

impl Date {
    pub fn import<R: Read>(reader: &mut R) -> Result<Self> {
        let year = reader.read_le_to_u16()?;
        let month = reader.read_to_u8()?;
        let day = reader.read_to_u8()?;
        let hour = reader.read_to_u8()?;
        let minute = reader.read_to_u8()?;
        let second = reader.read_to_u8()?;
        let centi_second = reader.read_to_u8()?; // Need to add that later
        Ok(Date {
            year,
            month,
            day,
            hour,
            minute,
            second,
            centi_second,
        })
    }
}
