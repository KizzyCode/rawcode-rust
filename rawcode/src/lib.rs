#![doc = include_str!("../../README.md")]
#![cfg_attr(feature = "no_std", no_std)]

#[macro_use]
pub mod error;
pub mod coding;

pub use coding::*;
