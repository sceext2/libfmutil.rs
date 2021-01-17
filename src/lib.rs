#![no_std]
#![deny(unsafe_code)]

#[cfg(feature = "libfmhal")]
pub mod hal;

#[cfg(feature = "pid")]
pub mod pid;

#[cfg(test)]
mod test;
