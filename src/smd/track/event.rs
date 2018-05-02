use error::EventImport as EventImportError;
use ez_io::ReadE;
use std::io::Result as IoResult;
use std::io::{Read, Seek, SeekFrom};

pub enum Event {
    NotePlay(NotePlay),
    DeltaTime(DeltaTime),
    WaitAgain,
    WaitAdd(i8),
    Wait1Byte(u8),
    Wait2Byte(u16),
    TrackEnd,
    LoopPoint,
    SetOctave(u8),
    SetTempo(u8),
    SetSample(u8),
    SetModulation(u8),
    SetBend(i16),
    SetVolume(u8),
    SetExpression(u8),
    SetPanning(u8),
    UnknownEvent,
    UnknownEventOne(u8),
    UnknownEventTwo([u8; 2]),
    UnknownEventThree([u8; 3]),
    UnknownEventFive([u8; 5]),
}

pub struct NotePlay {
    pub velocity: u8,
    pub octave_change: OctaveChange,
    pub key: Key,
    pub length: Option<u32>,
}

#[derive(Clone)]
pub enum OctaveChange {
    Down2 = 0x0,
    Down1 = 0x10,
    NoChange = 0x20,
    Up1 = 0x30,
}

#[derive(Clone)]
pub enum Key {
    C = 0x00,
    CSharp = 0x01,
    D = 0x02,
    DSharp = 0x03,
    E = 0x04,
    F = 0x05,
    FSharp = 0x06,
    G = 0x07,
    GSharp = 0x08,
    A = 0x09,
    ASharp = 0x0A,
    B = 0x0B,
    CUp = 0x0C,
    CSharpUp = 0x0D,
    DUp = 0x0E,
    DSharpUp = 0x0F,
}

pub struct DeltaTime {
    pub value: u8,
}

impl Event {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<Event, EventImportError> {
        let op_code = reader.read_to_u8()?;
        Ok(match op_code {
            0x00...0x7f => {
                reader.seek(SeekFrom::Current(-1))?;
                Event::NotePlay(NotePlay::import(reader)?)
            }
            0x80...0x8f => {
                reader.seek(SeekFrom::Current(-1))?;
                Event::DeltaTime(DeltaTime::import(reader)?)
            }
            0x90 => Event::WaitAgain,
            0x91 => Event::WaitAdd(reader.read_to_i8()?),
            0x92 => Event::Wait1Byte(reader.read_to_u8()?),
            0x93 => Event::Wait2Byte(reader.read_le_to_u16()?),
            0x98 => Event::TrackEnd,
            0x99 => Event::LoopPoint,
            0xA0 => Event::SetOctave(reader.read_to_u8()?),
            0xA4 => Event::SetTempo(reader.read_to_u8()?),
            0xAC => Event::SetSample(reader.read_to_u8()?),
            0xBE => Event::SetModulation(reader.read_to_u8()?),
            0xD7 => Event::SetBend(reader.read_le_to_i16()?),
            0xE0 => Event::SetVolume(reader.read_to_u8()?),
            0xE3 => Event::SetExpression(reader.read_to_u8()?),
            0xE8 => Event::SetPanning(reader.read_to_u8()?),
            0x9D => Event::UnknownEvent,
            0x9C => Event::UnknownEventOne(reader.read_to_u8()?),
            0xA9 => Event::UnknownEventOne(reader.read_to_u8()?),
            0xAA => Event::UnknownEventOne(reader.read_to_u8()?),
            0xB2 => Event::UnknownEventOne(reader.read_to_u8()?),
            0xB5 => Event::UnknownEventOne(reader.read_to_u8()?),
            0xBF => Event::UnknownEventOne(reader.read_to_u8()?),
            0xD0 => Event::UnknownEventOne(reader.read_to_u8()?),
            0xD1 => Event::UnknownEventOne(reader.read_to_u8()?),
            0xD2 => Event::UnknownEventOne(reader.read_to_u8()?),
            0xDB => Event::UnknownEventOne(reader.read_to_u8()?),
            0xA8 => Event::UnknownEventTwo([reader.read_to_u8()?; 2]),
            0xB4 => Event::UnknownEventTwo([reader.read_to_u8()?; 2]),
            0xD6 => Event::UnknownEventTwo([reader.read_to_u8()?; 2]),
            0xF6 => Event::UnknownEventTwo([reader.read_to_u8()?; 2]),
            0xD4 => Event::UnknownEventThree([reader.read_to_u8()?; 3]),
            0xE2 => Event::UnknownEventThree([reader.read_to_u8()?; 3]),
            0xEA => Event::UnknownEventThree([reader.read_to_u8()?; 3]),
            0xDC => Event::UnknownEventFive([reader.read_to_u8()?; 5]),
            op_code => return Err(EventImportError::UnknownEvent(op_code)),
        })
    }
}

impl NotePlay {
    pub fn import<R: Read>(reader: &mut R) -> IoResult<NotePlay> {
        let velocity = reader.read_to_u8()?;
        let key_flags = reader.read_to_u8()?;
        let octave_change = OctaveChange::get(key_flags);
        let key = Key::get(key_flags);
        let length = match key_flags & 0xC0 {
            0x00 => None,
            0x40 => Some(u32::from(reader.read_to_u8()?)),
            0x80 => {
                Some((u32::from(reader.read_to_u8()?) << 8) | (u32::from(reader.read_to_u8()?)))
            }
            0xC0 => Some(
                (u32::from(reader.read_to_u8()?) << 16) | (u32::from(reader.read_to_u8()?) << 8)
                    | (u32::from(reader.read_to_u8()?)),
            ),
            _ => panic!(), // Unreachable
        };
        Ok(NotePlay {
            velocity,
            octave_change,
            key,
            length,
        })
    }
    /// Calculates the frequency for this note
    pub fn calc_frequency(&self, octave: u8) -> f64 {
        let key_number = (4 + ((i32::from(octave) - 1) * 12)) + self.key.clone() as i32 + match self.octave_change {
            OctaveChange::Down2 => -24,
            OctaveChange::Down1 => -12,
            OctaveChange::NoChange => 0,
            OctaveChange::Up1 => 12
        };
        2f64.powf(f64::from(key_number - 49i32)/12f64) * 440f64
    }
}

impl OctaveChange {
    pub fn get(key_flags: u8) -> OctaveChange {
        match key_flags & 0x30 {
            0x00 => OctaveChange::Down2,
            0x10 => OctaveChange::Down1,
            0x20 => OctaveChange::NoChange,
            0x30 => OctaveChange::Up1,
            _ => panic!(), // Impossible to reach
        }
    }
}

impl Key {
    pub fn get(key_flags: u8) -> Key {
        match key_flags & 0x0F {
            0x00 => Key::C,
            0x01 => Key::CSharp,
            0x02 => Key::D,
            0x03 => Key::DSharp,
            0x04 => Key::E,
            0x05 => Key::F,
            0x06 => Key::FSharp,
            0x07 => Key::G,
            0x08 => Key::GSharp,
            0x09 => Key::A,
            0x0A => Key::ASharp,
            0x0B => Key::B,
            0x0C => Key::CUp,
            0x0D => Key::CSharpUp,
            0x0E => Key::DUp,
            0x0F => Key::DSharpUp,
            _ => panic!(), // Impossible to reach
        }
    }
}

impl DeltaTime {
    pub fn import<R: Read>(reader: &mut R) -> IoResult<DeltaTime> {
        let value = reader.read_to_u8()?;
        Ok(DeltaTime { value })
    }
}
