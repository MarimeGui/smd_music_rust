use std::io::{Read, Seek};
use Result;

pub struct WAVI {}

impl WAVI {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<WAVI> {
        Ok(WAVI {})
    }
}
