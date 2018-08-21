pub mod event;

use self::event::Event;
use ez_io::{MagicNumberCheck, ReadE};
use std::io::{Read, Seek, SeekFrom};
use Result;

pub struct TrackChunk {
    pub track_id: u8,
    pub channel_id: u8,
    pub events: Vec<Event>,
}

impl TrackChunk {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<TrackChunk> {
        reader.check_magic_number(&[b't', b'r', b'k', b' '])?;
        reader.seek(SeekFrom::Current(8))?;
        let chunk_end = u64::from(reader.read_le_to_u32()?) + reader.seek(SeekFrom::Current(0))?;
        let track_id = reader.read_to_u8()?;
        let channel_id = reader.read_to_u8()?;
        reader.seek(SeekFrom::Current(2))?;
        let mut events = Vec::new();
        while reader.seek(SeekFrom::Current(0))? < chunk_end {
            events.push(Event::import(reader)?)
        }
        while reader.read_to_u8()? == 0x98 {}
        reader.seek(SeekFrom::Current(-1))?;
        Ok(TrackChunk {
            track_id,
            channel_id,
            events,
        })
    }
}
