//! Defines the basic encoding traits

mod array;
mod boolean;
mod integer;
mod strarray;
mod unit;

pub use crate::coding::{strarray::StrArray, unit::Unit};
use crate::error::Error;

/// A rawcode type with a const size
pub trait RawcodeConstSize {
    /// The encoded size of `Self`
    const SIZE: usize;

    /// The encoded size of `Self`
    fn size() -> usize {
        Self::SIZE
    }
}

/// A rawcode encodable type
pub trait RawcodeEncode
where
    Self: RawcodeConstSize,
{
    /// Encodes `self` into `buf`
    fn encode(&self, buf: &mut [u8]) -> Result<(), Error>;
}

/// A rawcode decode type
pub trait RawcodeDecode
where
    Self: RawcodeConstSize + Sized,
{
    /// Decodes `Self` from `buf`
    fn decode(buf: &[u8]) -> Result<Self, Error>;
}

/// Encodes `value` to `buf`
pub fn to_slice<T>(value: &T, buf: &mut [u8]) -> Result<(), Error>
where
    T: RawcodeEncode,
{
    value.encode(buf)
}
/// Encodes `value` to `buf` at the given `pos` and increments the position accordingly
pub fn to_slice_at<T>(value: &T, buf: &mut [u8], pos: &mut usize) -> Result<(), Error>
where
    T: RawcodeEncode,
{
    // Create the buffer and encode the value
    let buf = buf.get_mut(*pos..).and_then(|buf| buf.get_mut(..T::SIZE)).ok_or(e!("Truncated buffer"))?;
    value.encode(buf)?;

    // Advance the position
    *pos += T::SIZE;
    Ok(())
}
/// Encodes `value`
#[cfg(feature = "std")]
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: RawcodeEncode,
{
    let mut buf = vec![0; T::SIZE];
    value.encode(&mut buf)?;
    Ok(buf)
}

/// Decodes `value` from `buf`
pub fn from_slice<T>(buf: &[u8]) -> Result<T, Error>
where
    T: RawcodeDecode,
{
    T::decode(buf)
}
/// Decodes `value` from `buf` at the given `pos` and increments the position accordingly
pub fn from_slice_at<T>(buf: &[u8], pos: &mut usize) -> Result<T, Error>
where
    T: RawcodeDecode,
{
    // Create the buffer and encode the value
    let buf = buf.get(*pos..).and_then(|buf| buf.get(..T::SIZE)).ok_or(e!("Truncated data"))?;
    let value = T::decode(buf)?;

    // Advance the position
    *pos += T::SIZE;
    Ok(value)
}
