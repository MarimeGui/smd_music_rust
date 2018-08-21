use ez_io::error::{MagicNumberCheckError, WrongMagicNumber};
use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum SMDError {
    IoError(IoError),
    WrongMagicNumber(WrongMagicNumber),
    UnknownEvent(u8),
    UTF8Error(FromUtf8Error),
}

impl Error for SMDError {
    fn description(&self) -> &str {
        match *self {
            SMDError::IoError(ref e) => e.description(),
            SMDError::UnknownEvent(_) => "Unknown event Op Code",
            SMDError::WrongMagicNumber(ref e) => e.description(),
            SMDError::UTF8Error(ref e) => e.description(),
        }
    }
}

impl fmt::Display for SMDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SMDError::IoError(ref e) => e.fmt(f),
            SMDError::UnknownEvent(ref op_code) => write!(f, "Unknown Op Code: 0x{:X}", op_code),
            SMDError::WrongMagicNumber(ref e) => e.fmt(f),
            SMDError::UTF8Error(ref e) => e.fmt(f),
        }
    }
}

impl From<IoError> for SMDError {
    fn from(err: IoError) -> SMDError {
        SMDError::IoError(err)
    }
}

impl From<MagicNumberCheckError> for SMDError {
    fn from(err: MagicNumberCheckError) -> SMDError {
        match err {
            MagicNumberCheckError::IoError(io) => SMDError::IoError(io),
            MagicNumberCheckError::MagicNumber(mn) => SMDError::WrongMagicNumber(mn),
        }
    }
}

impl From<FromUtf8Error> for SMDError {
    fn from(err: FromUtf8Error) -> SMDError {
        SMDError::UTF8Error(err)
    }
}
