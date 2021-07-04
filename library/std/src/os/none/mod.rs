//! Definitions for bare metal platforms that nevertheless do have a POSIX compatibility layer

// TODO: Figure out if these can be proclaimed as "stable" and if yes, since what version
#![stable(feature = "raw_ext", since = "1.1.0")]

pub mod fs;
pub mod raw;
