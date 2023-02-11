//! Implements the encoding of list types

use crate::{
    coding::{RawcodeConstSize, RawcodeDecode, RawcodeEncode},
    error::Error,
};
use core::{
    fmt::{self, Debug, Formatter},
    ops::Deref,
    str,
};

/// An array that always guarantees that it's contents are valid UTF-8
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Utf8Array<const LEN: usize> {
    /// An UTF-8 byte array
    utf8_bytes: [u8; LEN],
}
impl<const LEN: usize> Utf8Array<LEN> {
    /// Tries to create a new `Utf8Array` from a string slice
    pub fn from_str(string: &str) -> Option<Self> {
        // Soundness is implicitly guaranteed because `&str` is always valid UTF-8
        let bytes = <[u8; LEN]>::try_from(string.as_bytes()).ok()?;
        Some(Self { utf8_bytes: bytes })
    }
    // Creates a new `Utf8Array` from a byte array
    pub fn from_array(array: [u8; LEN]) -> Option<Self> {
        // Soundness is guaranteed because because `str::from_utf8` validates the UTF-8 encoding
        match str::from_utf8(&array) {
            Ok(_) => Some(Self { utf8_bytes: array }),
            Err(_) => None,
        }
    }
}
impl<const LEN: usize> Utf8Array<LEN> {
    /// The underlying bytes as array
    pub fn as_array(&self) -> &[u8; LEN] {
        &self.utf8_bytes
    }
    /// The underlying bytes as string slice
    pub fn as_str(&self) -> &str {
        // This is safe because we've already validated the string during construction
        unsafe { str::from_utf8_unchecked(&self.utf8_bytes) }
    }
    /// Destructures `self` into the underlying array
    pub fn into_array(self) -> [u8; LEN] {
        self.utf8_bytes
    }
}
impl<const LEN: usize> Debug for Utf8Array<LEN> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Utf8Array").field("utf8_bytes", &self.as_str()).finish()
    }
}

/// A string array
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrArray<const LEN: usize> {
    /// The underlying bytes
    utf8_bytes: Utf8Array<LEN>,
}
impl<const LEN: usize> StrArray<LEN> {
    /// Creates a new `StrArray`
    ///
    /// # Panic
    /// Panics if the literal is invalid UTF-8
    pub fn new(bytes: &[u8; LEN]) -> Self {
        match Utf8Array::from_array(*bytes) {
            Some(utf8_bytes) => Self { utf8_bytes },
            None => panic!("Invalid string literal"),
        }
    }
}
impl<const LEN: usize> TryFrom<&str> for StrArray<LEN> {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let utf8_bytes = Utf8Array::from_str(value).ok_or(e!("Invalid string length"))?;
        Ok(Self { utf8_bytes })
    }
}
#[cfg(feature = "std")]
impl<const LEN: usize> TryFrom<String> for StrArray<LEN> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
impl<const LEN: usize> TryFrom<[u8; LEN]> for StrArray<LEN> {
    type Error = Error;

    fn try_from(bytes: [u8; LEN]) -> Result<Self, Self::Error> {
        let utf8_bytes = Utf8Array::from_array(bytes).ok_or(e!("Array contains non-UTF-8 bytes"))?;
        Ok(Self { utf8_bytes })
    }
}
impl<const LEN: usize> TryFrom<&[u8]> for StrArray<LEN> {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let bytes = <[u8; LEN]>::try_from(value).map_err(|_| e!("Invalid slice length"))?;
        Self::try_from(bytes)
    }
}
#[cfg(feature = "std")]
impl<const LEN: usize> TryFrom<Vec<u8>> for StrArray<LEN> {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_slice())
    }
}
impl<const LEN: usize> Deref for StrArray<LEN> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.utf8_bytes.as_str()
    }
}
impl<const LEN: usize> AsRef<str> for StrArray<LEN> {
    fn as_ref(&self) -> &str {
        self.utf8_bytes.as_str()
    }
}
impl<const LEN: usize> AsRef<[u8]> for StrArray<LEN> {
    fn as_ref(&self) -> &[u8] {
        self.utf8_bytes.as_array()
    }
}
impl<const LEN: usize> AsRef<[u8; LEN]> for StrArray<LEN> {
    fn as_ref(&self) -> &[u8; LEN] {
        self.utf8_bytes.as_array()
    }
}
#[cfg(feature = "std")]
impl<const LEN: usize> From<StrArray<LEN>> for String {
    fn from(array: StrArray<LEN>) -> Self {
        array.utf8_bytes.as_str().to_string()
    }
}
impl<const LEN: usize> From<StrArray<LEN>> for [u8; LEN] {
    fn from(array: StrArray<LEN>) -> Self {
        array.utf8_bytes.into_array()
    }
}
#[cfg(feature = "std")]
impl<const LEN: usize> From<StrArray<LEN>> for Vec<u8> {
    fn from(array: StrArray<LEN>) -> Self {
        array.utf8_bytes.as_array().to_vec()
    }
}
impl<const LEN: usize> RawcodeConstSize for StrArray<LEN> {
    const SIZE: usize = <[u8; LEN]>::SIZE;
}
impl<const LEN: usize> RawcodeEncode for StrArray<LEN> {
    fn encode(&self, buf: &mut [u8]) -> Result<(), Error> {
        self.utf8_bytes.as_array().encode(buf)
    }
}
impl<const LEN: usize> RawcodeDecode for StrArray<LEN> {
    fn decode(buf: &[u8]) -> Result<Self, Error> {
        let bytes = <[u8; LEN]>::decode(buf)?;
        Self::try_from(bytes)
    }
}
