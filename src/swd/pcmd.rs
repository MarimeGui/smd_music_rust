use std::io::{Read, Seek};
use Result;

pub struct PCMD {}

impl PCMD {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<PCMD> {
        Ok(PCMD {})
    }
}
