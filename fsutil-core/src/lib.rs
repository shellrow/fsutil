// -- crates
#[macro_use]
extern crate log;

// -- export
pub use fs::{File, RemoteError, RemoteErrorType, RemoteFileSystem, RemoteResult};
// -- modules
pub mod fs;

// -- utils
pub(crate) mod utils;
// -- mock
#[cfg(test)]
pub(crate) mod mock;
