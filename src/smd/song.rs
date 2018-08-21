use ez_io::{MagicNumberCheck, ReadE};
use std::io::{Read, Seek, SeekFrom};
use Result;

pub struct SongChunk {
    pub ticks_per_quarter_note: u16,
    pub nb_tracks: u8,
    pub nb_channels: u8,
}

impl SongChunk {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<SongChunk> {
        reader.check_magic_number(&[b's', b'o', b'n', b'g'])?;
        reader.seek(SeekFrom::Current(14))?; // Unknown Data
        let ticks_per_quarter_note = reader.read_le_to_u16()?;
        reader.seek(SeekFrom::Current(2))?; // Unknown Data
        let nb_tracks = reader.read_to_u8()?;
        let nb_channels = reader.read_to_u8()?;
        reader.seek(SeekFrom::Current(24))?; // Unknown Data
        while reader.read_to_u8()? == 255 {} // Padding
        reader.seek(SeekFrom::Current(-1))?;
        Ok(SongChunk {
            ticks_per_quarter_note,
            nb_tracks,
            nb_channels,
        })
    }
}
