extern crate ez_io;

type Result<T> = std::result::Result<T, error::SMDError>;

mod error;
pub mod smd;
pub mod swd;
mod util;
