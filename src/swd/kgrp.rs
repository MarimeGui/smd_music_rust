use std::io::{Read, Seek};
use Result;

pub struct KGRP {}

impl KGRP {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<KGRP> {
        Ok(KGRP {})
    }
}
