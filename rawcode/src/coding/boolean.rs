//! Implements boolean coding

use crate::{
    coding::{RawcodeConstSize, RawcodeDecode, RawcodeEncode},
    error::Error,
};

impl RawcodeConstSize for bool {
    const SIZE: usize = u8::SIZE;
}
impl RawcodeDecode for bool {
    fn decode(buf: &[u8]) -> Result<Self, Error> {
        let value = u8::decode(buf)?;
        match value {
            0x00 => Ok(false),
            0xff => Ok(true),
            _ => Err(e!("Invalid boolean")),
        }
    }
}
impl RawcodeEncode for bool {
    fn encode(&self, buf: &mut [u8]) -> Result<(), Error> {
        let value = match self {
            false => 0x00u8,
            true => 0xffu8,
        };
        value.encode(buf)
    }
}
