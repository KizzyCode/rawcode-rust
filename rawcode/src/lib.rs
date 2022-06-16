#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "no_std", no_std)]

#[macro_use]
pub mod error;
pub mod coding;

// Re-export coding traits and types
pub use coding::*;

// Re-export #[derive(Rawcode)]
#[cfg(feature = "rawcode_derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate rawcode_derive;
#[cfg(feature = "rawcode_derive")]
#[doc(hidden)]
pub use rawcode_derive::*;
