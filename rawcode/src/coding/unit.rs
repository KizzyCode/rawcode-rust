//! Implements encoding for `()` and `PhantomData<T>`

use crate::{
    coding::{RawcodeConstSize, RawcodeDecode, RawcodeEncode},
    error::Error,
};
use core::marker::PhantomData;

/// Provide an `ident`-compatible alias for the `()`-type
pub type Unit = ();

/// Implements the rawcode traits for zero-sized types
macro_rules! impl_zero_sized {
    ($type:ident $(<$($generics:tt),*>)?) => {
        impl $(<$($generics),*>)? RawcodeConstSize for $type $(<$($generics),*>)? {
            const SIZE: usize = 0;
        }
        impl $(<$($generics),*>)? RawcodeDecode for $type $(<$($generics),*>)? {
            fn decode(buf: &[u8]) -> Result<Self, Error> {
                match buf.len() {
                    Self::SIZE => Ok(Default::default()),
                    _ => Err(e!("Encoded data is too long")),
                }
            }
        }
        impl $(<$($generics),*>)? RawcodeEncode for $type $(<$($generics),*>)? {
            fn encode(&self, buf: &mut [u8]) -> Result<(), Error> {
                match buf.len() {
                    Self::SIZE => Ok(()),
                    _ => Err(e!("Target buffer is too large")),
                }
            }
        }
    };
}
impl_zero_sized!(Unit);
impl_zero_sized!(PhantomData<T>);
