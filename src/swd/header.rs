use std::io::{Read, Seek};
use Result;

pub struct Header {}

impl Header {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<Header> {
        Ok(Header {})
    }
}
