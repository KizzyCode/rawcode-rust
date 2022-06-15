//! Implements integer coding

use crate::{
    coding::{RawcodeConstSize, RawcodeDecode, RawcodeEncode},
    error::Error,
};
use core::mem;

// Specialized byte encoding
impl RawcodeConstSize for u8 {
    const SIZE: usize = 1;
}
impl RawcodeDecode for u8 {
    fn decode(buf: &[u8]) -> Result<Self, Error> {
        // Validate the input length
        match buf.len() {
            len if len > Self::SIZE => Err(e!("Encoded data is too long")),
            len if len < Self::SIZE => Err(e!("Encoded data is too short")),
            _ => Ok(buf[0]),
        }
    }
}
impl RawcodeEncode for u8 {
    fn encode(&self, buf: &mut [u8]) -> Result<(), Error> {
        // Validate the buffer length
        match buf.len() {
            len if len > Self::SIZE => return Err(e!("Target buffer is too large")),
            len if len < Self::SIZE => return Err(e!("Target buffer is too small")),
            _ => buf[0] = *self,
        }
        Ok(())
    }
}

macro_rules! impl_int {
    ($type:ty) => {
        impl RawcodeConstSize for $type {
            const SIZE: usize = mem::size_of::<Self>();
        }
        impl RawcodeDecode for $type {
            fn decode(buf: &[u8]) -> Result<Self, Error> {
                // Decode the bytes
                let bytes = <[u8; mem::size_of::<Self>()]>::decode(buf)?;
                let value = Self::from_le_bytes(bytes);
                Ok(value)
            }
        }
        impl RawcodeEncode for $type {
            fn encode(&self, buf: &mut [u8]) -> Result<(), Error> {
                let bytes = self.to_le_bytes();
                bytes.encode(buf)
            }
        }
    };
}
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(u128);
impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(i128);
