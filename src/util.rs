use std::io::{Result, Seek, SeekFrom};

pub trait SeekAlign: Seek {
    fn align_16(&mut self) -> Result<()> {
        let position = self.seek(SeekFrom::Current(0))? as i64;
        if 16 - (position % 16) != 0 {
            self.seek(SeekFrom::Current(16 - (position % 16)))?;
        }
        Ok(())
    }
}

impl<S: Seek> SeekAlign for S {}
