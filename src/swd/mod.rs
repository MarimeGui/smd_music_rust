pub mod header;
pub mod kgrp;
pub mod pcmd;
pub mod prgi;
pub mod wavi;

// use self::header::Header;
// use self::kgrp::KGRP;
// use self::pcmd::PCMD;
// use self::prgi::PRGI;
// use self::wavi::WAVI;
use std::io::{Read, Seek};
use Result;

// pub struct SWD {
//     pub header: Header,
//     pub wavi: WAVI,
//     pub prgi: PRGI,
//     pub kgrp: KGRP,
//     pub pcmd: PCMD,
// }

pub struct SWD {}

impl SWD {
    pub fn import<R: Read + Seek>(reader: &mut R) -> Result<SWD> {
        Ok(SWD {})
    }
}
