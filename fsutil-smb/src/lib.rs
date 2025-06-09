// -- crates
#[macro_use]
extern crate tracing;

mod client;

#[cfg(target_family = "unix")]
pub use client::{SmbCredentials, SmbEncryptionLevel, SmbFileSystem, SmbOptions, SmbShareMode};
#[cfg(target_family = "windows")]
pub use client::{SmbCredentials, SmbFileSystem};

// -- utils
#[cfg(target_family = "unix")]
pub(crate) mod utils;
// -- mock
#[cfg(test)]
pub(crate) mod mock;
