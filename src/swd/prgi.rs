use std::io::{Read, Seek};
use Result;

pub struct PRGI {}

impl PRGI {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<PRGI> {
        Ok(PRGI {})
    }
}
