use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum EventImport {
    IoError(IoError),
    UnknownEvent(u8),
}

impl Error for EventImport {
    fn description(&self) -> &str {
        match *self {
            EventImport::IoError(ref e) => e.description(),
            EventImport::UnknownEvent(_) => "Unknown event Op Code",
        }
    }
}

impl fmt::Display for EventImport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EventImport::IoError(ref e) => e.fmt(f),
            EventImport::UnknownEvent(ref op_code) => write!(f, "Unknown Op Code: 0x{:X}", op_code),
        }
    }
}

impl From<IoError> for EventImport {
    fn from(err: IoError) -> EventImport {
        EventImport::IoError(err)
    }
}
