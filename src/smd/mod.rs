pub mod header;
pub mod song;
pub mod track;

use self::header::HeaderChunk;
use self::song::SongChunk;
use self::track::TrackChunk;
use std::io::{Read, Seek};
use Result;

pub struct SMD {
    pub header: HeaderChunk,
    pub song: SongChunk,
    pub tracks: Vec<TrackChunk>,
}

impl SMD {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<Self> {
        let header = HeaderChunk::import(reader)?;
        let song = SongChunk::import(reader)?;
        let mut tracks = Vec::with_capacity(usize::from(song.nb_tracks));
        while tracks.len() < tracks.capacity() {
            tracks.push(TrackChunk::import(reader)?);
        }
        Ok(SMD {
            header,
            song,
            tracks,
        })
    }
}
